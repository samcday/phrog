pub mod keypad_shuffle;
pub mod lockscreen;
pub mod shell;
pub mod supervised_child;

mod dbus;
mod session_object;
mod sessions;
mod user_session_page;
mod user;

use anyhow::{anyhow, Context};
use gtk::{gdk, gio};
use wayland_client::protocol::wl_registry;

pub const APP_ID: &str = "mobi.phosh.phrog";

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
        return Err(anyhow!("Phoc parent compositor not detected"))
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
