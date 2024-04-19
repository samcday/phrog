mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct PhrogShell(ObjectSubclass<imp::PhrogShell>)
        @extends phosh_dm::Shell;
}

impl PhrogShell {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for PhrogShell {
    fn default() -> Self {
        Self::new()
    }
}
