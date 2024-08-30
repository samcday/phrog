use anyhow::{anyhow, Context};
use gtk::{gdk, gio};
use wayland_client::protocol::wl_registry;
use crate::nested_phoc::NestedPhoc;

mod keypad_shuffle;
mod lockscreen;
pub mod nested_phoc;
mod session_object;
mod sessions;
pub mod shell;
mod users;
mod user_session_page;

pub const APP_ID: &str = "mobi.phosh.phrog";

struct DetectPhoc (bool);

impl wayland_client::Dispatch<wl_registry::WlRegistry, ()> for DetectPhoc {
    fn event(
        state: &mut Self,
        _: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<DetectPhoc>,
    ) {
        // When receiving events from the wl_registry, we are only interested in the
        // `global` event, which signals a new available global.
        // When receiving this event, we just print its characteristics in this example.
        if let wl_registry::Event::Global { interface, .. } = event {
            if interface == "phosh_private" {
                state.0 = true;
            }
        }
    }
}

fn is_phoc_detected() -> anyhow::Result<bool> {
    let conn = wayland_client::Connection::connect_to_env()?;

    // Retrieve the WlDisplay Wayland object from the connection. This object is
    // the starting point of any Wayland program, from which all other objects will
    // be created.
    let display = conn.display();

    // Create an event queue for our event processing
    let mut event_queue = conn.new_event_queue();
    // And get its handle to associate new objects to it
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());
    let detect = &mut DetectPhoc(false);
    event_queue.roundtrip(detect)?;
    Ok(detect.0)
}

pub fn init(phoc: Option<String>) -> anyhow::Result<Option<NestedPhoc>> {
    gio::resources_register_include!("phrog.gresource").context("failed to register resources.")?;

    let mut nested_phoc = if let Some(phoc_binary) = phoc {
        if !is_phoc_detected().context("failed to detect Wayland compositor globals")? {
            Some(NestedPhoc::new(&phoc_binary))
        } else { None }
    } else { None };

    gdk::set_allowed_backends("wayland");

    let display = if let Some(nested_phoc) = nested_phoc.as_mut() {
        let display_name = nested_phoc.wait_for_startup();
        std::env::set_var("WAYLAND_DISPLAY", &display_name);
        gdk::init();
        gdk::Display::open(&display_name)
    } else {
        gdk::init();
        gdk::Display::default()
    };

    if display.is_none() {
        return Err(anyhow!("failed GDK init"));
    }

    gtk::init().unwrap();
    libhandy::init();
    Ok(nested_phoc)
}
