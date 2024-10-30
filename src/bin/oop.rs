trait Show {
    fn show(&self);
}

struct Foo;
impl Show for Foo {
    fn show(&self) {
        
    }
}

struct Bar;
impl Show for Bar {
    fn show(&self) {
        
    }
}

fn with_show(s: &impl Show) {
    s.show()
}


fn main() {
    with_show(&Foo);

}
