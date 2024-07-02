struct Command<F> where F: FnOnce() -> () {
    command: F
}

impl<T> Command<T> where T: FnOnce() -> () {
    fn myprint(&self, s: String) -> Command<T> {
        let f: T = || {println!("{}", s);};
        Self {
            command: f
        }
    }
}

fn main() {
}
