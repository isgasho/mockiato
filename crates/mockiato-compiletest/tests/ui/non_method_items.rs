use mockiato::mockable;

#[allow(dead_code)]
macro_rules! macro_in_trait {
    () => {};
}

#[mockable]
trait Foo {
    const BAR: usize;

    type Baz;

    macro_in_trait!();
}

fn main() {}
