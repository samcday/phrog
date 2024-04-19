use gtk::glib;
use gtk::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass};
use phosh_dm::subclass::layer_surface::LayerSurfaceImpl;
use phosh_dm::subclass::lockscreen::LockscreenImpl;

#[derive(Default)]
pub struct Lockscreen;

#[glib::object_subclass]
impl ObjectSubclass for Lockscreen {
    const NAME: &'static str = "PhoshDMLockscreen";
    type Type = super::Lockscreen;
    type ParentType = phosh_dm::Lockscreen;
}

impl ObjectImpl for Lockscreen {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl LayerSurfaceImpl for Lockscreen {
}

impl LockscreenImpl for Lockscreen {
}
