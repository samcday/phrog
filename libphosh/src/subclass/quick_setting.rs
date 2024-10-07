use glib::{Cast, Class, subclass::prelude::*};
use glib::translate::ToGlibPtr;
use gtk::subclass::prelude::ButtonImpl;
use crate::QuickSetting;

pub trait QuickSettingImpl: QuickSettingImplExt + ObjectImpl + ButtonImpl {
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::QuickSettingImplExt> Sealed for T {}
}

pub trait QuickSettingImplExt: sealed::Sealed + ObjectSubclass {}
impl<T: QuickSettingImpl> QuickSettingImplExt for T {}

unsafe impl<T: QuickSettingImpl> IsSubclassable<T> for QuickSetting {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
