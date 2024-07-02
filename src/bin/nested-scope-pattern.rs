


fn main() {
    let a: &str = "hey".into();
    let b = Box::new(88);
    let mut closure = {
        let mut b = b;
        move || { *b = *b + a.len()}
    };
    closure();
}
