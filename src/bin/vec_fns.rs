use std::io::{BufReader, self, BufRead};

fn main() {
    let mut file = std::fs::File::open("vec_fns.rs").unwrap();

    let mut bufreader = BufReader::new(file);


    let mut s = String::new();
    let mut v = Vec::new();

    bufreader.read_until(b'B', &mut v);

//    let _ = io::copy(&mut file, &mut v);
    dbg!(String::from_utf8(v));

}
