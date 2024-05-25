use std::ffi::CStr;
use glib::{Class, prelude::*, subclass::prelude::*, Type};
use glib::ffi::GType;
use glib::translate::*;
use libc::c_char;
use crate::prelude::ShellExt;
use crate::Shell;

pub trait ShellImpl: ShellImplExt + ObjectImpl {
    fn get_lockscreen_type(&self) -> Type {
        self.parent_get_lockscreen_type()
    }
    fn load_extension_point(&self, extension_point: String) {
        self.parent_load_extension_point(extension_point);
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ShellImplExt> Sealed for T {}
}

pub trait ShellImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_get_lockscreen_type(&self) -> Type {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PhoshShellClass;
            if let Some(f) = (*parent_class).get_lockscreen_type {
                return from_glib(f(self.obj().unsafe_cast_ref::<Shell>().to_glib_none().0));
            }
            return Type::UNIT;
        }
    }
    fn parent_load_extension_point(&self, extension_point: String) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PhoshShellClass;
            if let Some(f) = (*parent_class).load_extension_point {
                f(self.obj().unsafe_cast_ref::<Shell>().to_glib_none().0, extension_point.to_glib_none().0);
            }
        }
    }
}
impl<T: ShellImpl> ShellImplExt for T {}

unsafe impl<T: ShellImpl> IsSubclassable<T> for Shell {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.get_lockscreen_type = Some(get_lockscreen_type::<T>);
        klass.load_extension_point = Some(load_extension_point::<T>);
    }
}

unsafe extern "C" fn get_lockscreen_type<T: ShellImpl>(ptr: *mut ffi::PhoshShell) -> GType {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.get_lockscreen_type().into_glib()
}

unsafe extern "C" fn load_extension_point<T: ShellImpl>(ptr: *mut ffi::PhoshShell, extension_point: *const c_char) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.load_extension_point(CStr::from_ptr(extension_point).to_str().unwrap().to_string());
}
