mod common;

use gtk::{glib, Window};
use gtk::glib::clone;
use libphosh::prelude::ShellExt;
use libphosh::prelude::WallClockExt;
use libphosh::WallClock;
use std::env::temp_dir;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

use greetd_ipc::codec::SyncCodec;
use greetd_ipc::AuthMessageType::Secret;
use greetd_ipc::{Request, Response};
use input_event_codes::*;
use phrog::shell::Shell;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use anyhow::Context;
use gtk::prelude::ListBoxExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::traits::WidgetExt;
use common::*;
use wayland_client::Connection;
use zbus::zvariant::ObjectPath;

fn system_dbus(tmpdir: &Path) -> Child {
    let config_path = tmpdir.join("system-dbus.xml");
    let sock_path = tmpdir.join("system.sock");
    let dbus_path = format!("unix:path={}", sock_path.display());
    std::fs::write(&config_path, format!(r#"
    <!DOCTYPE busconfig PUBLIC "-//freedesktop//DTD D-Bus Bus Configuration 1.0//EN"
     "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
    <busconfig>
      <type>system</type>
      <keep_umask/>
      <listen>{}</listen>
      <policy context="default">
        <allow send_destination="*" eavesdrop="true"/>
        <allow eavesdrop="true"/>
        <allow own="*"/>
      </policy>
    </busconfig>
    "#, dbus_path)).expect("failed to write system dbus config");

    std::env::set_var("DBUS_SYSTEM_BUS_ADDRESS", dbus_path);
    let child = std::process::Command::new("dbus-daemon")
        .arg(format!("--config-file={}", config_path.to_str().unwrap()))
        .stdout(Stdio::inherit())
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to launch dbus-daemon");

    let start = Instant::now();
    while !sock_path.exists() {
        if start.elapsed() > Duration::from_secs(5) {
            panic!("dbus-daemon failed to launch");
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    child
}

struct AccountsFixture {}
struct UserFixture {}

#[zbus::interface(name = "org.freedesktop.Accounts")]
impl AccountsFixture {
    async fn list_cached_users(&self) -> Vec<ObjectPath> {
        vec![ObjectPath::from_static_str_unchecked("/org/freedesktop/Accounts/1")]
    }
}

#[zbus::interface(name = "org.freedesktop.Accounts.User")]
impl UserFixture {
    #[zbus(property)]
    async fn real_name(&self) -> &str {
        "Phoshi"
    }
    #[zbus(property)]
    async fn user_name(&self) -> &str {
        "phoshi"
    }
    #[zbus(property)]
    async fn icon_file(&self) -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/phoshi.png").display().to_string()
    }
}

async fn run_accounts_fixture() -> anyhow::Result<zbus::Connection> {
    let connection = zbus::Connection::system().await.context("failed to connect to system bus")?;
    connection
        .object_server()
        .at("/org/freedesktop/Accounts", AccountsFixture {})
        .await.context("failed to serve org.freedesktop.Accounts")?;
    connection
        .object_server()
        .at("/org/freedesktop/Accounts/1", UserFixture {})
        .await.context("failed to serve org.freedesktop.Accounts.User")?;
    connection
        .request_name("org.freedesktop.Accounts")
        .await.context("failed to request name")?;
    Ok(connection)
}

fn start_recording(name: &str) -> Option<Child> {
    if let Ok(base_path) = std::env::var("RECORD_TESTS") {
        if let Ok(child) = std::process::Command::new("wf-recorder")
            .arg("-f")
            .arg(PathBuf::from(base_path).join(format!("{}.mp4", name)))
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        { Some(child) } else { None }
    } else {
        None
    }
}

#[test]
fn test_simple_flow() {
    let tmp = tempdir::TempDir::new("phrog-test-system-dbus").unwrap();
    let _system_dbus = system_dbus(tmp.path());

    let nested_phoc = phrog::init(Some("phoc".into()));

    let _conn = async_global_executor::block_on(async move {
        run_accounts_fixture().await.unwrap()
    });

    let logged_in = Arc::new(AtomicBool::new(false));
    fake_greetd(&logged_in);

    let wall_clock = WallClock::new();
    wall_clock.set_default();
    let shell = Shell::new();
    shell.set_default();
    shell.set_locked(true);

    let ready_called = Arc::new(AtomicBool::new(false));
    let (ready_tx, ready_rx) = async_channel::bounded(1);
    shell.connect_ready(clone!(@strong ready_called => move |shell| {
        ready_called.store(true, Ordering::Relaxed);

        let (_, _, width, height) = shell.usable_area();
        let vp = VirtualPointer::new(Connection::connect_to_env().unwrap(), width as _, height as _);
        let kb = VirtualKeyboard::new(Connection::connect_to_env().unwrap());
        ready_tx.send_blocking((vp, kb)).expect("notify ready failed");
    }));

    glib::spawn_future_local(clone!(@weak shell => async move {
        let (vp, kb) = ready_rx.recv().await.unwrap();
        glib::timeout_future(Duration::from_millis(2000)).await;
        // Move the mouse to first user ro  w and click on it.
        let lockscreen = unsafe { phrog::lockscreen::INSTANCE.as_mut().unwrap() };
        let usp = lockscreen.imp().user_session_page.get().unwrap();

        vp.click_on(usp.imp().box_users.selected_row().as_ref().unwrap()).await;

        // wait for keypad page to slide in
        glib::timeout_future(Duration::from_millis(500)).await;

        // type password (uh, literally)
        let keys = [
            KEY_P!(),
            KEY_A!(),
            KEY_S!(),
            KEY_S!(),
            KEY_W!(),
            KEY_O!(),
            KEY_R!(),
            KEY_D!(),
            KEY_ENTER!(),
        ];
        for key in keys {
            kb.keypress(key).await;
            glib::timeout_future(Duration::from_millis(100)).await;
        }

        glib::timeout_future(Duration::from_millis(2500)).await;
        gtk::main_quit();
    }));

    let _recording = start_recording("simple-flow");
    gtk::main();

    assert!(ready_called.load(Ordering::Relaxed));
    assert!(logged_in.load(Ordering::Relaxed));
}

fn fake_greetd(logged_in: &Arc<AtomicBool>) {
    let path = temp_dir().join(format!(
        ".phrog-test-greetd-{}.sock",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));
    std::env::set_var("GREETD_SOCK", &path);
    std::thread::spawn(clone!(@strong logged_in => move || {
            let listener = UnixListener::bind(&path).unwrap();
            loop {
                let (mut stream, _addr) = listener
                    .accept()
                    .expect("failed to accept greetd connection");
                match Request::read_from(&mut stream).unwrap() {
                    Request::CreateSession { .. } => Response::AuthMessage {
                        auth_message_type: Secret,
                        auth_message: "Password:".to_string(),
                    }
                    .write_to(&mut stream)
                    .unwrap(),
                    req => panic!("wrong request: {:?}", req),
                }

                match Request::read_from(&mut stream).unwrap() {
                    Request::PostAuthMessageResponse {
                        response: Some(password),
                    } => {
                        assert_eq!(password, "password");
                        Response::Success.write_to(&mut stream).unwrap();
                    }
                    req => panic!("wrong request: {:?}", req),
                }

                match Request::read_from(&mut stream).unwrap() {
                    Request::StartSession { .. } => {
                        Response::Success.write_to(&mut stream).unwrap();
                        logged_in.store(true, Ordering::Relaxed);
                    }
                    req => panic!("wrong request: {:?}", req),
                }
            }
        }));
}
