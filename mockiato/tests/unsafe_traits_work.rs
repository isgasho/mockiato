use mockiato_codegen::mockable;

#[mockable]
unsafe trait Foo {
    unsafe fn bar(&self);
}

fn main() {
    let _mock = FooMock::new();
}
