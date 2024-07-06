use std::error::Error;

use tokio::{net::TcpStream, io::{self, AsyncWriteExt, AsyncReadExt}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connect = TcpStream::connect("127.0.0.1:8888").await?;
    let (mut read, mut write) = io::split(connect);
    tokio::spawn(async move {
        write.write(b"hey there").await;
        write.write(b"person").await;
    });

    let mut buf: [u8; 5] = Default::default();

    while let Ok(b) = read.read(&mut buf).await {
        if b == 0 {
            break
        } else {
            println!("buf: {:?}", buf)
        }
    }

    Ok(())
}
