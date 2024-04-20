mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct UserSessionPage(ObjectSubclass<imp::UserSessionPage>)
        @extends gtk::Widget, gtk::Box;
}

impl UserSessionPage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
