use std::error::Error;

use tokio::{net::TcpListener, io::{self, AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async {
            let (mut reader, mut writer) = io::split(socket);

            let mut buf = [0; 10];

            while let Ok(n) = reader.read(&mut buf).await {
                if n == 0 {
                    break;
                } else {
                    println!("I did hereby read {:?}", buf);
                    writer.write(&buf[..n]).await.unwrap();
                }
            }
        });
    }
}
