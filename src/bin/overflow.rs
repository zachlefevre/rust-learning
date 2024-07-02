#[allow(arithmetic_overflow)]
fn main() {
    let mut number: i32 = 2147483647;
    number = number + 1;
    dbg!(number);
}
