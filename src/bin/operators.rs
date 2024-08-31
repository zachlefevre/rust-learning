struct Foo;

impl PartialEq for Foo {
  fn eq(&self, _: &Foo) -> bool { todo!() }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { todo!() }
}

fn main() {
    
}
