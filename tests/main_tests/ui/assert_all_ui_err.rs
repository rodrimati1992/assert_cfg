assert_cfg::all!{
    feature = "foo",
    feature = "bar",
    feature = "qux",
}

assert_cfg::all!{
    feature = "qux",
}

assert_cfg::all!{
    feature = "bar",
    all(feature = "qux"),
}


fn main(){}