use glib::{Class, prelude::*, subclass::prelude::*};
use glib::ffi::gboolean;
use glib::translate::*;
use crate::Shell;

pub trait ShellImpl: ShellImplExt + ObjectImpl {
    fn setup(&self) {
        self.parent_setup();
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ShellImplExt> Sealed for T {}
}

pub trait ShellImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_setup(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PhoshShellClass;
            if let Some(f) = (*parent_class).setup {
                f(self.obj().unsafe_cast_ref::<Shell>().to_glib_none().0)
            }
        }
    }
}
impl<T: ShellImpl> ShellImplExt for T {}

unsafe impl<T: ShellImpl> IsSubclassable<T> for Shell {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.setup = Some(shell_setup::<T>);
    }
}

unsafe extern "C" fn shell_setup<T: ShellImpl>(ptr: *mut ffi::PhoshShell) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.setup();
}
