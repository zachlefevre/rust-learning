fn square<T, O>(t: T) -> O
where T: std::ops::Mul<Output = O> + Copy {
    t * t
}

fn main() {
    dbg!(square(14));
}
