fn take_slice<T: std::fmt::Debug>(s: &[T]) {
    dbg!(s);
}

fn take_str(s: &str) {
    dbg!(s);
}

fn main() {
    let array = [1, 2, 3];
    dbg!(array[..].len());
    dbg!(array[1..].len());
    dbg!(array[1..1].len());
    dbg!(array[1..=1].len());

    take_slice(&array);
    take_slice(&array[..]);

    let s = "hey there";

    let text = String::from(s);
    take_str(&text);

    take_str(&text[..]);

    take_str(s);

    dbg!(s.chars()
        .filter(|ch| ! ch.is_whitespace()).collect::<String>());

    dbg!(text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect::<String>());

    let b: i32 = *array.get(2).unwrap_or(&8);

}
