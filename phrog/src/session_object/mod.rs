mod imp;

use gtk::glib;
use gtk::glib::Object;

glib::wrapper! {
    pub struct SessionObject(ObjectSubclass<imp::SessionObject>);
}

impl SessionObject {
    pub fn new(name: &str, session_type: &str, command: &str, desktop_names: &str) -> Self {
        Object::builder()
            .property("name", name.to_string())
            .property("session-type", session_type.to_string())
            .property("command", command.to_string())
            .property("desktop-names", desktop_names.to_string())
            .build()
    }
}
