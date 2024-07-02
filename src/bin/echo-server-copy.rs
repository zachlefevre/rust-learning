use std::error::Error;

use tokio::{net::TcpListener, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8888").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async {
            let (mut reader, mut writer) = io::split(socket);
            io::copy(&mut reader, &mut writer).await.unwrap();
        });
    }
}
