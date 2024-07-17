// Rust has the same typeclass coherence properties of Haskell and so we need to do newtype wrappers, like Haskell, if we want multiple ways to implement typeclasses against a single type.

trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

#[derive(Debug)]
struct Sum(i32);

#[derive(Debug)]
struct Product(i32);

impl Monoid for Sum {
    fn empty() -> Self {
        Sum(0)
    }
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }

}

impl Monoid for Product {
    fn empty() -> Self {
        Self(1)
    }
    fn combine(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }

}




fn main() {
    dbg!(Sum(1).combine(Sum::empty()));
    dbg!(Product(1).combine(Product::empty()));

}
