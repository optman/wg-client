use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};

pub fn select_bind_addr<T: ToSocketAddrs>(dest: T) -> Result<&'static str, Box<dyn Error>> {
    let bind_addr = match dest.to_socket_addrs()?.next() {
        Some(SocketAddr::V4(_)) => "0.0.0.0:0",
        Some(SocketAddr::V6(_)) => "[::]:0",
        None => "0.0.0.0:0",
    };

    Ok(bind_addr)
}
