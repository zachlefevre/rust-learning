use std::{error::Error, future::Future, pin::Pin};

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



trait Handler {
    // we let the user choose their output type, although it *must* be a Future of this particuar shape
    type Future: Future<Output = Result<HttpResponse, Box<dyn Error>>>;

    fn call(&mut self, req: HttpRequest) -> Self::Future;
}

impl Server {
    async fn run<F>(self, mut handler: F) -> Result<(), Box<dyn Error>> where
        F: Handler
    {
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let (read, write) = io::split(socket);
            let request = make_request(read);
            let response = handler.call(request).await;
            match response {
                Ok(http_response) => write_response(http_response, write), // the compiler is smart enough to see write is used exclusively in these two arms and therefore is not moved in the *other* arm.
                Err(err) => write_error(err, write),
            }
        }
    }
}


struct HttpHandler;
impl Handler for HttpHandler {
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Box<dyn Error>>>>>;

    fn call(&mut self, req: HttpRequest) -> Self::Future {
        Box::pin(async move
                 {
                     if req.path == "/" {
                         Ok(HttpResponse)
                     } else {
                         Ok(HttpResponse)
                     }
                 }
        )
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Server {
        addr: "127.0.0.1:8888".into()
    };

    server.run(HttpHandler).await
}
