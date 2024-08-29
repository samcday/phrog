use std::io::{BufRead, BufReader};
use std::process::Stdio;
use gtk::glib::g_info;

const PHOC_RUNNING_PREFIX: &str = "Running compositor on wayland display '";

pub struct NestedPhoc {
    process: std::process::Child,
}

impl Drop for NestedPhoc {
    fn drop(&mut self) {
        if self.process.kill().is_ok() {
            let _ = self.process.wait();
        }
    }
}

impl NestedPhoc {
    pub fn new(binary: &str) -> Self {
        Self {
            process: std::process::Command::new(binary)
                .stdout(Stdio::piped())
                .stdin(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("failed to launch nested phoc")
        }
    }

    pub fn wait_for_startup(&mut self) -> String {
        for line in BufReader::new(self.process.stdout.as_mut().expect("failed to read nested phoc output")).lines() {
            let line = line.unwrap();
            if line.starts_with(PHOC_RUNNING_PREFIX) {
                let display_name = line.strip_prefix(PHOC_RUNNING_PREFIX)
                        .and_then(|v| v.strip_suffix('\''))
                        .expect("failed to parse nested phoc display name")
                        .to_string();
                g_info!("phrog", "spawned phoc on {}", display_name);
                return display_name;
            }
        }

        panic!("Nested phoc failed to start up");
    }
}
