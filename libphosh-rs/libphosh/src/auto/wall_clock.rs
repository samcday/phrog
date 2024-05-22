// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "PhoshWallClock")]
    pub struct WallClock(Object<ffi::PhoshWallClock, ffi::PhoshWallClockClass>);

    match fn {
        type_ => || ffi::phosh_wall_clock_get_type(),
    }
}

impl WallClock {
        pub const NONE: Option<&'static WallClock> = None;
    

    #[doc(alias = "phosh_wall_clock_new")]
    pub fn new() -> WallClock {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::phosh_wall_clock_new())
        }
    }

    #[doc(alias = "phosh_wall_clock_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> WallClock {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::phosh_wall_clock_get_default())
        }
    }
}

impl Default for WallClock {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WallClock>> Sealed for T {}
}

pub trait WallClockExt: IsA<WallClock> + sealed::Sealed + 'static {
    #[doc(alias = "phosh_wall_clock_get_clock")]
    #[doc(alias = "get_clock")]
    fn clock(&self, time_only: bool) -> glib::GString {
        unsafe {
            from_glib_none(ffi::phosh_wall_clock_get_clock(self.as_ref().to_glib_none().0, time_only.into_glib()))
        }
    }

    #[doc(alias = "phosh_wall_clock_local_date")]
    fn local_date(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::phosh_wall_clock_local_date(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_wall_clock_set_default")]
    fn set_default(&self) {
        unsafe {
            ffi::phosh_wall_clock_set_default(self.as_ref().to_glib_none().0);
        }
    }

    //#[doc(alias = "phosh_wall_clock_string_for_datetime")]
    //fn string_for_datetime(&self, datetime: /*Ignored*/&glib::DateTime, clock_format: /*Ignored*/gdesktop_enums::ClockFormat, show_full_date: bool) -> glib::GString {
    //    unsafe { TODO: call ffi:phosh_wall_clock_string_for_datetime() }
    //}

    #[doc(alias = "date-time")]
    fn date_time(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "date-time")
    }

    fn time(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "time")
    }

    #[doc(alias = "date-time")]
    fn connect_date_time_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_date_time_trampoline<P: IsA<WallClock>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshWallClock, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WallClock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::date-time\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_date_time_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "time")]
    fn connect_time_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_time_trampoline<P: IsA<WallClock>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshWallClock, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WallClock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::time\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_time_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<WallClock>> WallClockExt for O {}
