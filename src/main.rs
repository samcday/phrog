use phrog::shell::Shell;
use clap::Parser;
use gtk::glib::Object;
use gtk::Application;
use libphosh::prelude::*;
use libphosh::WallClock;

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

    gtk::main();

    Ok(())
}
