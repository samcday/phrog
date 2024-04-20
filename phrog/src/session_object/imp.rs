use std::cell::OnceCell;
use gtk::glib;
use gtk::glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::SessionObject)]
pub struct SessionObject {
    #[property(get, set)]
    name: OnceCell<String>,
    #[property(get, set)]
    session_type: OnceCell<String>,
    #[property(get, set)]
    command: OnceCell<String>,
    #[property(get, set)]
    desktop_names: OnceCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for SessionObject {
    const NAME: &'static str = "PhrogSessionObject";
    type Type = super::SessionObject;
}

#[glib::derived_properties]
impl ObjectImpl for SessionObject {}
