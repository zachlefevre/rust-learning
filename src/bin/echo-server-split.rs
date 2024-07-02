use tokio::{net::TcpListener, io::{self, AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8989").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async {
            let (mut read, mut write) = io::split(socket);
        });
    }
}
