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


trait NoDynamic {
    fn no_dyn(self) where Self: Sized { }
    fn yes_dyn(&self) { }

}


fn foo<T>(t: &dyn NoDynamic) {
//    t.no_dyn()
    t.yes_dyn()
}

fn dothing(t: &dyn Animal) {
    dbg!(t.speak());
}

fn main() {
    let n = 23;
    let animal: &dyn Animal = if n == 22 {&Wombat} else {&Dog};
    dothing(animal);

    

}
