use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Shell(ObjectSubclass<imp::Shell>)
        @extends phosh_dm::Shell;
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
    use gtk::glib;
    use gtk::glib::Type;
    use gtk::prelude::StaticType;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use phosh_dm::subclass::shell::ShellImpl;
    use crate::lockscreen::Lockscreen;

    #[derive(Default)]
    pub struct Shell;

    #[glib::object_subclass]
    impl ObjectSubclass for Shell {
        const NAME: &'static str = "PhrogShell";
        type Type = super::Shell;
        type ParentType = phosh_dm::Shell;
    }

    impl ObjectImpl for Shell {

    }

    impl ShellImpl for Shell {
        fn get_lockscreen_type(&self) -> Type {
            Lockscreen::static_type()
        }
    }
}
