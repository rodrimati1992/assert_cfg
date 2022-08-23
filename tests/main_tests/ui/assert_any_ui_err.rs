assert_cfg::any!{
    feature = "qux",
    feature = "bob",
    feature = "something",
}

assert_cfg::any!{}

assert_cfg::any!{
    feature = "bar",
}

assert_cfg::any!{
    feature = "foo",
    feature = "baz",
}


fn main(){}