use glib::{Class, prelude::*, subclass::prelude::*, Type};
use glib::ffi::GType;
use glib::translate::*;
use crate::Shell;
use crate::traits::ShellExt;

pub trait ShellImpl: ShellImplExt + ObjectImpl {
    fn get_lockscreen_type(&self) -> Type {
        self.parent_get_lockscreen_type()
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
}
impl<T: ShellImpl> ShellImplExt for T {}

unsafe impl<T: ShellImpl> IsSubclassable<T> for Shell {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.get_lockscreen_type = Some(get_lockscreen_type::<T>)
    }
}

unsafe extern "C" fn get_lockscreen_type<T: ShellImpl>(ptr: *mut ffi::PhoshShell) -> GType {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.get_lockscreen_type().into_glib()
}
