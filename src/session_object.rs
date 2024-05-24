use gtk::glib;
use gtk::glib::Object;

glib::wrapper! {
    pub struct SessionObject(ObjectSubclass<imp::SessionObject>);
}

impl SessionObject {
    pub fn new(
        id: &str,
        name: &str,
        session_type: &str,
        command: &str,
        desktop_names: &str,
    ) -> Self {
        Object::builder()
            .property("id", id.to_string())
            .property("name", name.to_string())
            .property("session-type", session_type.to_string())
            .property("command", command.to_string())
            .property("desktop-names", desktop_names.to_string())
            .build()
    }
}

mod imp {
    use gtk::glib;
    use gtk::glib::Properties;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use std::cell::OnceCell;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::SessionObject)]
    pub struct SessionObject {
        #[property(get, set)]
        id: OnceCell<String>,
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
}
