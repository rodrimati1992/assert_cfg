#![no_std]

#[doc(hidden)]
pub mod __ {
    pub use core::{cfg, concat};

    pub use crate::assert_only_one_stuff::{assert_only_one, CfgPred};
}

mod assert_only_one_stuff;

/// Asserts that exactly one of the passed-in features is enabled.
#[macro_export]
macro_rules! assert_one_cfg {
    (
        $( $ident:ident $( = $feature:expr)? ),*
        $(,)?
    ) => {
        const _: () = {
            use $crate::__::{CfgPred, concat, cfg};

            $crate::__::assert_only_one([
                $(
                    CfgPred {
                        enabled: cfg!($ident $( = $feature)?),
                        list_str: concat!("- `", stringify!($ident $( = $feature)?),"`\n"),
                    },
                )*
            ])
        };
    }
}
