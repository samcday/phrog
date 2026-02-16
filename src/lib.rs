pub mod lockscreen;
pub mod shell;
pub mod supervised_child;

mod dbus;
pub mod session_object;
mod sessions;
mod user;
mod user_session_page;

use anyhow::{anyhow, Context};
use gtk::{gdk, gio};
use std::path::Path;
use wayland_client::protocol::wl_registry;

pub const APP_ID: &str = "mobi.phosh.phrog";
pub const TEXT_DOMAIN: &str = "phrog";
pub const LOCALEDIR: &str = "/usr/share/locale";

pub fn i18n_setup(
    locale_dir: &Path,
    locale: Option<&str>,
    language: Option<&str>,
) -> anyhow::Result<()> {
    if let Some(language) = language {
        std::env::set_var("LANGUAGE", language);
    }

    let locale = locale.unwrap_or("");
    gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, locale).with_context(|| {
        if locale.is_empty() {
            "failed to set process locale from environment".to_string()
        } else {
            format!("failed to set process locale to {locale}")
        }
    })?;
    gettextrs::textdomain("phosh").context("failed to set gettext domain")?;
    gettextrs::bind_textdomain_codeset("phosh", "UTF-8")
        .context("failed to set phosh gettext domain codeset")?;
    gettextrs::bindtextdomain("phosh", locale_dir)
        .context("failed to bind phosh gettext domain")?;
    gettextrs::bind_textdomain_codeset(TEXT_DOMAIN, "UTF-8")
        .context("failed to set phrog gettext domain codeset")?;
    gettextrs::bindtextdomain(TEXT_DOMAIN, locale_dir)
        .context("failed to bind phrog gettext domain")?;
    Ok(())
}

struct DetectPhoc(bool);

impl wayland_client::Dispatch<wl_registry::WlRegistry, ()> for DetectPhoc {
    fn event(
        state: &mut Self,
        _: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<DetectPhoc>,
    ) {
        if let wl_registry::Event::Global { interface, .. } = event {
            if interface == "phosh_private" {
                state.0 = true;
            }
        }
    }
}

fn is_phoc_detected() -> anyhow::Result<bool> {
    let conn = wayland_client::Connection::connect_to_env()?;
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());
    let detect = &mut DetectPhoc(false);
    event_queue.roundtrip(detect)?;
    Ok(detect.0)
}

pub fn init() -> anyhow::Result<()> {
    gio::resources_register_include!("phrog.gresource").context("failed to register resources.")?;

    if !is_phoc_detected().context("failed to detect Wayland compositor globals")? {
        return Err(anyhow!("Phoc parent compositor not detected"));
    }

    gdk::set_allowed_backends("wayland");

    gdk::init();

    let display = gdk::Display::default();
    if display.is_none() {
        return Err(anyhow!("failed GDK init"));
    }

    gtk::init()?;
    libhandy::init();
    Ok(())
}
