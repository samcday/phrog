mod virtual_keyboard;
mod virtual_pointer;

use gtk::glib;
use gtk::glib::clone;
use libphosh::prelude::ShellExt;
use libphosh::prelude::WallClockExt;
use libphosh::WallClock;
use std::env::temp_dir;
use std::os::unix::net::UnixListener;

use std::sync::atomic::{AtomicBool, Ordering};

use virtual_keyboard::VirtualKeyboard;
use virtual_pointer::VirtualPointer;
use greetd_ipc::codec::SyncCodec;
use greetd_ipc::AuthMessageType::Secret;
use greetd_ipc::{Request, Response};
use input_event_codes::*;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wayland_client::Connection;
use phrog::shell::Shell;

#[test]
fn test_simple_flow() {
    let nested_phoc = phrog::init(Some("phoc".into()));

    let logged_in = Arc::new(AtomicBool::new(false));
    fake_greetd(&logged_in);

    let wall_clock = WallClock::new();
    wall_clock.set_default();
    let shell = Shell::new();
    shell.set_default();
    shell.set_locked(true);

    let vp = VirtualPointer::new(Connection::connect_to_env().unwrap());
    let kb = VirtualKeyboard::new(Connection::connect_to_env().unwrap());

    let ready_called = Arc::new(AtomicBool::new(false));
    let (ready_tx, ready_rx) = async_channel::bounded(1);
    shell.connect_ready(clone!(@strong ready_called => move |_| {
            ready_called.store(true, Ordering::Relaxed);
            ready_tx.send_blocking(()).expect("notify ready failed");
        }));
    glib::spawn_future_local(clone!(@weak shell => async move {
            ready_rx.recv().await.unwrap();
            glib::timeout_future(Duration::from_millis(500)).await;

            // Move the mouse to where the main user selection should be.
            // TODO: calculate this somehow?
            // If we had access to the Gtk.Widget ref I think it's straightforward to determine
            // absolute (screen) coords...
            let (_, _, width, height) = shell.usable_area();
            let width = width as u32;
            let height = height as u32;
            let x = width / 2;
            let y = height / 2 - 25; // 25 pixel nudge upwards
            vp.move_to(x, y, width, height).await;

            // wait a few millis after mouse move - for improved observation in recordings
            glib::timeout_future(Duration::from_millis(250)).await;
            // then click the main user
            vp.click();

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
