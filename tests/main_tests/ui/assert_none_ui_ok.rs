assert_cfg::none!{
    feature = "hello",
    feature = "world",
}

assert_cfg::none!{
    feature = "hello",
    all(feature = "foo", feature = "world")
}

assert_cfg::none!{
    feature = "hello",
    any(not(feature = "foo"), feature = "world")
}
