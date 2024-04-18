use std::ffi::{c_char, c_int};

extern "C" {
    fn phosh_log_set_log_domains(domains: *const c_char);
    fn hdy_init();
    fn cui_init(v: c_int);
}

fn main() {
    println!("Hello, world!");

    gtk::init().unwrap();

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
