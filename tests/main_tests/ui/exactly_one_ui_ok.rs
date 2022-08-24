assert_cfg::exactly_one!{
    all(feature = "foo", feature = "bar"),
    feature = "qux",
}

assert_cfg::exactly_one!{
    any(feature = "foo", feature = "bar"),
    feature = "qux",
}

assert_cfg::exactly_one!{
    feature = "foo",
    not(feature = "bar"),
    feature = "qux",
}




