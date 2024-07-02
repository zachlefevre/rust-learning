fn main() {
    struct Finally;
    impl Drop for Finally {
        fn drop(&mut self) {
            println!("doing thing in the finally block")
        }
    }
    let _end = Finally;
    println!("doing something at the end");
}
