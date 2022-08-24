
assert_cfg::all!{}

assert_cfg::all!{
    feature = "bar",
}

assert_cfg::all!{
    feature = "foo",
    feature = "baz",
}

assert_cfg::all!{
    all(),
    feature = "bar",
}

assert_cfg::all!{
    feature = "bar",
    any(feature = "baz"),
}

assert_cfg::all!{
    feature = "bar",
    not(feature = "hello"),
}

