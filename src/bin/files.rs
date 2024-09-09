use std::{env, fs::File, io::Read};

fn main() {
    let arg = env::args().nth(1).expect("gimme an arg");
    let mut file = File::open(arg).expect("open file");
    let mut buf = [0; 10];
    while let Ok(size) = file.read(&mut buf) {
        if size == 0 {break};
        dbg!(String::from_utf8(buf.to_vec()));
    }

}
