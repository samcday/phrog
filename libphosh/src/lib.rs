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
#[allow(unused_imports)]
mod auto;
pub mod subclass;

pub mod prelude;
