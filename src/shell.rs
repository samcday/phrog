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

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {
    use crate::lockscreen::Lockscreen;
    use gtk::glib;
    use gtk::glib::Type;
    use gtk::prelude::StaticType;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use libphosh::subclass::shell::ShellImpl;

    #[derive(Default)]
    pub struct Shell;

    #[glib::object_subclass]
    impl ObjectSubclass for Shell {
        const NAME: &'static str = "PhrogShell";
        type Type = super::Shell;
        type ParentType = libphosh::Shell;
    }

    impl ObjectImpl for Shell {}

    impl ShellImpl for Shell {
        fn get_lockscreen_type(&self) -> Type {
            Lockscreen::static_type()
        }
    }
}
