#[derive(Debug)]
struct Person {
    name: &'static str
}


#[derive(Debug)]
struct Person2<'a> {
    name: &'a str
}

fn main() {
    dbg!(Person2 {
        name: "hey there"
    });
}
