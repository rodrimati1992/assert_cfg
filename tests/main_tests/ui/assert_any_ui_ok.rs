assert_cfg::any!{
    feature = "bar",
}

assert_cfg::any!{
    feature = "foo",
    feature = "baz",
}

assert_cfg::any!{
    all(feature = "foo", feature = "bar"),
    feature = "baz",
}

assert_cfg::any!{
    any(feature = "foo", feature = "qux"),
    feature = "hello",
}

assert_cfg::any!{
    feature = "hello",
    not(feature = "world"),
}