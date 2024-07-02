#[derive(Debug)]
struct Person {name: String, age: i32}

fn main() {
    let mut p1 = Person {name: "zachary".to_string(), age: 27};
    let n = std::mem::replace(&mut p1.name, "foo".to_string());
    let a = std::mem::replace(&mut p1.age, 28);
    dbg!(p1);
}
