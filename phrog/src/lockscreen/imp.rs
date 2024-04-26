use crate::session_object::SessionObject;
use crate::user_session_page::UserSessionPage;
use async_channel::{Receiver, Sender};
use greetd_ipc::codec::SyncCodec;
use greetd_ipc::{Request, Response};
use gtk::glib::{clone, closure_local, ObjectExt};
use gtk::prelude::*;
use gtk::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
use gtk::subclass::widget::WidgetImpl;
use gtk::{gio, glib, Widget};
use phosh_dm::subclass::layer_surface::LayerSurfaceImpl;
use phosh_dm::subclass::lockscreen::LockscreenImpl;
use phosh_dm::{LockscreenExt, LockscreenPage};
use std::cell::{OnceCell, RefCell};
use std::os::unix::net::UnixStream;

#[derive(Default)]
pub struct Lockscreen {
    user_session_page: OnceCell<UserSessionPage>,
}

#[glib::object_subclass]
impl ObjectSubclass for Lockscreen {
    const NAME: &'static str = "PhoshDMLockscreen";
    type Type = super::Lockscreen;
    type ParentType = phosh_dm::Lockscreen;
}

fn run_greetd() -> (Sender<Request>, Receiver<Response>) {
    let (greetd_req_send, greetd_req_recv) = async_channel::bounded::<Request>(1);
    let (greetd_resp_send, greetd_resp_recv) = async_channel::bounded(1);

    gio::spawn_blocking(move || {
        let mut sock = UnixStream::connect(std::env::var("GREETD_SOCK").unwrap()).unwrap();

        while let Ok(req) = greetd_req_recv.recv_blocking() {
            req.write_to(&mut sock).unwrap();
            greetd_resp_send
                .send_blocking(Response::read_from(&mut sock).unwrap())
                .unwrap();
        }
    });

    (greetd_req_send, greetd_resp_recv)
}

impl ObjectImpl for Lockscreen {
    fn constructed(&self) {
        self.parent_constructed();

        self.user_session_page.set(UserSessionPage::new()).unwrap();

        if let Some(c) = self.obj().carousel() {
            // Remove the first page from the default lockscreen (info widget)
            c.remove(c.children().first().unwrap());

            // Replace it with the user+session selection page.
            c.prepend(self.user_session_page.get().unwrap())
        }

        let (greetd_sender, greetd_receiver) = run_greetd();

        self.user_session_page.get().unwrap()
            .connect_closure("login", false, closure_local!(@weak-allow-none self as this => move |_: UserSessionPage, username: String, session: SessionObject| {
                let this = if let Some(this) = this { this } else {
                    return;
                };

                this.obj().upcast_ref::<Widget>().set_sensitive(false);

                glib::spawn_future_local(clone!(@strong greetd_sender, @strong greetd_receiver => async move {
                    greetd_sender.send(Request::CreateSession { username }).await.unwrap();
                    let resp = greetd_receiver.recv().await.unwrap();

                    this.obj().upcast_ref::<Widget>().set_sensitive(true);
                    this.obj().upcast_ref::<phosh_dm::Lockscreen>().set_page(LockscreenPage::Unlock);
                }));
            }));
    }
}

impl LayerSurfaceImpl for Lockscreen {}

impl WidgetImpl for Lockscreen {}

impl LockscreenImpl for Lockscreen {}
