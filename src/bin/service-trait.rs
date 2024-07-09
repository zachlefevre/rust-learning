use std::{error::Error, future::Future};

use tokio::{net::TcpListener, io::{self, AsyncRead, AsyncWrite}};

struct HttpRequest {
    path: String
}
struct HttpResponse;

struct Server {
    addr: String
}

fn make_request(_read: impl AsyncRead) -> HttpRequest {
    todo!()
}

fn write_response(_http_response: HttpResponse, _write: impl AsyncWrite) {
    ()
}

fn write_error(_error: Box<dyn Error>, _write: impl AsyncWrite) {
    ()
}



impl Server {
    async fn run<F, Fut>(self, handler: F) -> Result<(), Box<dyn Error>> where
        F: Fn(HttpRequest) -> Fut,
        Fut: Future<Output = Result<HttpResponse, Box<dyn Error>>>
    {
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let (read, write) = io::split(socket);
            let request = make_request(read);
            let response = handler(request).await;
            match response {
                Ok(http_response) => write_response(http_response, write), // the compiler is smart enough to see write is used exclusively in these two arms and therefore is not moved in the *other* arm.
                Err(err) => write_error(err, write),
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Server {
        addr: "127.0.0.1:8888".into()
    };

    async fn handler(req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
        if req.path == "/" {
            Ok(HttpResponse)
        } else {
            Ok(HttpResponse)
        }
    }

    server.run(handler).await
}
