mod lockscreen;

use std::ffi::{c_char, c_int};
use gtk::glib::{ObjectExt, StaticType};
use gtk::glib::subclass::register_type;
use crate::lockscreen::Lockscreen;

extern "C" {
    fn phosh_log_set_log_domains(domains: *const c_char);
    fn hdy_init();
    fn cui_init(v: c_int);
}

fn main() {
    println!("Hello, world!");

    gtk::init().unwrap();

    // This is necessary to ensure the Lockscreen type is actually registered. Otherwise it's done
    // lazily the first time it's instantiated, but we're currently hacking the type lookup in
    // phosh_lockscreen_new
    let _ = Lockscreen::static_type();

    unsafe {
        phosh_log_set_log_domains(c"all".as_ptr());
        hdy_init();
        cui_init(1);
    }

    let shell = phosh_dm::Shell::default().unwrap();

    shell.connect_ready(|_| {
        println!("Shell is ready");
    });

    println!("shell instance: {:?}", shell);

    gtk::main();
}
