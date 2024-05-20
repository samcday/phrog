use gtk;
use libphosh as phosh;
use libphosh::prelude::*;

fn main() {
    gtk::init().unwrap();

    let clock = phosh::WallClock::new();
    clock.set_default();

    let shell = phosh::Shell::new();
    shell.set_default();

    shell.connect_ready(|_| {
        glib::g_message!("example", "Rusty shell ready");
    });

    gtk::main();
}
