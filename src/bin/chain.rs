fn main() {
    dbg!(vec![1, 2, 3, 4, 5].iter().chain(Some(8).iter()));
}
