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
    use crate::APP_ID;
    use anyhow::{anyhow, Context};
    use async_channel::{Receiver, Sender};
    use greetd_ipc::codec::SyncCodec;
    use greetd_ipc::{AuthMessageType, ErrorType, Request, Response};
    use gtk::gio::Settings;
    use gtk::glib::{clone, closure_local, g_critical, g_warning, ObjectExt};
    use gtk::prelude::SettingsExtManual;
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
        user_session_page: OnceCell<UserSessionPage>,
        greetd_sender: OnceCell<Sender<Request>>,
        greetd_receiver: OnceCell<Receiver<Response>>,
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
            self.user_session_page.set(UserSessionPage::new()).unwrap();

            self.obj()
                .add_extra_page(self.user_session_page.get().unwrap());
            self.obj().set_default_page(LockscreenPage::Extra);

            let (greetd_sender, greetd_receiver) = run_greetd();

            self.greetd_sender.set(greetd_sender.clone()).unwrap();
            self.greetd_receiver.set(greetd_receiver.clone()).unwrap();
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

            return Ok(());
        }

        async fn greetd_req(&self, req: Request) -> anyhow::Result<Response> {
            self.greetd_sender
                .get()
                .unwrap()
                .send(req)
                .await
                .context("send greetd request")?;
            match self
                .greetd_receiver
                .get()
                .unwrap()
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
                return None;
            }
            let resp = resp.unwrap();

            match resp {
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
                    // If this is an Info message, wait 1 sec and then encourage the caller to
                    // progress the conversation with some awkward silence.
                    if let AuthMessageType::Info = auth_message_type {
                        // TODO: detecting fingerprint prompt phrases could be done here.
                        // show a visual cue (fingerprint icon) somewhere to indicate fprint
                        // liveness or something?
                        glib::timeout_future_seconds(1).await;
                        return Some(Request::PostAuthMessageResponse {
                            response: Some(String::from("\n")),
                        });
                    }
                }
                Response::Success => {
                    self.obj().set_unlock_status("Logging in...");
                    self.start_session().await.unwrap();
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
                        username: self.user_session_page.get().unwrap().username().unwrap(),
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
