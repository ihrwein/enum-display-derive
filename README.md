# Derive Display trait for enums

This crate can derive a `Display` implementation for very simple enums,
like the following one:

```rust
#[macro_use]
extern crate enum_display_derive;

use std::fmt::Display;

#[derive(Display)]
enum FizzBuzz {
   Fizz,
   Buzz,
   FizzBuzz,
   Number(u64),
}

fn fb(i: u64) {
   match (i % 3, i % 5) {
       (0, 0) => FizzBuzz::FizzBuzz,
       (0, _) => FizzBuzz::Fizz,
       (_, 0) => FizzBuzz::Buzz,
       (_, _) => FizzBuzz::Number(i),
   }
}

fn main() {
   for i in 0..100 {
       println!("{}", fb(i));
   }
}
```