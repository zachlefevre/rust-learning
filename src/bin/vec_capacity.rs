fn main() {
    let mut v = Vec::with_capacity(5);
    dbg!(v.capacity());
    dbg!(&v);
    for i in 0..15 {
        dbg!(v.capacity());
        v.push(i);
    };
    dbg!(v.capacity());
    dbg!(v);
}
