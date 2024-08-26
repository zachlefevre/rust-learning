fn main() {
    let s = "hey there person";
    let s1 = String::from(s);
    let s2 = s.to_owned();
    dbg!(s);
}
