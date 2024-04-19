use glib::{Class, prelude::*, subclass::prelude::*};
use glib::ffi::gboolean;
use glib::translate::*;
use crate::Shell;

pub trait ShellImpl: ShellImplExt + ObjectImpl {
    fn setup_idle_cb(&self) -> bool {
        self.parent_setup_idle_cb()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ShellImplExt> Sealed for T {}
}

pub trait ShellImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_setup_idle_cb(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PhoshShellClass;
            if let Some(f) = (*parent_class).setup_idle_cb {
                return from_glib(f(self.obj().unsafe_cast_ref::<Shell>().to_glib_none().0));
            }
            false
        }
    }
}
impl<T: ShellImpl> ShellImplExt for T {}

unsafe impl<T: ShellImpl> IsSubclassable<T> for Shell {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.setup_idle_cb = Some(shell_setup_idle_cb::<T>);
    }
}

unsafe extern "C" fn shell_setup_idle_cb<T: ShellImpl>(ptr: *mut ffi::PhoshShell) -> gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.setup_idle_cb().into_glib()
}