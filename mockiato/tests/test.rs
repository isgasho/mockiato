#![feature(custom_attribute, plugin)]

use mockiato_codegen::mockable;
use std::fmt::{self, Display};

trait Debug {}

#[mockable]
trait Greeter: fmt::Debug {
    fn say_hello(&self, name: &dyn Display) -> String;
}

#[test]
fn test() {
    let _mock = GreeterMock::new();
}
