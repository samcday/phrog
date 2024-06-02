mod keypad_shuffle;
mod lockscreen;
mod session_object;
mod sessions;
mod shell;
mod user_session_page;
mod users;

use std::io::{BufRead, BufReader, Read};
use std::process::Stdio;
use crate::shell::Shell;
use clap::Parser;
use gtk::{Application, gdk, gio};
use gtk::glib::{g_info, StaticType};
use libphosh::prelude::*;
use libphosh::WallClock;

pub const APP_ID: &str = "com.samcday.phrog";

const PHOC_RUNNING_PREFIX: &'static str = "Running compositor on wayland display '";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "phoc",
        help = "Launch nested phoc compositor if necessary"
    )]
    phoc: Option<String>,
}

pub fn init(phoc: Option<String>) {
    gio::resources_register_include!("phrog.gresource").expect("Failed to register resources.");

    let display = if let Some(phoc_binary) = phoc {
        let display_name = spawn_phoc(&phoc_binary).expect("failed to spawn phoc");
        g_info!("phrog", "spawned phoc on {}", display_name);
        std::env::set_var("WAYLAND_DISPLAY", &display_name);
        gdk::set_allowed_backends("wayland");
        gdk::init();
        gdk::Display::open(&display_name)
    } else {
        gdk::set_allowed_backends("wayland");
        gdk::init();
        gdk::Display::default()
    };

    if display.is_none() {
        panic!("failed GDK init");
    }

    gtk::init().unwrap();
    libphosh::init();
}

pub fn spawn_phoc(binary: &str) -> Option<String> {
    let mut phoc = std::process::Command::new(binary)
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    // Wait for startup message.
    let mut display = None;
    for line in BufReader::new(phoc.stdout.as_mut().unwrap()).lines() {
        let line = line.unwrap();
        if line.starts_with(PHOC_RUNNING_PREFIX) {
            display = Some(
                line.strip_prefix(PHOC_RUNNING_PREFIX)
                    .unwrap()
                    .strip_suffix("'")
                    .unwrap()
                    .to_string(),
            );
            break;
        }
    }

    ctrlc::set_handler(move || {
        phoc.kill().unwrap();
        phoc.wait().unwrap();
    }).unwrap();

    display
}

fn main() {
    let mut args = Args::parse();
    args.phoc = args.phoc.and_then(|v| if v == "" { None } else { Some(v) });

    // TODO: check XDG_RUNTIME_DIR here? Angry if not set? Default?

    init(args.phoc);

    let _app = Application::builder().application_id(APP_ID).build();

    let wall_clock = WallClock::new();
    wall_clock.set_default();

    let shell = Shell::new();
    shell.set_default();

    shell.connect_ready(|_| {
        println!("Shell is ready");
    });

    shell.set_locked(true);

    gtk::main();
}

#[cfg(test)]
mod test {
    use std::ptr::read;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use gtk::glib;
    use gtk::glib::clone;
    use libphosh::prelude::ShellExt;
    use libphosh::prelude::WallClockExt;
    use libphosh::WallClock;
    use crate::init;
    use crate::shell::Shell;

    #[test]
    fn shell_ready() {
        init(None);

        let wall_clock = WallClock::new();
        wall_clock.set_default();
        let shell = Shell::new();
        shell.set_default();
        shell.set_locked(true);

        let mut ready_called = Arc::new(AtomicBool::new(false));
        let (ready_tx, ready_rx) = async_channel::bounded(1);
        shell.connect_ready(clone!(@strong ready_called => move |_| {
            ready_called.store(true, Ordering::Relaxed);
            ready_tx.send_blocking(()).expect("notify ready failed");
        }));
        glib::spawn_future_local(clone!(@weak shell => async move {
            ready_rx.recv().await.unwrap();
            gtk::main_quit();
        }));

        gtk::main();
        assert!(ready_called.load(Ordering::Relaxed));
    }
}
