// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "PhoshLockscreenPage")]
pub enum LockscreenPage {
    #[doc(alias = "PHOSH_LOCKSCREEN_PAGE_INFO")]
    Info,
    #[doc(alias = "PHOSH_LOCKSCREEN_PAGE_EXTRA")]
    Extra,
    #[doc(alias = "PHOSH_LOCKSCREEN_PAGE_UNLOCK")]
    Unlock,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for LockscreenPage {
    type GlibType = ffi::PhoshLockscreenPage;

    #[inline]
fn into_glib(self) -> ffi::PhoshLockscreenPage {
match self {
            Self::Info => ffi::PHOSH_LOCKSCREEN_PAGE_INFO,
            Self::Extra => ffi::PHOSH_LOCKSCREEN_PAGE_EXTRA,
            Self::Unlock => ffi::PHOSH_LOCKSCREEN_PAGE_UNLOCK,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::PhoshLockscreenPage> for LockscreenPage {
    #[inline]
unsafe fn from_glib(value: ffi::PhoshLockscreenPage) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::PHOSH_LOCKSCREEN_PAGE_INFO => Self::Info,
            ffi::PHOSH_LOCKSCREEN_PAGE_EXTRA => Self::Extra,
            ffi::PHOSH_LOCKSCREEN_PAGE_UNLOCK => Self::Unlock,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for LockscreenPage {
                #[inline]
    #[doc(alias = "phosh_lockscreen_page_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::phosh_lockscreen_page_get_type()) }
                }
            }

impl glib::HasParamSpec for LockscreenPage {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for LockscreenPage {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for LockscreenPage {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for LockscreenPage {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<LockscreenPage> for glib::Value {
    #[inline]
    fn from(v: LockscreenPage) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
