use crate::QuickSetting;
use glib::{subclass::prelude::*, Class};
use gtk::subclass::prelude::BoxImpl;

pub trait QuickSettingImpl: QuickSettingImplExt + ObjectImpl + BoxImpl {}

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
