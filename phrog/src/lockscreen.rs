use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Lockscreen(ObjectSubclass<imp::Lockscreen>)
        @extends phosh_dm::Lockscreen, gtk::Widget;
}

impl Lockscreen {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Lockscreen {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {
    use crate::session_object::SessionObject;
    use crate::user_session_page::UserSessionPage;
    use async_channel::{Receiver, Sender};
    use greetd_ipc::codec::SyncCodec;
    use greetd_ipc::{ErrorType, Request, Response};
    use gtk::glib::{clone, closure_local, g_critical, g_warning, ObjectExt};
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
        greetd_sender: OnceCell<Sender<Request>>,
        greetd_receiver: OnceCell<Receiver<Response>>,
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
                if let Err(err) = req.write_to(&mut sock) {
                    g_critical!("greetd", "error sending request: {}", err);
                    continue;
                }
                match Response::read_from(&mut sock) {
                    Err(err) => {
                        g_critical!("greetd", "error receiving response: {}", err);
                        continue;
                    }
                    Ok(resp) => {
                        if let Err(err) = greetd_resp_send.send_blocking(resp) {
                            g_critical!("greetd", "error sending response on channel: {}", err);
                            continue;
                        }
                    }
                }
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

            self.greetd_sender.set(greetd_sender.clone()).unwrap();
            self.greetd_receiver.set(greetd_receiver.clone()).unwrap();

            self.user_session_page.get().unwrap()
                .connect_closure("login", false, closure_local!(@weak-allow-none self as this => move |_: UserSessionPage, username: String, session: SessionObject| {
                    let this = if let Some(this) = this { this } else {
                        return;
                    };

                    this.obj().upcast_ref::<Widget>().set_sensitive(false);

                    glib::spawn_future_local(clone!(@strong greetd_sender, @strong greetd_receiver => async move {
                        greetd_sender.send(Request::CreateSession { username }).await.unwrap();
                        this.obj().upcast_ref::<Widget>().set_sensitive(true);

                        match greetd_receiver.recv().await {
                            Ok(resp) => this.on_greetd_resp(resp),
                            Err(err) => {
                                // TODO: visual thing here (shake the user list view? pop a modal?)
                                panic!("failed to create greetd session: {}", err);
                            }
                        }
                    }));
            }));
        }
    }

    impl Lockscreen {
        fn on_greetd_resp(&self, resp: Response) {
            match resp {
                Response::AuthMessage { auth_message, auth_message_type } => {
                    if auth_message == "Password:" {
                        self.obj().lbl_unlock_status().unwrap().set_label("Enter Passcode");
                    } else {
                        self.obj().lbl_unlock_status().unwrap().set_label(&auth_message);
                    }
                    self.obj().entry_pin().unwrap().delete_text(0, -1);
                    self.obj().upcast_ref::<phosh_dm::Lockscreen>().set_page(LockscreenPage::Unlock);
                }
                Response::Error { error_type, description } => {
                    if let ErrorType::AuthError = error_type {
                        self.obj().upcast_ref::<phosh_dm::Lockscreen>().shake_label();
                        self.obj().lbl_unlock_status().unwrap().set_label(&description);
                    }
                }
                r => {
                    panic!("unhandled greetd response: {:?}", r);
                }
            }
        }
    }

    impl LayerSurfaceImpl for Lockscreen {}
    impl WidgetImpl for Lockscreen {}
    impl LockscreenImpl for Lockscreen {
        fn unlock_submit_cb(&self) {
            let greetd_sender = self.greetd_sender.get().unwrap().clone();
            let greetd_receiver = self.greetd_receiver.get().unwrap().clone();
            glib::spawn_future_local(clone!(@strong greetd_sender, @strong greetd_receiver, @weak self as this => async move {
                this.obj().upcast_ref::<Widget>().set_sensitive(false);
                this.obj().lbl_unlock_status().unwrap().set_label("Checking...");

                greetd_sender.send(Request::PostAuthMessageResponse {
                    response: Some(this.obj().entry_pin().unwrap().text().to_string())
                }).await.unwrap();

                match greetd_receiver.recv().await {
                    Ok(resp) => this.on_greetd_resp(resp),
                    Err(err) => {
                        panic!("failed to create greetd session: {}", err);
                    }
                }

                this.obj().upcast_ref::<Widget>().set_sensitive(true);
            }));
        }
    }
}
