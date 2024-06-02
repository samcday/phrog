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
use gtk::glib::{g_critical, g_info, StaticType};
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
    gio::resources_register_include!("phrog.gresource")
        .expect("Failed to register resources.");

    let mut args = Args::parse();

    // TODO: check XDG_RUNTIME_DIR here? Angry if not set? Default?

    args.phoc = args.phoc.and_then(|v| if v == "" { None } else { Some(v) });


    let display = if let Some(phoc_binary) = args.phoc {
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
    let _app = Application::builder().application_id(APP_ID).build();

    let wall_clock = WallClock::new();
    wall_clock.set_default();

    libphosh::init();

    let shell = Shell::new();
    shell.set_default();

    shell.connect_ready(|_| {
        println!("Shell is ready");
    });

    shell.set_locked(true);

    gtk::main();
}
