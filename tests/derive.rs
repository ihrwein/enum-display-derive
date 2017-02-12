#[macro_use]
extern crate enum_display_derive;

use std::fmt::Display;

#[test]
fn test_derive() {
    #[derive(Display)]
    enum FizzBuzz {
        Fizz,
        Buzz,
    }

    assert_eq!(String::from("Fizz"), format!("{}", FizzBuzz::Fizz));
    assert_eq!(String::from("Buzz"), format!("{}", FizzBuzz::Buzz));
}