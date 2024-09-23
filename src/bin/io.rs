use std::io::{BufRead, BufWriter, Read};

struct Foo;
impl std::io::Read for Foo {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl std::io::Write for Foo {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl std::io::BufRead for Foo {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        todo!()
    }

    fn consume(&mut self, amt: usize) {
        todo!()
    }
}


fn foo(n: &dyn BufRead) {
    
}

fn main() {
    let mut file = std::fs::File::open("/home/zlef/dotfiles/init.el").unwrap();
    let mut buf: [u8; 10] = [0; 10];
    file.read(&mut buf).unwrap();

    let mut v = Vec::new();
    file.read_to_end(&mut v).unwrap();

    file.read_to_end(&mut v).unwrap();
    dbg!(String::from_utf8(v));


    let writer = BufWriter::new(file);

}
