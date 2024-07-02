fn main() {
    dbg!(vec![1, 2, 3, 4, 5, 6, 7, 8].chunks(chunk_size));
    dbg!(vec![1, 2, 3, 4, 5, 6, 7, 8].chunks(3).collect::<Vec<_>>());
    dbg!(vec![1, 2, 3, 4, 5, 6, 7, 8].chunks_exact(3).collect::<Vec<_>>());
}
