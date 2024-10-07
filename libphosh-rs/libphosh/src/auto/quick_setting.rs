// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{StatusIcon};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "PhoshQuickSetting")]
    pub struct QuickSetting(Object<ffi::PhoshQuickSetting, ffi::PhoshQuickSettingClass>) @extends gtk::Button, gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        type_ => || ffi::phosh_quick_setting_get_type(),
    }
}

impl QuickSetting {
        pub const NONE: Option<&'static QuickSetting> = None;
    

    #[doc(alias = "phosh_quick_setting_new")]
    pub fn new() -> QuickSetting {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::phosh_quick_setting_new()).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`QuickSetting`] objects.
            ///
            /// This method returns an instance of [`QuickSettingBuilder`](crate::builders::QuickSettingBuilder) which can be used to create [`QuickSetting`] objects.
            pub fn builder() -> QuickSettingBuilder {
                QuickSettingBuilder::new()
            }
        

    #[doc(alias = "phosh_quick_setting_open_settings_panel")]
    pub fn open_settings_panel(panel: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::phosh_quick_setting_open_settings_panel(panel.to_glib_none().0);
        }
    }
}

impl Default for QuickSetting {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`QuickSetting`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct QuickSettingBuilder {
            builder: glib::object::ObjectBuilder<'static, QuickSetting>,
        }

        impl QuickSettingBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn active(self, active: bool) -> Self {
                            Self { builder: self.builder.property("active", active), }
                        }

                            pub fn has_status(self, has_status: bool) -> Self {
                            Self { builder: self.builder.property("has-status", has_status), }
                        }

                            pub fn present(self, present: bool) -> Self {
                            Self { builder: self.builder.property("present", present), }
                        }

                            #[cfg(feature = "gtk_v3_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_6")))]
    pub fn always_show_image(self, always_show_image: bool) -> Self {
                            Self { builder: self.builder.property("always-show-image", always_show_image), }
                        }

