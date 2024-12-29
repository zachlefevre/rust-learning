use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let input = [];

    let mut reader = BufReader::new(input);
    


    let mut buf = String::new();

    while let _ = reader.read_line(&mut buf).unwrap() {
        dbg!(&buf);
    }
}
