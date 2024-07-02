mod visitor;

trait Animal {
    fn speak(&self) -> String;
}
struct Wombat;
impl Animal for Wombat {
    fn speak(&self) -> String {
        "speaking... (translated from wombat)".to_string()
    }
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> String {
        "bark".to_string()
    }
}

fn dothing(t: &dyn Animal) {
    dbg!(t.speak());
}

fn main() {
    let n = 23;
    let animal: &dyn Animal = if n == 22 {&Wombat} else {&Dog};
    dothing(animal);
}
