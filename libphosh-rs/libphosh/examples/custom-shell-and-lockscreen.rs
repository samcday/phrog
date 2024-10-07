use gtk;
use libphosh as phosh;
use libphosh::prelude::*;

fn main() {
    gtk::init().unwrap();

    let clock = phosh::WallClock::new();
    clock.set_default();

    let shell = custom_shell::CustomShell::new();
    shell.set_default();
    shell.set_locked(true);

    shell.connect_ready(|_| {
        glib::g_message!("example", "Custom Rusty shell ready");
    });

    gtk::main();
}

mod custom_shell {
    use glib::Object;
    use gtk::glib;

    glib::wrapper! {
        pub struct CustomShell(ObjectSubclass<imp::CustomShell>)
            @extends libphosh::Shell;
    }

    impl CustomShell {
        pub fn new() -> Self {
            Object::builder().build()
        }
    }

    impl Default for CustomShell {
        fn default() -> Self {
            Self::new()
        }
    }

    mod imp {
        use gtk::glib;
        use gtk::glib::Type;
        use gtk::prelude::StaticType;
        use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
        use libphosh::subclass::shell::ShellImpl;
        use crate::custom_lockscreen::CustomLockscreen;

        #[derive(Default)]
        pub struct CustomShell;

        #[glib::object_subclass]
        impl ObjectSubclass for CustomShell {
            const NAME: &'static str = "CustomShell";
            type Type = super::CustomShell;
            type ParentType = libphosh::Shell;
        }

        impl ObjectImpl for CustomShell {}

        impl ShellImpl for CustomShell {
            fn get_lockscreen_type(&self) -> Type {
                CustomLockscreen::static_type()
            }
        }
    }
}

mod custom_lockscreen {
    use glib::Object;

    glib::wrapper! {
        pub struct CustomLockscreen(ObjectSubclass<imp::CustomLockscreen>)
            @extends libphosh::Lockscreen;
    }

    impl CustomLockscreen {
        pub fn new() -> Self {
            Object::builder().build()
        }
    }

    impl Default for CustomLockscreen {
        fn default() -> Self {
            Self::new()
        }
    }

    mod imp {
        use gtk::glib::ObjectExt;
        use gtk::subclass::prelude::*;
        use gtk::{glib, Button, Image, Label};
        use gtk::prelude::WidgetExt;
        use libphosh::Lockscreen;
        use libphosh::prelude::LockscreenExt;
        use libphosh::subclass::lockscreen::LockscreenImpl;

        #[derive(Default)]
        pub struct CustomLockscreen {}

        #[glib::object_subclass]
        impl ObjectSubclass for CustomLockscreen {
            const NAME: &'static str = "CustomLockscreen";
            type Type = super::CustomLockscreen;
            type ParentType = Lockscreen;
        }

        impl ObjectImpl for CustomLockscreen {
            fn constructed(&self) {
                self.parent_constructed();
                glib::g_message!("example", "Constructed custom Lockscreen");

                let hi = Image::builder()
                    .icon_name("face-kiss")
                    .pixel_size(100)
                    .build();
                self.obj().add_extra_page(&hi);
                hi.set_visible(true);

                self.obj().connect_lockscreen_unlock(|_| {
                    glib::g_message!("example", "Custom Lockscreen was unlocked.");
                });

                self.obj().connect_page_notify(|me| {
                    glib::g_message!("example", "Lockscreen page changed to {:?}", me.page());
                });
            }
        }

        impl WidgetImpl for CustomLockscreen {}
        impl ContainerImpl for CustomLockscreen {}
        impl BinImpl for CustomLockscreen {}
        impl WindowImpl for CustomLockscreen {}
        impl LockscreenImpl for CustomLockscreen {}
    }
}
