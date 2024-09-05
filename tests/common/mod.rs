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
use gtk::glib::clone;
pub use virtual_pointer::VirtualPointer;
pub use virtual_keyboard::VirtualKeyboard;

pub fn start_recording(name: &str) -> Option<Child> {
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
