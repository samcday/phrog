#![cfg_attr(docsrs, feature(doc_cfg))]

// TODO
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

pub use auto::*;
mod auto;
pub mod subclass;

pub mod prelude;

// FIXME: manually added. it should have been generated in high level crate tho.
#[doc(alias = "phosh_init")]
pub fn init() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::phosh_init();
    }
}
