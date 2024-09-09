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
    use std::cell::{Cell, OnceCell, RefCell};
    use std::collections::HashSet;
    use crate::lockscreen::Lockscreen;
    use gtk::glib::{GString, Properties, Type};
    use gtk::prelude::StaticType;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::{gdk, glib, CssProvider, StyleContext};
    use gtk::gio::{IOExtensionPoint, Settings};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use libphosh::ffi::PHOSH_EXTENSION_POINT_QUICK_SETTING_WIDGET;
    use libphosh::prelude::ShellExt;
    use libphosh::subclass::shell::ShellImpl;
    use crate::APP_ID;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Shell)]
    pub struct Shell {
        #[property(get, set)]
        fake_greetd: Cell<bool>,

        #[cfg(feature = "keypad-shuffle")]
        #[property(get, set)]
        keypad_shuffle_qs: RefCell<Option<crate::keypad_shuffle::ShuffleKeypadQuickSetting>>,

        provider: Cell<CssProvider>,

        pub dbus_connection: OnceCell<zbus::Connection>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Shell {
        const NAME: &'static str = "PhrogShell";
        type Type = super::Shell;
        type ParentType = libphosh::Shell;
    }

    impl Shell {
        #[cfg(feature = "keypad-shuffle")]
        fn enable_keypad_shuffle(&self) {
            IOExtensionPoint::implement(
                std::str::from_utf8(PHOSH_EXTENSION_POINT_QUICK_SETTING_WIDGET).unwrap(),
                crate::keypad_shuffle::ShuffleKeypadQuickSetting::static_type(),
                "keypad-shuffle",
                10
            ).expect("failed to implement plugin point");

            let settings = Settings::new("sm.puri.phosh.plugins");
            let mut qs: HashSet<GString> = HashSet::from_iter(settings.strv("quick-settings"));
            qs.insert(GString::from("keypad-shuffle"));
            settings.set_strv("quick-settings", qs.iter().collect::<Vec<&GString>>()).expect("failed to enable keypad-shuffle");
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for Shell {
        fn constructed(&self) {
            let system_dbus = async_global_executor::block_on(zbus::Connection::system()).unwrap();
            self.dbus_connection.set(system_dbus).unwrap();

            self.parent_constructed();

            let provider = CssProvider::new();
            provider.load_from_resource("/mobi/phosh/phrog/phrog.css");
            StyleContext::add_provider_for_screen(
                &gdk::Screen::default().unwrap(),
                &provider,
                // Slightly hacky, we want to be above phosh to override some stuff
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 5,
            );

            self.provider.set(provider);

            #[cfg(feature = "keypad-shuffle")]
            self.enable_keypad_shuffle();

            self.obj().connect_ready(move |shell| {
                let lockscreen = shell.lockscreen_manager().lockscreen()
                    .and_then(|v| v.downcast::<Lockscreen>().ok())
                    .expect("failed to get lockscreen");
                let user_session_page = lockscreen.user_session_page();
                let settings = Settings::new(APP_ID);
                user_session_page.select_user(&settings.string("last-user"));
                user_session_page.select_session(&settings.string("last-session"));
            });
        }
    }

    impl ShellImpl for Shell {
        fn get_lockscreen_type(&self) -> Type {
            Lockscreen::static_type()
        }
    }
}
