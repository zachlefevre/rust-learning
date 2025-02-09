use tokio::{net::{TcpListener, TcpSocket, TcpStream}, task};
use std::future::Future;


struct Server {
    addr: String
}

struct Request;
struct Response;


impl Server {
    async fn run<F, Fut>(self, handler: F) -> Result<(), std::io::Error>
        where F: Fn(Request) -> Fut,
              Fut: Future<Output = Result<Response, std::io::Error>>
    {
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            let request = make_request(&socket);
            let response = handler(request).await;
            match response {
                Ok(response) =>
                    respond(response, &socket),
                Err(e) => respond_error(e, &socket)
            }

        }
    }
}

fn respond_error(response: std::io::Error, socket: &TcpStream) {

}
fn respond(response: Response, socket: &TcpStream) {
    
}


fn make_request(socket: &TcpStream) -> Request {
    todo!()
}



fn main() {
    
}
