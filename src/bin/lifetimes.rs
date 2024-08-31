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


    let t = ("hey there".to_string(), "hey there".to_string());
    let (ref a, ref b) = t;

    dbg!(t);

    let v = "hey there person".split(" ").map(String::from).collect::<Vec<_>>();
    dbg!(&v[0]);

}
