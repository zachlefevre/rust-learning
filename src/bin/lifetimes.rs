#[derive(Debug)]
struct Person {
    name: &'static str
}


#[derive(Debug)]
struct Person2<'a> {
    name: &'a str
}

fn do_thing(i: &mut i32) {
    *i += 18
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


    let mut t = 12;
    do_thing(&mut t);
    dbg!(t);


    let op = Some("hey there person".to_string());

    if let Some(ref thing) = op {
        dbg!(thing);
    }

    dbg!(op);


    let slice = ["hey", "there"].map(String::from);

    let binding = "foo".to_string();
    let thing = slice.get(0).unwrap_or(&binding);
    thing;

}


fn some_func(i: &Option<String>) -> Option<()> {
    match *i {
        None => None,
        Some(ref v) => {
            let n = v;
            Some(())
        }
    }
}
