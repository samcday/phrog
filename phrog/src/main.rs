mod lockscreen;

use std::ffi::{c_char, c_int, CString};
use gtk::glib::{StaticType};
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
        let loglevel = CString::new(std::env::var("G_MESSAGES_DEBUG").unwrap_or("".to_string())).unwrap();
        phosh_log_set_log_domains(loglevel.as_ptr());
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
