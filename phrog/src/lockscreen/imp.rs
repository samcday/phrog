use std::cell::{OnceCell, RefCell};
use std::ops::DerefMut;
use std::os::unix::net::UnixStream;
use greetd_ipc::codec::SyncCodec;
use greetd_ipc::Request;
use gtk::{glib, Widget};
use gtk::glib::{closure_local, ObjectExt};
use gtk::prelude::*;
use gtk::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
use gtk::subclass::widget::WidgetImpl;
use phosh_dm::LockscreenExt;
use phosh_dm::subclass::layer_surface::LayerSurfaceImpl;
use phosh_dm::subclass::lockscreen::LockscreenImpl;
use crate::session_object::SessionObject;
use crate::user_session_page::UserSessionPage;

#[derive(Default)]
pub struct Lockscreen {
    user_session_page: OnceCell<UserSessionPage>,
    greetd_socket: OnceCell<RefCell<UnixStream>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Lockscreen {
    const NAME: &'static str = "PhoshDMLockscreen";
    type Type = super::Lockscreen;
    type ParentType = phosh_dm::Lockscreen;
}

impl ObjectImpl for Lockscreen {
    fn constructed(&self) {
        self.parent_constructed();

        self.greetd_socket.set(RefCell::new(UnixStream::connect(std::env::var("GREETD_SOCK").unwrap()).unwrap())).unwrap();
        self.user_session_page.set(UserSessionPage::new()).unwrap();

        if let Some(c) = self.obj().carousel() {
            // Remove the first page from the default lockscreen (info widget)
            c.remove(c.children().first().unwrap());

            // Replace it with the user+session selection page.
            c.prepend(self.user_session_page.get().unwrap())
        }

        self.user_session_page.get().unwrap()
            .connect_closure("login", false, closure_local!(@weak-allow-none self as this => move |_: UserSessionPage, username: String, session: SessionObject| {
                let this = if let Some(this) = this { this } else {
                    return;
                };

                this.obj().upcast_ref::<Widget>().set_sensitive(false);

                glib::spawn_future_local(async move {
                    // TODO: error handling
                    Request::CreateSession { username }.write_to(this.greetd_socket.get().unwrap().borrow_mut().deref_mut()).unwrap();
                });
            }));
    }
}

impl LayerSurfaceImpl for Lockscreen {
}

impl WidgetImpl for Lockscreen {}

impl LockscreenImpl for Lockscreen {
}
