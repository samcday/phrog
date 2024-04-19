use glib::{Class, prelude::*, subclass::prelude::*};
use glib::translate::*;
use crate::Lockscreen;
use crate::subclass::layer_surface::LayerSurfaceImpl;

pub trait LockscreenImpl: LockscreenImplExt + ObjectImpl {

}

mod sealed {
    pub trait Sealed {}
    impl<T: super::LockscreenImplExt> Sealed for T {}
}

pub trait LockscreenImplExt: sealed::Sealed + ObjectSubclass {
    // fn parent_setup_idle_cb(&self) -> bool {
    //     unsafe {
    //         let data = Self::type_data();
    //         let parent_class = data.as_ref().parent_class() as *mut ffi::PhoshLockscreenClass;
    //         if let Some(f) = (*parent_class).setup_idle_cb {
    //             return from_glib(f(self.obj().unsafe_cast_ref::<Lockscreen>().to_glib_none().0));
    //         }
    //         false
    //     }
    // }
}
impl<T: LockscreenImpl> LockscreenImplExt for T {}

unsafe impl<T: LockscreenImpl + LayerSurfaceImpl> IsSubclassable<T> for Lockscreen {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
    }
}
