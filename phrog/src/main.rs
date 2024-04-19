mod phrog_shell;

use std::ffi::{c_char, c_int};
use phosh_dm::ShellExt;
use gtk::subclass::prelude::*;
use gtk::glib;
use phosh_dm::subclass::shell::ShellImpl;
use crate::phrog_shell::PhrogShell;

extern "C" {
    fn phosh_log_set_log_domains(domains: *const c_char);
    fn hdy_init();
    fn cui_init(v: c_int);
}

fn main() {
    println!("Hello, world!");

    gtk::init().unwrap();

    unsafe {
        // phosh_log_set_log_domains(c"all".as_ptr());
        hdy_init();
        cui_init(1);
    }

    // let shell = phosh_dm::Shell::default().unwrap();
    let shell = PhrogShell::new();

    shell.set_default();

    shell.connect_ready(|_| {
        println!("Shell is ready");
    });

    println!("shell instance: {:?}", shell);

    gtk::main();
}
