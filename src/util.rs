use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};

pub fn select_bind_addr<T: ToSocketAddrs>(dest: T) -> Result<SocketAddr, Box<dyn Error>> {
    let bind_addr4 = SocketAddr::from(([0, 0, 0, 0], 0));
    let bind_addr6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 0));

    let bind_addr = match dest.to_socket_addrs()?.next() {
        Some(SocketAddr::V4(_)) => bind_addr4,
        Some(SocketAddr::V6(_)) => bind_addr6,
        None => bind_addr4,
    };

    Ok(bind_addr)
}
