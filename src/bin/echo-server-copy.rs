use std::{error::Error, io::{self}, net::TcpListener};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    loop {
        println!("listening");
        let (socket, addr) = listener.accept()?;
        dbg!(addr);
        let mut read = socket;
        let mut write = read.try_clone()?;

        io::copy(&mut read, &mut write)?;
    }
}
