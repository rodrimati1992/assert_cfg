assert_cfg::exactly_one!{
    feature = "std",
    feature = "no_std",
}

assert_cfg::exactly_one!{
    feature = "foo",
    feature = "bar",
}

assert_cfg::exactly_one!{
    feature = "foo",
    feature = "bar",
    feature = "qux",
}



fn main(){}