use glib::Object;
use greetd_ipc::AuthMessageType::Secret;
use greetd_ipc::ErrorType::AuthError;
use greetd_ipc::{Request, Response};
use gtk::glib;

glib::wrapper! {
    pub struct Lockscreen(ObjectSubclass<imp::Lockscreen>)
        @extends libphosh::Lockscreen, gtk::Widget;
}

#[cfg(feature = "test")]
pub static mut INSTANCE: Option<Lockscreen> = None;

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
    use crate::lockscreen::fake_greetd_interaction;
    use crate::shell::Shell;
    use crate::user_session_page::UserSessionPage;
    use crate::APP_ID;
    use anyhow::{anyhow, Context};
    use async_channel::{Receiver, Sender};
    use greetd_ipc::codec::SyncCodec;
    use greetd_ipc::{AuthMessageType, ErrorType, Request, Response};
    use gtk::gio::Settings;
    use gtk::glib::{clone, closure_local, g_critical, g_warning, ObjectExt};
    use gtk::prelude::SettingsExtManual;
    use gtk::prelude::*;
    use gtk::glib::PropertySet;
    use gtk::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
    use gtk::subclass::widget::WidgetImpl;
    use gtk::traits::WidgetExt;
    use gtk::{gio, glib};
    use libphosh::prelude::*;
    use libphosh::subclass::lockscreen::LockscreenImpl;
    use libphosh::LockscreenPage;
    use std::cell::{OnceCell, RefCell};
    use std::os::unix::net::UnixStream;

    #[derive(Default)]
    pub struct Lockscreen {
        pub user_session_page: OnceCell<UserSessionPage>,
        greetd: RefCell<Option<(Sender<Request>, Receiver<Response>)>>,
        session: RefCell<Option<String>>,
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
            let mut sock = std::env::var("GREETD_SOCK").ok().and_then(|path| UnixStream::connect(path).ok());
            while let Ok(req) = greetd_req_recv.recv_blocking() {
                let resp = if let Some(ref mut sock) = sock {
                    req.write_to(sock)
                        .and_then(|_| { Response::read_from(sock) })
                        .unwrap_or_else(|err| Response::Error {
                            error_type: ErrorType::Error, description: err.to_string() })
                } else {
                    Response::Error {error_type: ErrorType::Error, description: "Greetd not connected".into()}
                };

                if let Err(err) = greetd_resp_send.send_blocking(resp) {
                    g_critical!("greetd", "error sending greetd response on channel: {}", err);
                    continue;
                }
            }
        });

        (greetd_req_send, greetd_resp_recv)
    }

    impl ObjectImpl for Lockscreen {
        fn constructed(&self) {
            self.user_session_page.set(UserSessionPage::new()).unwrap();

            self.obj()
                .add_extra_page(self.user_session_page.get().unwrap());
            self.obj().set_default_page(LockscreenPage::Extra);

            self.obj()
                .connect_page_notify(clone!(@weak self as this => move |ls| {
                    glib::spawn_future_local(clone!(@weak ls => async move {
                        // Page is lockscreen, begin greetd conversation.
                        if ls.page() == LockscreenPage::Unlock {
                            this.create_session().await;
                        } else {
                            // No longer on unlock, cancel session.
                            this.cancel_session().await;
                        }
                    }));
                }));

            self.user_session_page.get().unwrap().connect_closure(
                "login",
                false,
                closure_local!(@weak-allow-none self as this => move |_: UserSessionPage| {
                        let this = if let Some(this) = this { this } else {
                            return;
                        };
                        this.obj().set_page(LockscreenPage::Unlock);
                }),
            );

            self.parent_constructed();
            #[cfg(feature = "test")]
            unsafe { crate::lockscreen::INSTANCE = Some(self.obj().clone()) };
        }
    }

    impl Lockscreen {
        // Whenever user swipes away from keypad entry page, and after an auth failure, fire off a
        // CancelSession.
        async fn cancel_session(&self) {
            if self.session.borrow().is_none() {
                // no session to cancel
                return;
            }

            if let Err(err) = self.greetd_req(Request::CancelSession).await {
                g_warning!("greetd", "greetd CancelSession failed: {}", err);
            }
            self.session.replace(None);
        }

        async fn create_session(&self) {
            let user = self.user_session_page.get().unwrap().username();
            if user.is_none() || self.session.borrow().eq(&user) {
                // no user selected, or the session for that user is already started
                return;
            }

            self.session.replace(user.clone());
            let username = user.unwrap();
            g_warning!("greetd", "creating greetd session for user {}", username);
            self.obj().set_sensitive(false);
            let mut req = Some(Request::CreateSession { username });
            while let Some(next_req) = req.take() {
                req = self.greetd_interaction(next_req).await;
            }
        }

        async fn start_session(&self) -> anyhow::Result<()> {
            let session = self.user_session_page.get().unwrap().session();

            let settings = Settings::new(APP_ID);
            if let Err(err) =
                settings.set("last-user", self.session.clone().take().unwrap_or_default())
            {
                g_warning!("lockscreen", "setting last-user failed {}", err);
            }

            if let Err(err) = settings.set("last-session", session.id()) {
                g_warning!("lockscreen", "setting last-session failed {}", err);
            }
            self.greetd_req(Request::StartSession {
                cmd: vec![session.command()],
                env: vec![
                    format!("XDG_SESSION_TYPE={}", session.session_type()),
                    format!("XDG_CURRENT_DESKTOP={}", session.desktop_names()),
                    format!("XDG_SESSION_DESKTOP={}", session.id()),
                    format!("GDMSESSION={}", session.id()),
                ],
            })
                .await
                .context("start session")?;

            Ok(())
        }

        async fn greetd_req(&self, req: Request) -> anyhow::Result<Response> {
            if libphosh::Shell::default().downcast::<Shell>().unwrap().fake_greetd() {
                return fake_greetd_interaction(req);
            }
            if self.greetd.borrow().is_none() {
                self.greetd.set(Some(run_greetd()));
            }
            let mut borrow = self.greetd.borrow_mut();
            let (sender, receiver) = borrow.as_mut().unwrap();
            sender
                .send(req)
                .await
                .context("send greetd request")?;
            match receiver
                .recv()
                .await
                .context("receive greetd response")?
            {
                Response::Error {
                    error_type: ErrorType::Error,
                    description,
                } => Err(anyhow!("greetd error: {}", description)),
                resp => Ok(resp),
            }
        }

        async fn greetd_interaction(&self, req: Request) -> Option<Request> {
            let resp = self.greetd_req(req).await;

            if let Err(err) = resp {
                g_critical!("greetd", "failed to send greetd request: {:?}", err);
                self.obj().set_unlock_status("Greetd error, check logs");
                return None;
            }

            match resp.unwrap() {
                Response::AuthMessage {
                    auth_message_type,
                    auth_message,
                } => {
                    g_warning!(
                        "greetd",
                        "got greetd auth message ({:?}) {}",
                        auth_message_type,
                        auth_message
                    );
                    self.obj().set_unlock_status(&auth_message);
                    // TODO: it would be nice to override the GtkEntry input-purpose depending on
                    // AuthMessageType.
                    self.obj().set_sensitive(true);
                    match auth_message_type {
                        AuthMessageType::Info | AuthMessageType::Error =>
                            return Some(Request::PostAuthMessageResponse {
                                response: None,
                            }),
                        _ => {}
                    }
                }
                Response::Success => {
                    self.obj().set_unlock_status("Success. Logging in...");
                    self.start_session().await.unwrap();
                    libphosh::Shell::default().fade_out(0);
                }
                Response::Error {
                    error_type: ErrorType::AuthError,
                    description,
                } => {
                    g_warning!("greetd", "auth error '{}'", description);
                    self.obj().shake_pin_entry();
                    self.cancel_session().await;
                    glib::timeout_future_seconds(1).await;
                    return Some(Request::CreateSession {
                        username: self.user_session_page.get()?.username()?,
                    });
                }
                v => g_critical!("greetd", "unexpected response to start session: {:?}", v),
            }
            None
        }
    }

    impl WidgetImpl for Lockscreen {}
    impl LockscreenImpl for Lockscreen {
        fn unlock_submit(&self) {
            glib::spawn_future_local(clone!(@weak self as this => async move {
                this.obj().set_sensitive(false);
                let mut req = Some(Request::PostAuthMessageResponse {
                    response: Some(this.obj().pin_entry().to_string())
                });
                while let Some(next_req) = req.take() {
                    req = this.greetd_interaction(next_req).await;
                }
                this.obj().clear_pin_entry();
            }));
        }
    }
}

fn fake_greetd_interaction(req: Request) -> anyhow::Result<Response> {
    match req {
        Request::CreateSession { .. } => anyhow::Ok(Response::AuthMessage {
            auth_message_type: Secret,
            auth_message: "Password:".into(),
        }),
        Request::PostAuthMessageResponse { response } => {
            if response.is_none() || response.unwrap() != "0" {
                anyhow::Ok(Response::Error {
                    error_type: AuthError,
                    description: String::from("0"),
                })
            } else {
                anyhow::Ok(Response::Success)
            }
        }
        _ => anyhow::Ok(Response::Success),
    }
}
