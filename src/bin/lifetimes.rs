#[derive(Debug)]
struct Person {
    name: &'static str
}


#[derive(Debug)]
struct Person2<'a> {
    name: &'a str
}

fn main() {
    let array = "hey there person".split(" ").map(String::from).collect::<Vec<_>>();
    let m = &array[0]; // just borrow out of the vector of strings


    dbg!(Person2 {
        name: "hey there"
    });
}
