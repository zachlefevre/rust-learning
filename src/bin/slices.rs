fn main() {
    let array = [1, 2, 3];
    dbg!(array[..].len());
    dbg!(array[1..].len());
    dbg!(array[1..1].len());
    dbg!(array[1..=1].len());
}
