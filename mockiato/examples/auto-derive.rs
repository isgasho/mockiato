#![feature(plugin)]
#![plugin(mockiato_codegen)]

use std::fmt::{Debug, Display};

#[mockable]
trait Greeter: Debug {
    fn greet(&self, name: &Display) -> String;
}

fn main() {
    // TODO: this example needs to be updated
    // as soon as methods mocking is implemented.
    let greeter = GreeterMock {};

    println!("{:?}", greeter);
}
