// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../..
// from ../../gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gobject_sys as gobject;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, time_t, off_t, intptr_t, uintptr_t, FILE};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Records
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
    // PhoshShell
    //=========================================================================
    pub fn phosh_shell_get_type() -> GType;
    pub fn phosh_shell_get_default() -> *mut PhoshShell;

}
