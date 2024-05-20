use glib::{Cast, Class, subclass::prelude::*};
use glib::translate::ToGlibPtr;
use crate::Lockscreen;

pub trait LockscreenImpl: LockscreenImplExt + ObjectImpl {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::LockscreenImplExt> Sealed for T {}
}

pub trait LockscreenImplExt: sealed::Sealed + ObjectSubclass {}
impl<T: LockscreenImpl> LockscreenImplExt for T {}

unsafe impl<T: LockscreenImpl> IsSubclassable<T> for Lockscreen {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
