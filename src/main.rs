use clap::Parser;
use gtk::glib::*;
use gtk::Application;
use libphosh::prelude::*;
use libphosh::WallClock;
use nix::libc::SIGTERM;
use phrog::shell::Shell;

static G_LOG_DOMAIN: &str = "phrog";

static GLIB_LOGGER: GlibLogger =
    GlibLogger::new(GlibLoggerFormat::Plain, GlibLoggerDomain::CrateTarget);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Fake interactions with greetd (useful for local testing)"
    )]
    fake: bool,
}

fn main() -> anyhow::Result<()> {
    let _ = gettextrs::bindtextdomain("phosh", "/usr/share/locale");
    gettextrs::textdomain("phosh")?;

    log::set_logger(&GLIB_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let args = Args::parse();

    // TODO: check XDG_RUNTIME_DIR here? Angry if not set? Default?

    phrog::init()?;

    let _app = Application::builder().application_id(phrog::APP_ID).build();

    let wall_clock = WallClock::new();
    wall_clock.set_default();

    let shell: Shell = Object::builder()
        .property("fake-greetd", args.fake)
        .property("overview-visible", false)
        .build();
    shell.set_default();

    shell.connect_ready(|_| {
        info!("Shell is ready");
    });

    unix_signal_add_local_once(SIGTERM, || {
        gtk::main_quit();
    });

    gtk::main();

    Ok(())
}
