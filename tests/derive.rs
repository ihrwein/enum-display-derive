use enum_display_derive::Display;

#[test]
fn test_derive() {
    #[derive(Display)]
    enum FooBar {
        Foo,
        Bar,
        FooBar(),
        Number(i32),
    }

    assert_eq!(String::from("Foo"), format!("{}", FooBar::Foo));
    assert_eq!(String::from("Bar"), format!("{}", FooBar::Bar));
    assert_eq!(String::from("FooBar()"), format!("{}", FooBar::FooBar()));
    assert_eq!(String::from("42"), format!("{}", FooBar::Number(42)));
}
