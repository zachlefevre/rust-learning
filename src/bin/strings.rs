fn foo() -> &'static i32 {
    &8
}


fn bar() -> &'static str {
    "hey there"
}


fn main() {
    let s = "hey there person";
    let s1 = String::from(s);
    let s2 = s.to_owned();
    dbg!(s);

    dbg!("hey this is a sentence".chars().filter(|s| !s.is_whitespace()).collect::<String>());

    dbg!(foo());
    dbg!(bar());

}
