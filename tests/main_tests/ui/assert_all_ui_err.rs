
assert_cfg::all!{}

assert_cfg::all!{
    feature = "bar",
}

assert_cfg::all!{
    feature = "foo",
    feature = "baz",
}

assert_cfg::all!{
    feature = "foo",
    feature = "bar",
    feature = "qux",
}

assert_cfg::all!{
    feature = "qux",
}


fn main(){}