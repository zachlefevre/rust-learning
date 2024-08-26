fn main() {
    let items = ["hey", "there", "person"].map(String::from);
    for (k, v) in items.iter().enumerate() {
        dbg!(k, &*v);
    }

    for (k, v) in items.iter().zip(items.iter()) {
        dbg!(k, v);
    }
}
