use const_panic::PanicVal;

pub struct CfgPred {
    pub enabled: bool,
    /// A string with roughly this format:
    /// ```text
    /// - `feature = "foo"`\n
    /// ```
    pub list_str: &'static str,
}

macro_rules! for_range {
    ($pat:pat in $range:expr => $($code:tt)*) => {
        let core::ops::Range{mut start, end} = $range;

        while start < end {
            let $pat = start;

            $($code)*

            start+=1;
        }
    };
}

pub const fn assert_only_one<const LEN: usize>(cfgs: [CfgPred; LEN]) {
    let mut enabled_cfgs = 0usize;
    for_range! {i in 0..LEN =>
        enabled_cfgs += cfgs[i].enabled as usize;
    }

    match enabled_cfgs {
        0 => {
            let mut cfg_list = [PanicVal::EMPTY; LEN];

            for_range! {i in 0..LEN =>
                cfg_list[i] = PanicVal::write_str(cfgs[i].list_str);
            }

            let msg = "\nno features were enabled, expected exactly one of these to be enabled:\n";
            const_panic::concat_panic(&[&[PanicVal::write_str(msg)], &cfg_list]);
        }
        1 => (),
        _many => {
            let mut cfg_list = [PanicVal::EMPTY; LEN];

            for_range! {i in 0..LEN =>
                let cfg = &cfgs[i];
                if cfg.enabled {
                    cfg_list[i] = PanicVal::write_str(cfg.list_str);
                }
            }

            let msg = "\ntoo many features were enabled (exactly one must be enabled):\n";
            const_panic::concat_panic(&[&[PanicVal::write_str(msg)], &cfg_list]);
        }
    }
}
