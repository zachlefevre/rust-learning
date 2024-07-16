fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(22);
    dbg!(v.capacity());
    v.resize(10, 0);
    dbg!(v.capacity());
}
