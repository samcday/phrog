// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gobject_sys as gobject;
use gtk_sys as gtk;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, time_t, off_t, intptr_t, uintptr_t, FILE};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type PhoshLockscreenPage = c_int;
pub const PHOSH_LOCKSCREEN_PAGE_DEFAULT: PhoshLockscreenPage = 0;
pub const PHOSH_LOCKSCREEN_PAGE_UNLOCK: PhoshLockscreenPage = 1;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLayerSurfaceClass {
    pub parent_class: gtk::GtkWindowClass,
    pub configured: Option<unsafe extern "C" fn(*mut PhoshLayerSurface)>,
}

impl ::std::fmt::Debug for PhoshLayerSurfaceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLayerSurfaceClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("configured", &self.configured)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLockscreenClass {
    pub parent_class: PhoshLayerSurfaceClass,
}

impl ::std::fmt::Debug for PhoshLockscreenClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLockscreenClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshShellClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for PhoshShellClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshShellClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLayerSurface {
    pub parent_instance: gtk::GtkWindow,
}

impl ::std::fmt::Debug for PhoshLayerSurface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLayerSurface @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLockscreen {
    pub parent_instance: PhoshLayerSurface,
}

impl ::std::fmt::Debug for PhoshLockscreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLockscreen @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct PhoshShell {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PhoshShell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshShell @ {self:p}"))
         .finish()
    }
}

#[link(name = "phosh")]
extern "C" {

    //=========================================================================
    // PhoshLayerSurface
    //=========================================================================
    pub fn phosh_layer_surface_get_type() -> GType;

    //=========================================================================
    // PhoshLockscreen
    //=========================================================================
    pub fn phosh_lockscreen_get_type() -> GType;
    pub fn phosh_lockscreen_get_page(self_: *mut PhoshLockscreen) -> PhoshLockscreenPage;
    pub fn phosh_lockscreen_set_page(self_: *mut PhoshLockscreen, page: PhoshLockscreenPage);

    //=========================================================================
    // PhoshShell
    //=========================================================================
    pub fn phosh_shell_get_type() -> GType;
    pub fn phosh_shell_get_default() -> *mut PhoshShell;

}
