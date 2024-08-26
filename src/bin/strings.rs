fn main() {
    let s = "hey there person";
    let s1 = String::from(s);
    let s2 = s.to_owned();
    dbg!(s);

    dbg!("hey this is a sentence".to_string().chars().filter(|s| !s.is_whitespace()).collect::<String>());

}
