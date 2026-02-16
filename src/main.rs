use clap::Parser;
use gtk::glib::*;
use gtk::Application;
use libphosh::prelude::*;
use libphosh::WallClock;
use nix::libc::SIGTERM;
use phrog::shell::Shell;
use phrog::{LOCALEDIR, TEXT_DOMAIN};

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
    log::set_logger(&GLIB_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    if gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "").is_none() {
        warn!("failed to set locale");
    }
    if let Err(err) = gettextrs::textdomain("phosh") {
        warn!("failed to set gettext domain: {err}");
    }
    if let Err(err) = gettextrs::bind_textdomain_codeset("phosh", "UTF-8") {
        warn!("failed to set phosh gettext domain codeset: {err}");
    }
    if let Err(err) = gettextrs::bindtextdomain("phosh", LOCALEDIR) {
        warn!("failed to bind phosh gettext domain: {err}");
    }
    if let Err(err) = gettextrs::bind_textdomain_codeset(TEXT_DOMAIN, "UTF-8") {
        warn!("failed to set phrog gettext domain codeset: {err}");
    }
    if let Err(err) = gettextrs::bindtextdomain(TEXT_DOMAIN, LOCALEDIR) {
        warn!("failed to bind phrog gettext domain: {err}");
    }

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
