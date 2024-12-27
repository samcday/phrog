use clap::Parser;
use gtk::glib::*;
use gtk::Application;
use libphosh::prelude::*;
use libphosh::WallClock;
use nix::libc::{SIGTERM};
use phrog::shell::Shell;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "true",
        help = "Launch nested phoc compositor, if the current compositor is not already phoc"
    )]
    phoc: bool,

    #[arg(short, long, default_value = "false",
        help = "Fake interactions with greetd (useful for local testing)"
    )]
    fake: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // TODO: check XDG_RUNTIME_DIR here? Angry if not set? Default?

    let _nested_phoc = phrog::init(if args.phoc {
        Some(String::from("phoc"))
    } else { None });

    let _app = Application::builder().application_id(phrog::APP_ID).build();

    let wall_clock = WallClock::new();
    wall_clock.set_default();

    let shell: Shell = Object::builder()
        .property("fake-greetd", args.fake)
        .build();
    shell.set_default();
    shell.set_locked(true);

    shell.connect_ready(|_| {
        println!("Shell is ready");
    });

    unix_signal_add_local_once(SIGTERM, || {
        gtk::main_quit();
    });

    // TODO: upstream changes to expose phosh_log_set_log_domains
    // #[cfg(feature = "static")]
    // let mut debug_mode = false;
    // unix_signal_add(SIGUSR1, move || {
    //     // static only because libphosh isn't exporting phosh_log_set_log_domains (yet?)
    //     #[cfg(feature = "static")]
    //     {
    //         let shell = libphosh::Shell::default().downcast::<Shell>().unwrap();
    //         if debug_mode {
    //             g_warning!("🐸", "Ribbit ribbit!");
    //             debug_mode = false;
    //             let prev = CString::new(
    //                 std::env::var("G_MESSAGES_DEBUG").unwrap_or_default()).unwrap();
    //             unsafe { libphosh::ffi::phosh_log_set_log_domains(prev.as_ptr()); }
    //             shell.top_panel().style_context().remove_class("debug");
    //         } else {
    //             g_warning!("🐸", "Ribbit!");
    //             debug_mode = true;
    //             unsafe { libphosh::ffi::phosh_log_set_log_domains(c"all".as_ptr()); }
    //             shell.top_panel().style_context().add_class("debug");
    //         }
    //     }
    //     ControlFlow::Continue
    // });

    gtk::main();

    Ok(())
}
