//! Static assertions for crate features with informative errors.
//!
//! The macros from this crate print which specific features are responsible for
//! the assertion failure.
//!
//! # Example
//!
//! ### Exactly one feature
//!
//! This example demonstrates the [`exactly_one`] macro,
//! which asserts that exactly one of the listed features is enabled.
//!
//!
//! ```compile_fail
//! assert_cfg::exactly_one!{
//!     feature = "foo",
//!     feature = "bar",
//!     feature = "qux",
//! }
//! ```
//!
//! When the `"foo"` and `"bar"` features are enabled,
//! the above code produces this compile-time error:
//! ```text
//! error[E0080]: evaluation of constant value failed
//!  --> src/lib.rs:15:1
//!   |
//! 4 | / assert_cfg::exactly_one!{
//! 5 | |     feature = "foo",
//! 6 | |     feature = "bar",
//! 7 | |     feature = "qux",
//! 8 | | }
//!   | |_^ the evaluated program panicked at '
//! too many features were enabled, only one of them is allowed:
//! - `feature = "foo"`
//! - `feature = "bar"`
//!
//! ```
//!
//!
//!
//!

#![no_std]

#[doc(hidden)]
pub mod __ {
    pub use core::{cfg, concat};

    pub use crate::{assert_exactly_one::assert_exactly_one, condition::Cond};
}

#[macro_use]
mod internal_macros;
mod assert_exactly_one;
mod condition;

use crate::condition::Cond;

#[cfg(all(not(feature = "__test"), test))]
compile_error!(r#"The "__test" feature must be enabled to run tests"#);
