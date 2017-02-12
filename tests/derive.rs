#[macro_use]
extern crate enum_display_derive;

use std::fmt::Display;

#[test]
fn test_derive() {
    #[derive(Display)]
    enum FizzBuzz {
        Fizz,
        Buzz,
        Bar(),
        Number(i32),
    }

    assert_eq!(String::from("Fizz"), format!("{}", FizzBuzz::Fizz));
    assert_eq!(String::from("Buzz"), format!("{}", FizzBuzz::Buzz));
    assert_eq!(String::from("Bar()"), format!("{}", FizzBuzz::Bar()));
    assert_eq!(String::from("42"), format!("{}", FizzBuzz::Number(42)));
}
