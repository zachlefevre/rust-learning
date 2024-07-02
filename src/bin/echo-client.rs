use std::{error::Error, time::Duration};

use tokio::{net::TcpStream, io::{AsyncWriteExt, self, AsyncReadExt}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8888").await?;
    let (mut read, mut write) = io::split(socket);
    tokio::spawn(async move {
        write.write_all(b"aaaa").await;
        write.write_all(b"bbbb").await;
    });
    dbg!("writing");
    let mut buf = [0; 10];
    while let Ok(n) = read.read(&mut buf).await {
        if n == 0 {
            break
        }
        println!("got: {:?}", buf);
    }

    Ok(())
}
