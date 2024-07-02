use std::error::Error;

use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async {
            let (mut read, mut write) = tokio::io::split(socket);
            let mut buf = [0; 5];
            while let Ok(n) = read.read(&mut buf).await {
                match n {
                    0 => break,
                    _ => {
                        println!("{:?} - {}", buf, n);
                        write.write(&buf[..n]).await.unwrap()
                    }
                };
            }
        });
    }
}
