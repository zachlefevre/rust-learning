struct Fib {
    prev: i32,
    next: i32
}

impl Iterator for Fib {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.next;
        self.next = self.prev + self.next;
        self.prev = temp;
        Some(self.prev)
    }
}


fn main() {
    let f = Fib {
        prev: 0, next: 1
    };

    for (index, fib) in f.enumerate() {
        println!("{index}: {fib}")
    };
}