                            #[cfg(feature = "gtk_v2_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_6")))]
    pub fn image(self, image: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("image", image.clone().upcast()), }
                        }

                        //    #[cfg(feature = "gtk_v2_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_10")))]
    //pub fn image_position(self, image_position: /*Ignored*/gtk::PositionType) -> Self {
                        //    Self { builder: self.builder.property("image-position", image_position), }
                        //}

                            pub fn label(self, label: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("label", label.into()), }
                        }

                            //pub fn relief(self, relief: /*Ignored*/gtk::ReliefStyle) -> Self {
                        //    Self { builder: self.builder.property("relief", relief), }
                        //}

                            #[cfg_attr(feature = "v3_10", deprecated = "Since 3.10")]
    pub fn use_stock(self, use_stock: bool) -> Self {
                            Self { builder: self.builder.property("use-stock", use_stock), }
                        }

                            pub fn use_underline(self, use_underline: bool) -> Self {
                            Self { builder: self.builder.property("use-underline", use_underline), }
                        }

                            #[cfg(feature = "gtk_v2_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_4")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn xalign(self, xalign: f32) -> Self {
                            Self { builder: self.builder.property("xalign", xalign), }
                        }

                            #[cfg(feature = "gtk_v2_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_4")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn yalign(self, yalign: f32) -> Self {
                            Self { builder: self.builder.property("yalign", yalign), }
                        }

                            pub fn border_width(self, border_width: u32) -> Self {
                            Self { builder: self.builder.property("border-width", border_width), }
                        }

                            pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("child", child.clone().upcast()), }
                        }

                            //pub fn resize_mode(self, resize_mode: /*Ignored*/gtk::ResizeMode) -> Self {
                        //    Self { builder: self.builder.property("resize-mode", resize_mode), }
                        //}

                            pub fn app_paintable(self, app_paintable: bool) -> Self {
                            Self { builder: self.builder.property("app-paintable", app_paintable), }
                        }

                            pub fn can_default(self, can_default: bool) -> Self {
                            Self { builder: self.builder.property("can-default", can_default), }
                        }

                            pub fn can_focus(self, can_focus: bool) -> Self {
                            Self { builder: self.builder.property("can-focus", can_focus), }
                        }

                            #[cfg(feature = "gtk_v2_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_18")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn double_buffered(self, double_buffered: bool) -> Self {
                            Self { builder: self.builder.property("double-buffered", double_buffered), }
                        }

                            //pub fn events(self, events: /*Ignored*/gdk::EventMask) -> Self {
                        //    Self { builder: self.builder.property("events", events), }
                        //}

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn expand(self, expand: bool) -> Self {
                            Self { builder: self.builder.property("expand", expand), }
                        }

                            #[cfg(feature = "gtk_v3_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_20")))]
    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
                            Self { builder: self.builder.property("focus-on-click", focus_on_click), }
                        }

                        //    #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    //pub fn halign(self, halign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("halign", halign), }
                        //}

                            pub fn has_default(self, has_default: bool) -> Self {
                            Self { builder: self.builder.property("has-default", has_default), }
                        }

                            pub fn has_focus(self, has_focus: bool) -> Self {
                            Self { builder: self.builder.property("has-focus", has_focus), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
                            Self { builder: self.builder.property("has-tooltip", has_tooltip), }
                        }

                            pub fn height_request(self, height_request: i32) -> Self {
                            Self { builder: self.builder.property("height-request", height_request), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand(self, hexpand: bool) -> Self {
                            Self { builder: self.builder.property("hexpand", hexpand), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("hexpand-set", hexpand_set), }
                        }

                            pub fn is_focus(self, is_focus: bool) -> Self {
                            Self { builder: self.builder.property("is-focus", is_focus), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin(self, margin: i32) -> Self {
                            Self { builder: self.builder.property("margin", margin), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
                            Self { builder: self.builder.property("margin-bottom", margin_bottom), }
                        }

                            #[cfg(feature = "gtk_v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_end(self, margin_end: i32) -> Self {
                            Self { builder: self.builder.property("margin-end", margin_end), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_left(self, margin_left: i32) -> Self {
                            Self { builder: self.builder.property("margin-left", margin_left), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_right(self, margin_right: i32) -> Self {
                            Self { builder: self.builder.property("margin-right", margin_right), }
                        }

                            #[cfg(feature = "gtk_v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_start(self, margin_start: i32) -> Self {
                            Self { builder: self.builder.property("margin-start", margin_start), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin_top(self, margin_top: i32) -> Self {
                            Self { builder: self.builder.property("margin-top", margin_top), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn no_show_all(self, no_show_all: bool) -> Self {
                            Self { builder: self.builder.property("no-show-all", no_show_all), }
                        }

                            #[cfg(feature = "gtk_v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_8")))]
    pub fn opacity(self, opacity: f64) -> Self {
                            Self { builder: self.builder.property("opacity", opacity), }
                        }

                            pub fn parent(self, parent: &impl IsA<gtk::Container>) -> Self {
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

                            pub fn receives_default(self, receives_default: bool) -> Self {
                            Self { builder: self.builder.property("receives-default", receives_default), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            //pub fn style(self, style: &impl IsA</*Ignored*/gtk::Style>) -> Self {
                        //    Self { builder: self.builder.property("style", style.clone().upcast()), }
                        //}

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-markup", tooltip_markup.into()), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-text", tooltip_text.into()), }
                        }

                        //    #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    //pub fn valign(self, valign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("valign", valign), }
                        //}

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand(self, vexpand: bool) -> Self {
                            Self { builder: self.builder.property("vexpand", vexpand), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("vexpand-set", vexpand_set), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width_request(self, width_request: i32) -> Self {
                            Self { builder: self.builder.property("width-request", width_request), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`QuickSetting`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> QuickSetting {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::QuickSetting>> Sealed for T {}
}

pub trait QuickSettingExt: IsA<QuickSetting> + sealed::Sealed + 'static {
    #[doc(alias = "phosh_quick_setting_get_active")]
    #[doc(alias = "get_active")]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_quick_setting_get_active(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_quick_setting_get_has_status")]
    #[doc(alias = "get_has_status")]
    fn has_status(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_quick_setting_get_has_status(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_quick_setting_get_present")]
    #[doc(alias = "get_present")]
    fn is_present(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_quick_setting_get_present(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_quick_setting_get_status_icon")]
    #[doc(alias = "get_status_icon")]
    fn status_icon(&self) -> StatusIcon {
        unsafe {
            from_glib_none(ffi::phosh_quick_setting_get_status_icon(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_quick_setting_set_active")]
    fn set_active(&self, active: bool) {
        unsafe {
            ffi::phosh_quick_setting_set_active(self.as_ref().to_glib_none().0, active.into_glib());
        }
    }

    #[doc(alias = "phosh_quick_setting_set_has_status")]
    fn set_has_status(&self, has_status: bool) {
        unsafe {
            ffi::phosh_quick_setting_set_has_status(self.as_ref().to_glib_none().0, has_status.into_glib());
        }
    }

    #[doc(alias = "phosh_quick_setting_set_present")]
    fn set_present(&self, present: bool) {
        unsafe {
            ffi::phosh_quick_setting_set_present(self.as_ref().to_glib_none().0, present.into_glib());
        }
    }

    #[doc(alias = "phosh_quick_setting_set_status_icon")]
    fn set_status_icon(&self, widget: &impl IsA<StatusIcon>) {
        unsafe {
            ffi::phosh_quick_setting_set_status_icon(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "long-pressed")]
    fn connect_long_pressed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn long_pressed_trampoline<P: IsA<QuickSetting>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshQuickSetting, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(QuickSetting::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"long-pressed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(long_pressed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_long_pressed(&self) {
        self.emit_by_name::<()>("long-pressed", &[]);
    }

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P: IsA<QuickSetting>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshQuickSetting, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(QuickSetting::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_active_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "has-status")]
    fn connect_has_status_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_status_trampoline<P: IsA<QuickSetting>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshQuickSetting, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(QuickSetting::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-status\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_has_status_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "present")]
    fn connect_present_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_present_trampoline<P: IsA<QuickSetting>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshQuickSetting, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(QuickSetting::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::present\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_present_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "status-icon")]
    fn connect_status_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_status_icon_trampoline<P: IsA<QuickSetting>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshQuickSetting, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(QuickSetting::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::status-icon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_status_icon_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<QuickSetting>> QuickSettingExt for O {}
