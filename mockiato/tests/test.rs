#![feature(custom_attribute, plugin)]
#![plugin(mockiato_codegen)]

use std::fmt::{self, Display};

trait Debug {}

#[mockable(derive(Debug))]
trait Greeter<D>
where
    D: Display,
{
    fn say_hello(&self, name: D) -> String;
}

#[test]
fn test() {
    let _mock = GreeterMock;
}
