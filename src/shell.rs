use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Shell(ObjectSubclass<imp::Shell>)
        @extends libphosh::Shell;
}

impl Shell {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use std::cell::Cell;
    // use crate::keypad_shuffle::ShuffleKeypadQuickSetting;
    use crate::lockscreen::Lockscreen;
    use gtk::glib::{Properties, Type};
    use gtk::prelude::StaticType;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::{gdk, glib, CssProvider, StyleContext};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use libphosh::subclass::shell::ShellImpl;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Shell)]
    pub struct Shell {
        #[property(get, set)]
        fake_greetd: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Shell {
        const NAME: &'static str = "PhrogShell";
        type Type = super::Shell;
        type ParentType = libphosh::Shell;
        fn class_init(_klass: &mut Self::Class) {
            let provider = CssProvider::new();
            provider.load_from_resource("/mobi/phosh/phrog/phrog.css");
            StyleContext::add_provider_for_screen(
                &gdk::Screen::default().unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for Shell {}

    impl ShellImpl for Shell {
        fn get_lockscreen_type(&self) -> Type {
            Lockscreen::static_type()
        }

        // fn load_extension_point(&self, extension_point: String) {
        //     if extension_point == "phosh-quick-setting-widget" {
        //         gio::IOExtensionPoint::implement(
        //             extension_point,
        //             ShuffleKeypadQuickSetting::static_type(),
        //             "keypad-shuffle",
        //             10,
        //         )
        //         .unwrap();
        //     }
        // }
    }
}
