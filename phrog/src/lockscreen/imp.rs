use gtk::glib;
use gtk::glib::once_cell::unsync::OnceCell;
use gtk::prelude::ContainerExt;
use gtk::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt};
use phosh_dm::LockscreenExt;
use phosh_dm::subclass::layer_surface::LayerSurfaceImpl;
use phosh_dm::subclass::lockscreen::LockscreenImpl;
use crate::user_session_page::UserSessionPage;

#[derive(Default)]
pub struct Lockscreen {
    user_session_page: OnceCell<UserSessionPage>,
}

#[glib::object_subclass]
impl ObjectSubclass for Lockscreen {
    const NAME: &'static str = "PhoshDMLockscreen";
    type Type = super::Lockscreen;
    type ParentType = phosh_dm::Lockscreen;
}

impl ObjectImpl for Lockscreen {
    fn constructed(&self) {
        self.parent_constructed();

        let user_session_page = UserSessionPage::new();
        self.user_session_page.set(user_session_page).unwrap();

        if let Some(c) = self.obj().carousel() {
            // Remove the first page from the default lockscreen (info widget)
            c.remove(c.children().first().unwrap());

            // Replace it with the user+session selection page.
            c.prepend(self.user_session_page.get().unwrap())
        }
    }
}

impl LayerSurfaceImpl for Lockscreen {
}

impl LockscreenImpl for Lockscreen {
}
