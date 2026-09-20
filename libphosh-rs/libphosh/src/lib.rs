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
pub use ffi;
// Preserve the generated builder API and formatting.
#[allow(
    unused_imports,
    clippy::empty_line_after_outer_attr,
    clippy::wrong_self_convention
)]
#[rustfmt::skip]
mod auto;
pub mod subclass;

pub mod prelude;
