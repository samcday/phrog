use glib::Object;
use greetd_ipc::AuthMessageType::Secret;
use greetd_ipc::ErrorType::AuthError;
use greetd_ipc::{Request, Response};
use gtk::glib;

use crate::TEXT_DOMAIN;

static G_LOG_DOMAIN: &str = "phrog-lockscreen";

glib::wrapper! {
    pub struct Lockscreen(ObjectSubclass<imp::Lockscreen>)
        @extends libphosh::Lockscreen, gtk::Widget, gtk::Window, gtk::Bin;
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
    use super::{tr, G_LOG_DOMAIN};
    use crate::lockscreen::fake_greetd_interaction;
    use crate::shell::Shell;
    use crate::user_session_page::UserSessionPage;
    use crate::APP_ID;
    use anyhow::{anyhow, Context};
    use async_channel::{Receiver, Sender};
    use glib::{error, g_message, info, warn};
    use greetd_ipc::codec::SyncCodec;
    use greetd_ipc::{AuthMessageType, ErrorType, Request, Response};
    use gtk::gio::Settings;
    use gtk::glib::{clone, closure_local, timeout_add_once, ObjectExt, Properties};
    use gtk::prelude::SettingsExtManual;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};
    use libphosh::prelude::*;
    use libphosh::subclass::lockscreen::LockscreenImpl;
    use libphosh::LockscreenPage;
    use std::cell::{OnceCell, RefCell};
    use std::os::unix::net::UnixStream;
    use std::time::Duration;

    const QUIT_DELAY: u64 = 500;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Lockscreen)]
    pub struct Lockscreen {
        #[property(get, set)]
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
            let mut sock = std::env::var("GREETD_SOCK")
                .ok()
                .and_then(|path| UnixStream::connect(path).ok());
            while let Ok(req) = greetd_req_recv.recv_blocking() {
                let resp = if let Some(ref mut sock) = sock {
                    req.write_to(sock)
                        .and_then(|_| Response::read_from(sock))
                        .unwrap_or_else(|err| Response::Error {
                            error_type: ErrorType::Error,
                            description: err.to_string(),
                        })
                } else {
                    Response::Error {
                        error_type: ErrorType::Error,
                        description: "Greetd not connected".into(),
                    }
                };

                if let Err(err) = greetd_resp_send.send_blocking(resp) {
                    error!("error sending greetd response on channel: {}", err);
                    continue;
                }
            }
        });

        (greetd_req_send, greetd_resp_recv)
    }

    #[glib::derived_properties]
    impl ObjectImpl for Lockscreen {
        fn constructed(&self) {
            let self_obj = self.obj();
            let usp = UserSessionPage::new();

            // Default unlock status in PhoshLockscreen is "Enter Passcode", which doesn't make
            // sense in our case.
            self_obj.set_unlock_status("");

            // Insert the UserSessionPage widget into the "extra page" of Phosh.Lockscreen.
            // This sits in-between the Info and Unlock (keypad) pages.
            // We default to this page (which means inactivity bounces user back to it).
            self_obj.add_extra_page(&usp);
            self_obj.set_default_page(LockscreenPage::Extra);

            // Add a signal handler for when Phosh.Lockscreen active page changes.
            // We hook up greetd session initiation/cancellation to this.
            self_obj.connect_page_notify(clone!(@weak self as this => move |ls| {
                glib::spawn_future_local(clone!(@weak ls => async move {
                    // Page is lockscreen, begin greetd conversation.
                    if ls.page() == LockscreenPage::Unlock {
                        this.obj().set_default_page(LockscreenPage::Unlock);
                        this.create_session().await;
                    } else {
                        // No longer on unlock, cancel session.
                        this.obj().set_default_page(LockscreenPage::Extra);
                        this.cancel_session().await;
                        this.session.replace(None);
                        // Make absolutely sure that lockscreen is sensitive again.
                        // This should already be taken care of elsewhere, but if we somehow hit
                        // an edge case in the convoluted dance with greetd, we really don't want
                        // the user to end up with a lockscreen that cannot be interacted with, as
                        // that deadlocks the whole UI, basically.
                        this.obj().set_sensitive(true);
                    }
                }));
            }));

            // Add a handler for the UserSessionPage notifying of readiness, which happens when
            // all user+sessions on the system have been loaded. At this point we can decide if
            // the "trivial flow" is suitable (jump straight to keypad if there's only one user and
            // session choice available).
            usp.connect_ready_notify(clone!(@weak self_obj => move |usp| {
                let shell = Shell::default();
                let user_count = usp.imp().box_users.children().len();
                let session_count = shell.sessions().map_or(0, |s| s.n_items());
                // If there's only one user and one session, set the default + active page to the keypad.
                if session_count == 1 && user_count == 1 {
                    self_obj.set_page(LockscreenPage::Unlock);
                }
            }));

            usp.connect_closure(
                "login",
                false,
                closure_local!(@watch self_obj => move |_: UserSessionPage| {
                    self_obj.set_page(LockscreenPage::Unlock);
                }),
            );

            self.user_session_page.set(usp).unwrap();

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
                warn!("greetd CancelSession failed: {}", err);
            }
        }

        async fn create_session(&self) {
            let user = self.user_session_page.get().unwrap().username();
            if user.is_none() || self.session.borrow().eq(&user) {
                // no user selected, or the session for that user is already started
                return;
            }

            self.session.replace(user.clone());
            let username = user.unwrap();
            info!("creating greetd session for user {}", username);
            self.obj().set_unlock_status("Please wait…");
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
                warn!("setting last-user failed {}", err);
            }

            if let Err(err) = settings.set("last-session", session.id()) {
                warn!("setting last-session failed {}", err);
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
            if Shell::default().fake_greetd() {
                return fake_greetd_interaction(req);
            }
            if self.greetd.borrow().is_none() {
                self.greetd.set(Some(run_greetd()));
            }
            let (sender, receiver) = self.greetd.clone().take().unwrap();
            sender.send(req).await.context("send greetd request")?;
            match receiver.recv().await.context("receive greetd response")? {
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
                error!("failed to send greetd request: {:?}", err);
                self.obj().set_unlock_status(&tr("Error, please try again"));
                self.obj().set_sensitive(true);
                return None;
            }

            match resp.unwrap() {
                Response::AuthMessage {
                    auth_message_type,
                    auth_message,
                } => {
                    self.obj().set_unlock_status(&auth_message);
                    if let AuthMessageType::Error = auth_message_type {
                        self.obj().shake_pin_entry();
                        // Lockscreen will be made sensitive at the end of PIN shake.

                        // We can only communicate status via the unlock status label.
                        // As soon as we PostAuthMessageResponse below, that will move to the next
                        // auth attempt and likely a new auth message that will overwrite this one.
                        // So we wait here a second before dismissing the message, to ensure the
                        // user has a chance to notice the message.
                        glib::timeout_future_seconds(1).await;
                    } else {
                        self.obj().set_sensitive(true);
                    }
                    match auth_message_type {
                        AuthMessageType::Info | AuthMessageType::Error => {
                            // Dismiss the message and move on to the next auth question.
                            // TODO: This might mean that some info messages are swallowed.
                            // Currently, we only care about fprintd's Info message, which blocks
                            // on the response until fingerprint reader is deactivated.
                            return Some(Request::PostAuthMessageResponse { response: None });
                        }
                        _ => {
                            // TODO: set GtkEntry input-purpose depending on AuthMessageType.
                        }
                    }
                }
                Response::Success => {
                    self.obj().set_unlock_status("Success. Logging in…");
                    self.start_session().await.unwrap();
                    g_message!("phrog", "launched session, exiting in {}ms", QUIT_DELAY);
                    Shell::default().fade_out(0);
                    // Keep this timeout in sync with fadeout animation duration in phrog.css
                    timeout_add_once(Duration::from_millis(QUIT_DELAY), || {
                        gtk::main_quit();
                    });
                }
                Response::Error {
                    error_type: ErrorType::AuthError,
                    description,
                } => {
                    warn!("auth error: '{}'", description);
                    self.obj()
                        .set_unlock_status(&tr("Login failed, please try again"));
                    self.obj().shake_pin_entry();
                    // Greetd IPC dox seem to suggest that this isn't necessary, but then agreety
                    // does this, and if we don't we get a "session is already being configured"
                    // error. So.
                    self.cancel_session().await;
                    // We hold here for a second, so that the login failure message has a chance
                    // to marinate in users' gray meat. Otherwise, the caller driving this
                    // interaction will fire off the CreateSession immediately, which will then
                    // result in a new AuthMessage that overwrites the unlock status.
                    glib::timeout_future_seconds(1).await;

                    return Some(Request::CreateSession {
                        username: self.user_session_page.get()?.username()?,
                    });
                }
                v => error!("unexpected response to start session: {:?}", v),
            }
            None
        }
    }

    impl WidgetImpl for Lockscreen {}
    impl ContainerImpl for Lockscreen {}
    impl BinImpl for Lockscreen {}
    impl WindowImpl for Lockscreen {}
    impl LockscreenImpl for Lockscreen {
        fn unlock_submit(&self) {
            glib::spawn_future_local(clone!(@weak self as this => async move {
                this.obj().set_unlock_status("Please wait…");
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

fn tr(msgid: &str) -> String {
    gettextrs::dgettext(TEXT_DOMAIN, msgid)
}

fn fake_greetd_interaction(req: Request) -> anyhow::Result<Response> {
    match req {
        Request::CreateSession { .. } => anyhow::Ok(Response::AuthMessage {
            auth_message_type: Secret,
            auth_message: tr("Password:"),
        }),
        Request::PostAuthMessageResponse { response } => {
            if response.is_none() || response.unwrap() != "0" {
                anyhow::Ok(Response::Error {
                    error_type: AuthError,
                    description: tr("Incorrect password (hint: it's '0')"),
                })
            } else {
                anyhow::Ok(Response::Success)
            }
        }
        _ => anyhow::Ok(Response::Success),
    }
}
