pub mod dbus;
pub mod virtual_keyboard;
pub mod virtual_pointer;

use crate::common::virtual_keyboard::VirtualKeyboard;
use async_channel::Receiver;
use greetd_ipc::codec::SyncCodec;
use greetd_ipc::AuthMessageType::Secret;
use greetd_ipc::{Request, Response};
use gtk::gio::Settings;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Button, Grid, Revealer};
use libhandy::Carousel;
use libphosh::prelude::ShellExt;
use libphosh::prelude::WallClockExt;
use libphosh::WallClock;
use phrog::lockscreen::Lockscreen;
use phrog::shell::Shell;
use phrog::supervised_child::SupervisedChild;
use std::env::temp_dir;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;
pub use virtual_pointer::VirtualPointer;

pub struct Test {
    dbus_conn: zbus::Connection,
    pub logged_in: Arc<AtomicBool>,
    pub ready_called: Arc<AtomicBool>,
    pub ready_rx: Receiver<(VirtualPointer, VirtualKeyboard)>,
    recording: Option<SupervisedChild>,
    pub shell: Shell,
    system_dbus: SupervisedChild,
    tmp: TempDir,
    wall_clock: WallClock,
}

impl Test {
    pub fn start(&mut self, name: &str) {
        if let Ok(base_path) = std::env::var("RECORD_TESTS") {
            if let Ok(child) = std::process::Command::new("wf-recorder")
                .arg("-f")
                .arg(PathBuf::from(base_path).join(format!("{}.mp4", name)))
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                self.recording = Some(SupervisedChild::new("wf-recorder", child));
            }
        }

        gtk::main();
    }
}

pub fn test_init() -> Test {
    std::env::set_var("GSETTINGS_BACKEND", "memory");
    let tmp = tempfile::tempdir().unwrap();
    phrog::init().unwrap();
    let system_dbus = dbus::system_dbus(tmp.path());

    let if_settings = Settings::new("org.gnome.desktop.interface");
    // use a more appropriate (moar froggy) accent color
    if_settings.set_string("accent-color", "green").unwrap();

    let dbus_conn = async_global_executor::block_on(async move {
        dbus::run_accounts_fixture().await.unwrap()
    });

    let logged_in = Arc::new(AtomicBool::new(false));
    fake_greetd(&logged_in);

    let wall_clock = WallClock::new();
    wall_clock.set_default();
    let shell = Shell::new();
    shell.set_default();
    shell.set_locked(true);

    let ready_called = Arc::new(AtomicBool::new(false));
    let ready_called2 = ready_called.clone();
    let (ready_tx, ready_rx) = async_channel::bounded(1);
    shell.connect_ready(clone!(@strong ready_called2 => move |shell| {
        ready_called2.store(true, Ordering::Relaxed);

        let (_, _, width, height) = shell.usable_area();
        let vp = VirtualPointer::new(wayland_client::Connection::connect_to_env().unwrap(), width as _, height as _);
        let kb = VirtualKeyboard::new(wayland_client::Connection::connect_to_env().unwrap());
        ready_tx.send_blocking((vp, kb)).expect("notify ready failed");
    }));

    Test {
        dbus_conn,
        logged_in,
        ready_called,
        ready_rx,
        recording: None,
        shell,
        system_dbus,
        tmp,
        wall_clock,
    }
}

pub fn fake_greetd(logged_in: &Arc<AtomicBool>) {
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
                        assert_eq!(password, "0451");
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

pub fn get_lockscreen_bits(lockscreen: &mut Lockscreen) -> (Grid, Button) {
    // Here we do some yucky traversal of the UI structure in phosh/src/ui/lockscreen.ui in the
    // name of "art". We drill through to find the keypad, and then pick out the individual
    // digits + submit button to drive the UI interactions entirely via mouse.
    // This looks nice for the video recording.
    let carousel = lockscreen.child().unwrap().downcast::<Carousel>().unwrap();

    let keypad_page = carousel.children().get(2).unwrap().clone().downcast::<gtk::Box>().unwrap();
    let keypad_revealer = keypad_page.children().get(2).unwrap().clone().downcast::<Revealer>().unwrap();
    let keypad = keypad_revealer.child().unwrap().downcast::<Grid>().unwrap();
    let submit_box = keypad_page.children().get(3).unwrap().clone().downcast::<gtk::Box>().unwrap();
    let submit_btn = submit_box.children().first().unwrap().clone().downcast::<Button>().unwrap();
    (keypad, submit_btn)
}
