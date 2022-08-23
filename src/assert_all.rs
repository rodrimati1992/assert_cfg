use crate::Cond;

#[track_caller]
pub const fn assert_all<const LEN: usize>(cfgs: [Cond; LEN]) {
    if Cond::enabled_count(&cfgs) != LEN {
        Cond::panic_with_disabled(
            "\ntoo few features are enabled, these need to be enabled:\n",
            cfgs,
        )
    }
}

/// Asserts that all of the passed-in features are enabled.
///
/// # Example
///
/// This example demonstrates the error message when not enough features are enabled.
///
/// ```compile_fail
/// assert_cfg::all!{
///     feature = "foo",
///     feature = "bar",
///     feature = "qux",
/// }
/// ```
///
/// When only the `"foo"` feature is enabled,
/// the above code produces this compile-time error:
/// ```text
/// error[E0080]: evaluation of constant value failed
///  --> src/assert_all.rs:20:1
///   |
/// 4 | / assert_cfg::all!{
/// 5 | |     feature = "foo",
/// 6 | |     feature = "bar",
/// 7 | |     feature = "qux",
/// 8 | | }
///   | |_^ the evaluated program panicked at '
/// too few features are enabled, these need to be enabled:
/// - `feature = "qux"`
///
/// ```
#[macro_export]
macro_rules! all {
    (
        $( $ident:ident $( = $feature:expr)? ),*
        $(,)?
    ) => {
        $crate::__priv_call_cfg_fn!{
            $crate::__::assert_all,
            $(( $ident $( = $feature)? ))*
        }
    }
}
