use std::{error::Error};

use tokio::net::{TcpListener, TcpStream};

struct HttpRequest;
struct HttpResponse;

struct Server {
    addr: String
}

fn make_request(socket: &mut TcpStream) -> HttpRequest {
    HttpRequest
}

impl Server {
    async fn run<F>(self, handler: F) -> Result<(), Box<dyn Error>> where
        F: Fn(HttpRequest) -> HttpResponse {
        let listener = TcpListener::bind(self.addr).await?;
        
        loop {
            let (mut socket, _) = listener.accept().await?;
            let request = make_request(&mut socket);
            let response = handler(request);
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Server {
        addr: "127.0.0.1:8888".into()
    };

    server.run(|req| HttpResponse).await
}