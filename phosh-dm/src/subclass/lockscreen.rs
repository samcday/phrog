use glib::{Cast, Class, subclass::prelude::*};
use glib::translate::ToGlibPtr;
use crate::Lockscreen;
use crate::subclass::layer_surface::LayerSurfaceImpl;

pub trait LockscreenImpl: LockscreenImplExt + ObjectImpl {
    fn unlock_submit_cb(&self) {
        self.parent_unlock_submit_cb();
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::LockscreenImplExt> Sealed for T {}
}

pub trait LockscreenImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_unlock_submit_cb(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PhoshLockscreenClass;
            if let Some(f) = (*parent_class).unlock_submit_cb {
                f(self.obj().unsafe_cast_ref::<Lockscreen>().to_glib_none().0);
            }
        }
    }
}
impl<T: LockscreenImpl> LockscreenImplExt for T {}

unsafe impl<T: LockscreenImpl + LayerSurfaceImpl> IsSubclassable<T> for Lockscreen {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.unlock_submit_cb = Some(lockscreen_unlock_submit_cb::<T>)
    }
}

unsafe extern "C" fn lockscreen_unlock_submit_cb<T: LockscreenImpl>(ptr: *mut ffi::PhoshLockscreen) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.unlock_submit_cb();
}
