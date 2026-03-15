use glib;
use gtk;
use libphosh as phosh;
use libphosh::prelude::*;

fn main() {
    gtk::init().unwrap();

    let clock = phosh::WallClock::new();
    clock.set_default();

    let shell = phosh::Shell::new();
    shell.set_default();

    shell.connect_ready(move |s| {
        glib::g_message!("example", "Rusty shell ready");
        let screenshot_manager = s.screenshot_manager();

        let take_screenshot = glib::clone!(@weak screenshot_manager as sm => move || {
            sm.take_screenshot(None, Some("hello-world"), false, false);
        });

        glib::timeout_add_seconds_local_once(2, take_screenshot);
    });

    gtk::main();
}
