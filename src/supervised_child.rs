use std::process::Child;
use std::thread::sleep;
use std::time::{Duration, Instant};
use gtk::glib::{g_error, g_info, g_warning};
use nix::sys::signal::SIGTERM;
use nix::unistd::Pid;

pub struct SupervisedChild{
    pub child: Child,
    name: String,
}

impl SupervisedChild {
    pub fn new(name: &str, child: Child) -> Self {
        Self { child, name: name.to_string() }
    }

    pub fn stop(&mut self) {
        let pid = Pid::from_raw(self.child.id() as _);
        let label = format!("{} ({})", self.name, pid);
        g_info!("phrog", "Stopping process {} with SIGTERM", label);
        // First try to SIGTERM, allowing maximum of 5 seconds for graceful exit.
        match nix::sys::signal::kill(pid, SIGTERM) {
            Ok(_) => {
                let start = Instant::now();
                while start.elapsed() < Duration::from_secs(5) {
                    if self.child.try_wait().is_ok() {
                        return;
                    }
                    sleep(Duration::from_secs(1));
                }
                g_warning!("phrog", "Process {} ignored SIGTERM. Killing...", label);
            },
            Err(err) => g_warning!("phrog", "Failed to SIGTERM process {}: {}", label, err)
        }

        if let Err(err) = self.child.kill() {
            g_error!("phrog", "Failed to kill process {}: {}", label, err);
        }
    }
}

impl Drop for SupervisedChild {
    fn drop(&mut self) {
        self.stop();
    }
}
