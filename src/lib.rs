use gtk::{gdk, gio};
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

pub fn init(phoc: Option<String>) -> Option<NestedPhoc> {
    gio::resources_register_include!("phrog.gresource").expect("Failed to register resources.");

    let mut nested_phoc = if let Some(phoc_binary) = phoc {
        Some(NestedPhoc::new(&phoc_binary))
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
        panic!("failed GDK init");
    }

    gtk::init().unwrap();
    libhandy::init();
    nested_phoc
}
