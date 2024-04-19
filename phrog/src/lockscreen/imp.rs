use gtk::glib;
use gtk::prelude::ContainerExt;
use gtk::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
use phosh_dm::LockscreenExt;
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
        if let Some(c) = self.obj().carousel() {
            // Scalpel approach: remove the first page from the default lockscreen (clock widget)
            println!("carousel has {} pages", c.n_pages());
            c.remove(c.children().first().unwrap());
            // Up next: grab Phog user selection composite template:
            // https://gitlab.com/mobian1/phog/-/blob/main/src/ui/lockscreen.ui#L20-90
            // Bind that to a new PhrogUser widget (parent GtkBox)
            // Construct that new widget and prepend it to carousel.
        }
    }
}

impl LayerSurfaceImpl for Lockscreen {
}

impl LockscreenImpl for Lockscreen {
}
