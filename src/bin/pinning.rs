use std::{marker::PhantomPinned, pin::Pin};
struct Foo { _marker: PhantomPinned }

impl Foo {
    fn new() -> Self {
        Foo { _marker: PhantomPinned }
    }
}

fn thing<T>(t: Pin<T>) -> Pin<T> {
    t
}

fn main() {
    unsafe {
        let t: Pin<&Foo> = thing(Pin::new_unchecked(&Foo::new()));
    }
}
