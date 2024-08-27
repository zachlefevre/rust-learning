fn t<T: Iterator<Item=f32>>(t: T) -> f32 {
    t.take(10).map(|item| item.sin()).sum()
}

struct Foo;
impl Iterator for Foo {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1.1)
    }
}

fn main() {
    let items = ["hey", "there", "person"].map(String::from);
    for (k, v) in items.iter().enumerate() {
        dbg!(k, &*v);
    }

    for (k, v) in items.iter().zip(items.iter()) {
        dbg!(k, v);
    }
    dbg!(t(Foo));
}
