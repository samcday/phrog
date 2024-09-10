pub mod dbus;
pub mod virtual_keyboard;
pub mod virtual_pointer;

use std::env::temp_dir;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::process::{Child, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use greetd_ipc::{Request, Response};
use greetd_ipc::AuthMessageType::Secret;
use greetd_ipc::codec::SyncCodec;
use gtk::{Button, Revealer};
use gtk::glib::clone;
use gtk::prelude::*;
use libhandy::{Carousel, Deck};
use libphosh::Keypad;
use phrog::lockscreen::Lockscreen;
pub use virtual_pointer::VirtualPointer;
use libhandy::prelude::*;
use phrog::supervised_child::SupervisedChild;

pub fn start_recording(name: &str) -> Option<SupervisedChild> {
    if let Ok(base_path) = std::env::var("RECORD_TESTS") {
        if let Ok(child) = std::process::Command::new("wf-recorder")
            .arg("-f")
            .arg(PathBuf::from(base_path).join(format!("{}.mp4", name)))
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        { Some(SupervisedChild::new("wf-recorder", child)) } else { None }
    } else {
        None
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

pub fn get_lockscreen_bits(lockscreen: &mut Lockscreen) -> (Keypad, Button) {
    // Here we do some yucky traversal of the UI structure in phosh/src/ui/lockscreen.ui in the
    // name of "art". We drill through to find the keypad, and then pick out the individual
    // digits + submit button to drive the UI interactions entirely via mouse.
    // This looks nice for the video recording.
    let deck = lockscreen.child().unwrap().downcast::<Deck>().unwrap();
    let carousel = deck.visible_child().unwrap().downcast::<Carousel>().unwrap();
    let keypad_page = carousel.children().get(2).unwrap().clone().downcast::<gtk::Box>().unwrap();
    let keypad_revealer = keypad_page.children().get(2).unwrap().clone().downcast::<Revealer>().unwrap();
    let keypad = keypad_revealer.child().unwrap().downcast::<Keypad>().unwrap();
    let submit_box = keypad_page.children().get(3).unwrap().clone().downcast::<gtk::Box>().unwrap();
    let submit_btn = submit_box.children().get(0).unwrap().clone().downcast::<Button>().unwrap();
    (keypad, submit_btn)
}
