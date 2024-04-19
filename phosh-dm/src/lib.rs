#![cfg_attr(docsrs, feature(doc_cfg))]

/// Asserts that this is the main thread and either `gdk::init` or `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

mod auto;
pub mod subclass;

pub use auto::*;
pub use auto::traits::*;
