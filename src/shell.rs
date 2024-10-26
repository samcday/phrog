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
    use crate::keypad_shuffle::ShuffleKeypadQuickSetting;
    use crate::lockscreen::Lockscreen;
    use crate::APP_ID;
    use gtk::gio::IOExtensionPoint;
    use gtk::gio::Settings;
    use gtk::glib::GString;
    use gtk::glib::{Properties, Type};
    use gtk::prelude::StaticType;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::{gdk, glib, CssProvider, StyleContext};
    use libphosh::ffi::PHOSH_EXTENSION_POINT_QUICK_SETTING_WIDGET;
    use libphosh::prelude::ShellExt;
    use libphosh::subclass::shell::ShellImpl;
    use std::cell::RefCell;
    use std::cell::{Cell, OnceCell};
    use std::collections::HashSet;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Shell)]
    pub struct Shell {
        #[property(get, set)]
        fake_greetd: Cell<bool>,

        #[property(get, set)]
        keypad_shuffle_qs: RefCell<Option<ShuffleKeypadQuickSetting>>,

        provider: Cell<CssProvider>,

        pub dbus_connection: OnceCell<zbus::Connection>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Shell {
        const NAME: &'static str = "PhrogShell";
        type Type = super::Shell;
        type ParentType = libphosh::Shell;
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

            IOExtensionPoint::implement(
                std::str::from_utf8(PHOSH_EXTENSION_POINT_QUICK_SETTING_WIDGET).unwrap(),
                ShuffleKeypadQuickSetting::static_type(),
                "keypad-shuffle",
                10,
            ).expect("failed to implement plugin point");

            let settings = Settings::new("sm.puri.phosh.plugins");
            let mut qs: HashSet<GString> = HashSet::from_iter(settings.strv("quick-settings"));
            qs.insert(GString::from("keypad-shuffle"));
            settings.set_strv("quick-settings", qs.iter().collect::<Vec<&GString>>()).expect("failed to enable keypad-shuffle");

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
