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
    use std::cell::OnceCell;
    use std::os::unix::net::UnixStream;
    use std::process;
    use anyhow::{anyhow, Context};
    use gtk::traits::WidgetExt;
    use libphosh::prelude::*;
    use libphosh::LockscreenPage;

    #[derive(Default)]
    pub struct Lockscreen {
        user_session_page: OnceCell<UserSessionPage>,
        greetd_sender: OnceCell<Sender<Request>>,
        greetd_receiver: OnceCell<Receiver<Response>>,

    }

    #[glib::object_subclass]
    impl ObjectSubclass for Lockscreen {
        const NAME: &'static str = "PhrogLockscreen";
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

            self.obj().add_extra_page(self.user_session_page.get().unwrap());
            self.obj().set_default_page(LockscreenPage::Extra);

            let (greetd_sender, greetd_receiver) = run_greetd();

            self.greetd_sender.set(greetd_sender.clone()).unwrap();
            self.greetd_receiver.set(greetd_receiver.clone()).unwrap();

            self.user_session_page.get().unwrap()
                .connect_closure("login", false, closure_local!(@weak-allow-none self as this => move |_: UserSessionPage| {
                    let this = if let Some(this) = this { this } else {
                        return;
                    };
                    this.obj().set_page(LockscreenPage::Unlock);
            }));
        }
    }

    impl Lockscreen {
        async fn start_session(&self) -> anyhow::Result<()> {
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

        async fn login(&self) -> anyhow::Result<()> {
            let req = Request::CreateSession {
                username: self.user_session_page.get().unwrap().username().unwrap(),
            };
            match self.greetd_req(req).await? {
                Response::AuthMessage { auth_message_type, auth_message} => {
                    if auth_message != "Password:" {
                        // In future we might want to properly handle more complicated PAM
                        // conversations. For now, we only support simple password auth.
                        return Err(anyhow!("Unexpected auth message {}", auth_message));
                    }
                }
                Response::Success => {
                    // Some kind of PAM autologin madness? Just roll with it.
                    self.start_session().await?;
                }
                v => return Err(anyhow!("unexpected response to start session: {:?}", v))
            }

            let resp = self.greetd_req(Request::PostAuthMessageResponse {
                response: Some(self.obj().pin_entry().to_string())
            }).await?;

            match resp {
                Response::AuthMessage { auth_message_type: _, auth_message} => {
                    // Same deal as previously noted: we're only expecting a simplistic PAM
                    // conversation that asks for password and that's it.
                    return Err(anyhow!("Unexpected auth message {}", auth_message));
                }
                Response::Error { error_type: ErrorType::AuthError, description } => {
                    return Ok(())
                }
                Response::Success => self.start_session().await.unwrap(),
                v => return Err(anyhow!("unexpected response to start session: {:?}", v))
            }

            Ok(())
        }
    }

    impl WidgetImpl for Lockscreen {}
    impl LockscreenImpl for Lockscreen {
        fn unlock_submit_cb(&self) {
            glib::spawn_future_local(clone!(@weak self as this => async move {
                this.obj().upcast_ref::<Widget>().set_sensitive(false);
                if let Err(err) = this.login().await {
                    g_warning!("lockscreen", "Login failed: {}", err);
                }
                // If we got here, the login failed in some way.
                // For now the best we can do is visually indicate this to the user.
                // shake_pin_entry will set the lockscreen as sensitive again when
                // animation completes.
                this.obj().shake_pin_entry();
            }));
        }
    }
}
