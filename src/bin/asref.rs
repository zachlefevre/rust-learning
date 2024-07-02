fn main() {
    let a = Some("hey".to_string());
    let b = a.as_ref();
    dbg!(a);
}
