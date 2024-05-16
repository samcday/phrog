use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Lockscreen(ObjectSubclass<imp::Lockscreen>)
        @extends libphosh::Lockscreen, gtk::Widget;
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
    use crate::user_session_page::UserSessionPage;
    use async_channel::{Receiver, Sender};
    use greetd_ipc::codec::SyncCodec;
    use greetd_ipc::{AuthMessageType, ErrorType, Request, Response};
    use gtk::glib::{Cast, clone, closure_local, g_critical, g_warning, ObjectExt};
    use gtk::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
    use gtk::subclass::widget::WidgetImpl;
    use gtk::{gio, glib, Widget};
    use libphosh::subclass::lockscreen::LockscreenImpl;
    use libphosh::prelude::*;
    use std::cell::{Cell, OnceCell, RefCell};
    use std::os::unix::net::UnixStream;
    use std::process;
    use anyhow::{anyhow, Context};
    use gtk::traits::{ContainerExt, EditableExt, LabelExt, WidgetExt, EntryExt};
    use libphosh::LockscreenPage;

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
        type ParentType = libphosh::Lockscreen;
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
                .connect_closure("login", false, closure_local!(@weak-allow-none self as this => move |_: UserSessionPage| {
                    let this = if let Some(this) = this { this } else {
                        return;
                    };

                    this.obj().upcast_ref::<Widget>().set_sensitive(false);
                    glib::spawn_future_local(clone!(@strong greetd_sender, @strong greetd_receiver => async move {
                        if let Err(err) = this.start_login().await {
                            // TODO: visual thing here (shake the user list view? pop a modal?)
                            panic!("failed to create greetd session: {}", err);
                        }
                        this.obj().upcast_ref::<Widget>().set_sensitive(true);
                    }));
            }));
        }
    }

    impl Lockscreen {
        async fn start_login(&self) -> anyhow::Result<()> {
            let req = Request::CreateSession {
                username: self.user_session_page.get().unwrap().username().unwrap(),
            };

            match self.greetd_req(req).await? {
                Response::AuthMessage { auth_message_type, auth_message} => {
                    self.greetd_auth_msg(auth_message_type, auth_message);
                    Ok(())
                }
                Response::Success => {
                    // Some kind of PAM autologin madness? Just roll with it.
                    self.start_session().await?;
                    Ok(())
                }
                v => Err(anyhow!("unexpected response to start session: {:?}", v))
            }
        }

        async fn start_session(&self) -> anyhow::Result<()> {
            self.obj().lbl_unlock_status().unwrap().set_label("Logging in...");

            let session = self.user_session_page.get().unwrap().session();

            self.greetd_req(Request::StartSession {
                cmd: vec![session.command()],
                env: vec![
                    format!("XDG_SESSION_TYPE={}", session.session_type()),
                    format!("XDG_CURRENT_DESKTOP={}", session.desktop_names()),
                    format!("XDG_SESSION_DESKTOP={}", session.id()),
                    format!("GDMSESSION={}", session.id()),
                ],
            }).await.context("start session")?;

            process::exit(0);

            Ok(())
        }

        fn greetd_auth_msg(&self, auth_message_type: AuthMessageType, auth_message: String) {
            if auth_message == "Password:" {
                self.obj().lbl_unlock_status().unwrap().set_label("Enter Passcode");
            } else {
                self.obj().lbl_unlock_status().unwrap().set_label(&auth_message);
            }
            self.obj().entry_pin().unwrap().delete_text(0, -1);
            self.obj().upcast_ref::<libphosh::Lockscreen>().set_page(LockscreenPage::Unlock);
        }

        async fn greetd_req(&self, req: Request) -> anyhow::Result<Response> {
            self.greetd_sender.get().unwrap().send(req).await.context("send greetd request")?;
            match self.greetd_receiver.get().unwrap().recv().await.context("receive greetd response")? {
                Response::Error { error_type: ErrorType::Error, description } => {
                    Err(anyhow!("greetd error: {}", description))
                }
                resp => Ok(resp)
            }
        }
    }

    impl WidgetImpl for Lockscreen {}
    impl LockscreenImpl for Lockscreen {
        fn unlock_submit_cb(&self) {
            glib::spawn_future_local(clone!(@weak self as this => async move {
                this.obj().upcast_ref::<Widget>().set_sensitive(false);
                this.obj().lbl_unlock_status().unwrap().set_label("Checking...");

                let resp = this.greetd_req(Request::PostAuthMessageResponse {
                    response: Some(this.obj().entry_pin().unwrap().text().to_string())
                }).await;

                if let Err(err) = resp {
                    // TODO: what to do here?
                    panic!("greetd error: {:?}", err);
                }

                match resp.unwrap() {
                    Response::AuthMessage { auth_message_type, auth_message} => {
                        this.greetd_auth_msg(auth_message_type, auth_message);
                        this.obj().upcast_ref::<Widget>().set_sensitive(true);
                    }
                    Response::Error { error_type: ErrorType::AuthError, description } => {
                        this.obj().lbl_unlock_status().unwrap().set_label(&description);
                        // shake_label will re-enable self when finished
                        this.obj().upcast_ref::<libphosh::Lockscreen>().shake_label();
                        glib::timeout_future_seconds(1).await;

                        if let Err(err) = this.start_login().await {
                            // TODO: what to do here?
                            panic!("failed to restart greetd session: {:?}", err);
                        }
                    }
                    Response::Success => this.start_session().await.unwrap(),
                    v => panic!("unexpected greetd response: {:?}", v)
                };

            }));
        }
    }
}
