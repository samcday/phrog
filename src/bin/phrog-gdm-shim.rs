use anyhow::{Context, Result};
use async_channel::Sender;
use pam::Client;
use std::ffi::CString;
use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{error::Error, fmt::Display};
use zbus::connection::Builder;
use zbus::fdo;
use zbus::message::Header;
use zbus::names::BusName;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::OwnedObjectPath;
use zbus::Connection;

const GDM_BUS_NAME: &str = "org.gnome.DisplayManager";
const MANAGER_PATH: &str = "/org/gnome/DisplayManager/Manager";
const SESSION_PATH: &str = "/org/gnome/DisplayManager/Session";
const PASSWORD_SERVICE: &str = "gdm-password";
const DEFAULT_PAM_SERVICE: &str = "login";

static CHANNEL_COUNTER: AtomicU64 = AtomicU64::new(0);

#[zbus::proxy(
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1",
    interface = "org.freedesktop.login1.Manager"
)]
trait LoginManager {
    #[zbus(name = "GetSessionByPID")]
    fn get_session_by_pid(&self, pid: u32) -> zbus::Result<OwnedObjectPath>;

    #[zbus(name = "UnlockSession")]
    fn unlock_session(&self, id: &str) -> zbus::Result<()>;
}

#[zbus::proxy(
    default_service = "org.freedesktop.login1",
    interface = "org.freedesktop.login1.Session"
)]
trait LoginSession {
    #[zbus(property)]
    fn active(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn class(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn type_(&self) -> zbus::Result<String>;
}

struct DisplayManager;

#[zbus::interface(name = "org.gnome.DisplayManager.Manager")]
impl DisplayManager {
    async fn register_session(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn register_display(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn open_session(&self) -> fdo::Result<String> {
        Err(fdo::Error::AccessDenied(
            "phrog only supports reauthentication channels".into(),
        ))
    }

    async fn open_reauthentication_channel(
        &self,
        username: &str,
        #[zbus(connection)] connection: &Connection,
        #[zbus(header)] header: Header<'_>,
    ) -> fdo::Result<String> {
        let context = authorize_reauthentication(connection, &header, username).await?;
        let (listener, path, address) = create_channel_listener(context.uid)?;

        spawn_reauthentication_server(listener, path, context);

        Ok(address)
    }

    #[zbus(property)]
    fn version(&self) -> &str {
        "3.5.91"
    }
}

#[derive(Clone)]
struct ReauthContext {
    username: String,
    session_id: String,
    uid: u32,
}

struct UserVerifier {
    context: ReauthContext,
    done: Sender<()>,
    state: Mutex<VerifierState>,
}

#[derive(Default)]
struct VerifierState {
    service_name: Option<String>,
}

#[zbus::interface(name = "org.gnome.DisplayManager.UserVerifier")]
impl UserVerifier {
    fn enable_extensions(&self, _extensions: Vec<String>) -> fdo::Result<()> {
        Ok(())
    }

    async fn begin_verification(
        &self,
        service_name: &str,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        self.begin(service_name, emitter).await
    }

    async fn begin_verification_for_user(
        &self,
        service_name: &str,
        username: &str,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        if username != self.context.username {
            return Err(fdo::Error::AccessDenied(
                "reauthentication username does not match caller session".into(),
            ));
        }

        self.begin(service_name, emitter).await
    }

    async fn answer_query(
        &self,
        service_name: &str,
        answer: &str,
        #[zbus(connection)] connection: &Connection,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        let expected_service = self
            .state
            .lock()
            .map_err(|_| fdo::Error::Failed("verifier state lock poisoned".into()))?
            .service_name
            .clone();

        if expected_service.as_deref() != Some(service_name) {
            return Err(fdo::Error::InvalidArgs(
                "answer did not match the active authentication service".into(),
            ));
        }

        let auth_result = authenticate_password(&self.context.username, answer);
        match auth_result {
            Ok(()) => {
                emitter.verification_complete(service_name).await?;
                unlock_session(connection, &self.context.session_id).await?;
                let _ = self.done.try_send(());
            }
            Err(err) => {
                emitter.problem(service_name, &err.to_string()).await?;
                emitter.conversation_stopped(service_name).await?;
                self.state
                    .lock()
                    .map_err(|_| fdo::Error::Failed("verifier state lock poisoned".into()))?
                    .service_name = None;
            }
        }

        Ok(())
    }

    async fn cancel(&self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) -> fdo::Result<()> {
        let service_name = self
            .state
            .lock()
            .map_err(|_| fdo::Error::Failed("verifier state lock poisoned".into()))?
            .service_name
            .take();

        if let Some(service_name) = service_name {
            emitter.conversation_stopped(&service_name).await?;
        }

        let _ = self.done.try_send(());
        Ok(())
    }

    #[zbus(signal)]
    async fn conversation_started(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn conversation_stopped(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn info(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
        info: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn problem(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
        problem: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn info_query(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
        query: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn secret_info_query(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
        query: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn reset(signal_emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn service_unavailable(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
        message: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn verification_failed(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn verification_complete(
        signal_emitter: &SignalEmitter<'_>,
        service_name: &str,
    ) -> zbus::Result<()>;
}

impl UserVerifier {
    fn new(context: ReauthContext, done: Sender<()>) -> Self {
        Self {
            context,
            done,
            state: Mutex::default(),
        }
    }

    async fn begin(&self, service_name: &str, emitter: SignalEmitter<'_>) -> fdo::Result<()> {
        if service_name != PASSWORD_SERVICE {
            emitter
                .service_unavailable(service_name, "only password authentication is supported")
                .await?;
            return Ok(());
        }

        self.state
            .lock()
            .map_err(|_| fdo::Error::Failed("verifier state lock poisoned".into()))?
            .service_name = Some(service_name.to_string());

        emitter.conversation_started(service_name).await?;
        emitter.secret_info_query(service_name, "Password:").await?;
        Ok(())
    }
}

async fn authorize_reauthentication(
    connection: &Connection,
    header: &Header<'_>,
    username: &str,
) -> fdo::Result<ReauthContext> {
    let sender = header
        .sender()
        .ok_or_else(|| fdo::Error::AccessDenied("method call has no sender".into()))?;
    let sender = BusName::from(sender.to_owned());

    let dbus = fdo::DBusProxy::new(connection)
        .await
        .map_err(to_fdo_error)?;
    let pid = dbus
        .get_connection_unix_process_id(sender.clone())
        .await
        .map_err(to_fdo_error)?;
    let uid = dbus
        .get_connection_unix_user(sender)
        .await
        .map_err(to_fdo_error)?;

    let login_manager = LoginManagerProxy::new(connection)
        .await
        .map_err(to_fdo_error)?;
    let session_path = login_manager
        .get_session_by_pid(pid)
        .await
        .map_err(|err| fdo::Error::AccessDenied(format!("caller has no logind session: {err}")))?;
    let session = LoginSessionProxy::builder(connection)
        .path(session_path)
        .map_err(to_fdo_error)?
        .build()
        .await
        .map_err(to_fdo_error)?;

    let session_username = session.name().await.map_err(to_fdo_error)?;
    if session_username != username {
        return Err(fdo::Error::AccessDenied(
            "requested user does not match caller session".into(),
        ));
    }

    let class = session.class().await.map_err(to_fdo_error)?;
    if class != "user" {
        return Err(fdo::Error::AccessDenied(
            "caller is not a user session".into(),
        ));
    }

    let session_type = session.type_().await.map_err(to_fdo_error)?;
    if !matches!(session_type.as_str(), "wayland" | "x11") {
        return Err(fdo::Error::AccessDenied(
            "caller session is not graphical".into(),
        ));
    }

    let state = session.state().await.map_err(to_fdo_error)?;
    let active = session.active().await.map_err(to_fdo_error)?;
    if !active || !matches!(state.as_str(), "active" | "online") {
        return Err(fdo::Error::AccessDenied(
            "caller session is not active".into(),
        ));
    }

    Ok(ReauthContext {
        username: username.to_string(),
        session_id: session.id().await.map_err(to_fdo_error)?,
        uid,
    })
}

fn create_channel_listener(uid: u32) -> fdo::Result<(UnixListener, PathBuf, String)> {
    let runtime_dir = PathBuf::from(format!("/run/user/{uid}"));
    if !runtime_dir.is_dir() {
        return Err(fdo::Error::Failed(format!(
            "runtime directory {} does not exist",
            runtime_dir.display()
        )));
    }

    let counter = CHANNEL_COUNTER.fetch_add(1, Ordering::Relaxed);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| fdo::Error::Failed(err.to_string()))?
        .as_nanos();
    let path = runtime_dir.join(format!(
        "phrog-gdm-shim-{}-{now}-{counter}.sock",
        std::process::id()
    ));

    let listener = UnixListener::bind(&path).map_err(|err| {
        fdo::Error::Failed(format!(
            "failed to bind private reauthentication channel {}: {err}",
            path.display()
        ))
    })?;

    chown_and_chmod(&path, uid, 0o600)?;

    let address = format!("unix:path={}", path.display());
    Ok((listener, path, address))
}

fn chown_and_chmod(path: &PathBuf, uid: u32, mode: u32) -> fdo::Result<()> {
    let path = CString::new(path.as_os_str().as_encoded_bytes()).map_err(|_| {
        fdo::Error::Failed("private reauthentication channel path contains NUL".into())
    })?;

    let no_group = !0 as nix::libc::gid_t;
    let chown_result =
        unsafe { nix::libc::chown(path.as_ptr(), uid as nix::libc::uid_t, no_group) };
    if chown_result != 0 {
        return Err(fdo::Error::Failed(format!(
            "failed to chown private reauthentication channel: {}",
            std::io::Error::last_os_error()
        )));
    }

    let chmod_result = unsafe { nix::libc::chmod(path.as_ptr(), mode as nix::libc::mode_t) };
    if chmod_result != 0 {
        return Err(fdo::Error::Failed(format!(
            "failed to chmod private reauthentication channel: {}",
            std::io::Error::last_os_error()
        )));
    }

    Ok(())
}

fn spawn_reauthentication_server(listener: UnixListener, path: PathBuf, context: ReauthContext) {
    std::thread::spawn(move || {
        let result = match listener.accept() {
            Ok((stream, _)) => {
                async_global_executor::block_on(serve_reauthentication(stream, context))
            }
            Err(err) => Err(err).context("failed to accept private reauthentication connection"),
        };

        if let Err(err) = result {
            eprintln!("phrog-gdm-shim: reauthentication channel failed: {err:#}");
        }

        if let Err(err) = fs::remove_file(&path) {
            if err.kind() != std::io::ErrorKind::NotFound {
                eprintln!("phrog-gdm-shim: failed to remove {}: {err}", path.display());
            }
        }
    });
}

async fn serve_reauthentication(stream: UnixStream, context: ReauthContext) -> Result<()> {
    let (done_sender, done_receiver) = async_channel::bounded(1);
    let verifier = UserVerifier::new(context, done_sender);
    let connection = Builder::unix_stream(stream)
        .p2p()
        .server(zbus::Guid::generate())?
        .serve_at(SESSION_PATH, verifier)?
        .build()
        .await?;

    let _ = done_receiver.recv().await;
    connection.close().await?;
    Ok(())
}

fn authenticate_password(username: &str, password: &str) -> Result<()> {
    let service = std::env::var("PHROG_GDM_SHIM_PAM_SERVICE")
        .unwrap_or_else(|_| DEFAULT_PAM_SERVICE.to_string());
    let mut client = Client::with_password(&service)
        .with_context(|| format!("failed to start PAM service {service}"))?;

    client
        .conversation_mut()
        .set_credentials(username, password);
    client.authenticate().context("PAM authentication failed")
}

async fn unlock_session(connection: &Connection, session_id: &str) -> fdo::Result<()> {
    let login_manager = LoginManagerProxy::new(connection)
        .await
        .map_err(to_fdo_error)?;
    login_manager
        .unlock_session(session_id)
        .await
        .map_err(to_fdo_error)
}

fn to_fdo_error(err: impl Display + Error + Send + Sync + 'static) -> fdo::Error {
    fdo::Error::Failed(err.to_string())
}

async fn run() -> Result<()> {
    let _connection = Builder::system()?
        .serve_at(MANAGER_PATH, DisplayManager)?
        .name(GDM_BUS_NAME)?
        .build()
        .await
        .context("failed to export org.gnome.DisplayManager")?;

    std::future::pending::<()>().await;
    Ok(())
}

fn main() -> Result<()> {
    async_global_executor::block_on(run())
}
