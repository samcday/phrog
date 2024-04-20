mod imp;

use gtk::glib;
use glib::Object;

glib::wrapper! {
    pub struct Lockscreen(ObjectSubclass<imp::Lockscreen>)
        @extends phosh_dm::Lockscreen;
}

impl Lockscreen {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Lockscreen {
    fn default() -> Self {
        Self::new()
    }
}
