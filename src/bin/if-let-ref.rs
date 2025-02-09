fn main() {
    let string = "foo".to_string();
    let foo = &Some(&string);
    {
        if let Some(t) = *foo {
            dbg!("in top {}", t);
        }
    }
}
