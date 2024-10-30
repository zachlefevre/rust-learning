use std::net::ToSocketAddrs;

fn main() {
    dbg!("google.com:9080".to_socket_addrs().unwrap().collect::<Vec<_>>());
}
