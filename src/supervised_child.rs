use glib::{error, info, warn};
use nix::sys::signal::SIGTERM;
use nix::unistd::Pid;
use std::process::Child;
use std::thread::sleep;
use std::time::{Duration, Instant};

static G_LOG_DOMAIN: &str = "phrog-supervised-child";

pub struct SupervisedChild {
    pub child: Child,
    name: String,
}

impl SupervisedChild {
    pub fn new(name: &str, child: Child) -> Self {
        Self {
            child,
            name: name.to_string(),
        }
    }

    pub fn stop(&mut self) {
        let pid = Pid::from_raw(self.child.id() as _);
        let label = format!("{} ({})", self.name, pid);
        info!("Stopping process {} with SIGTERM", label);
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
                warn!("Process {} ignored SIGTERM. Killing...", label);
            }
            Err(err) => warn!("Failed to SIGTERM process {}: {}", label, err),
        }

        if let Err(err) = self.child.kill() {
            error!("Failed to kill process {}: {}", label, err);
        }
    }
}

impl Drop for SupervisedChild {
    fn drop(&mut self) {
        self.stop();
    }
}
