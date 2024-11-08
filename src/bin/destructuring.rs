fn one() {
    let t = &(String::from("hello"), 1);

    let (ref a, b) = *t;

    dbg!(t);
}

fn two() {
    let t = (String::from("hello"), 1);

    let (ref a, b) = t;
    //equivalent to let a = &t.0
    // ref is how we ask for borrows out of a destructure expression instead of non-borrowed values


    dbg!(t);
}

fn main() {
}

