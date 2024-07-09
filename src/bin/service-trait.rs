use std::error::Error;

use tokio::{net::TcpListener, io::{self, AsyncRead, AsyncWrite}};

struct HttpRequest;
struct HttpResponse;

struct Server {
    addr: String
}

fn make_request(_read: impl AsyncRead) -> HttpRequest {
    HttpRequest
}

fn write_response(_http_response: HttpResponse, _write: impl AsyncWrite) {
    ()
}

impl Server {
    async fn run<F>(self, handler: F) -> Result<(), Box<dyn Error>> where
        F: Fn(HttpRequest) -> HttpResponse,
    {
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let (read, write) = io::split(socket);
            let request = make_request(read);
            let response = handler(request);
            write_response(response, write);
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Server {
        addr: "127.0.0.1:8888".into()
    };

    server.run(|_req|  HttpResponse).await
}
