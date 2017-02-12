#[macro_use]
extern crate enum_display_derive;

use std::fmt::Display;

#[derive(Display)]
enum FizzBuzz {
    Fizz,
    Buzz,
}

#[test]
fn test_derive() {
    let a = FizzBuzz::Fizz;
    println!("{}", a);
}