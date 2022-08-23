use crate::Cond;

#[track_caller]
pub const fn assert_none<const LEN: usize>(cfgs: [Cond; LEN]) {
    if Cond::enabled_count(&cfgs) != 0 {
        Cond::panic_with_enabled("\nthese features must be disabled:\n", cfgs)
    }
}

/// Asserts that none of the passed-in features are enabled.
///
/// # Example
///
/// ```
/// assert_cfg::none!{
///     feature = "foo",
///     feature = "bar",
///     feature = "qux",
/// }
///
/// ```
///
/// When the `"foo"` and `"bar"` features are enabled,
/// the above code produces this compile-time error:
/// ```text
/// error[E0080]: evaluation of constant value failed
///  --> src/assert_none.rs:22:1
///   |
/// 4 | / assert_cfg::none!{
/// 5 | |     feature = "foo",
/// 6 | |     feature = "bar",
/// 7 | |     feature = "qux",
/// 8 | | }
///   | |_^ the evaluated program panicked at '
/// these features must be disabled:
/// - `feature = "foo"`
/// - `feature = "bar"`
/// ', src/assert_none.rs:4:1
///
/// ```
#[macro_export]
macro_rules! none {
    (
        $( $ident:ident $( = $feature:expr)? ),*
        $(,)?
    ) => {
        $crate::__priv_call_cfg_fn!{
            $crate::__::assert_none,
            $(( $ident $( = $feature)? ))*
        }
    }
}
