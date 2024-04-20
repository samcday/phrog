use glib::{Class, subclass::prelude::*};
use crate::LayerSurface;

pub trait LayerSurfaceImpl: LayerSurfaceImplExt + ObjectImpl {
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::LayerSurfaceImplExt> Sealed for T {}
}

pub trait LayerSurfaceImplExt: sealed::Sealed + ObjectSubclass {
}
impl<T: LayerSurfaceImpl> LayerSurfaceImplExt for T {}

unsafe impl<T: LayerSurfaceImpl> IsSubclassable<T> for LayerSurface {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
