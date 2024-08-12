use glib::{Cast, Class, subclass::prelude::*};
use glib::translate::ToGlibPtr;
use gtk::subclass::prelude::WidgetImpl;
use crate::Lockscreen;

pub trait LockscreenImpl: LockscreenImplExt + ObjectImpl + WidgetImpl {
    fn unlock_submit(&self) {
        self.parent_unlock_submit();
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::LockscreenImplExt> Sealed for T {}
}

pub trait LockscreenImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_unlock_submit(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PhoshLockscreenClass;
            if let Some(f) = (*parent_class).unlock_submit {
                f(self.obj().unsafe_cast_ref::<Lockscreen>().to_glib_none().0);
            }
        }
    }
}
impl<T: LockscreenImpl> LockscreenImplExt for T {}

unsafe impl<T: LockscreenImpl> IsSubclassable<T> for Lockscreen {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.unlock_submit = Some(crate::subclass::lockscreen::unlock_submit::<T>);
    }
}

unsafe extern "C" fn unlock_submit<T: LockscreenImpl>(ptr: *mut ffi::PhoshLockscreen) {
    let instance = &*(ptr as *mut T::Instance);
    instance.imp().unlock_submit();
}
