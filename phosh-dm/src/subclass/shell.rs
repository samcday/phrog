use glib::{Class, prelude::*, subclass::prelude::*};
use glib::translate::*;
use crate::Shell;

pub trait ShellImpl: ShellImplExt + ObjectImpl {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ShellImplExt> Sealed for T {}
}

pub trait ShellImplExt: sealed::Sealed + ObjectSubclass {}
impl<T: ShellImpl> ShellImplExt for T {}

unsafe impl<T: ShellImpl> IsSubclassable<T> for Shell {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
