assert_cfg::none!{
    feature = "foo",
    feature = "bar",
    feature = "qux",
}

assert_cfg::none!{
    feature = "hello",
    feature = "world",
}

assert_cfg::none!{
    feature = "foo",
    feature = "world",
}


fn main(){}