use ipnet::IpNet;
use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};
use std::process::Command;

pub fn select_bind_addr<T: ToSocketAddrs>(dest: T) -> Result<&'static str, Box<dyn Error>> {
    let bind_addr = match dest.to_socket_addrs()?.next() {
        Some(SocketAddr::V4(_)) => "0.0.0.0:0",
        Some(SocketAddr::V6(_)) => "[::]:0",
        None => "0.0.0.0:0",
    };

    Ok(bind_addr)
}

pub fn add_addr(addr: &IpNet, dev: &str) -> Result<(), Box<dyn Error>> {
    let mut c = Command::new("ip");
    if let IpNet::V6(_) = addr {
        c.arg("-6");
    };

    if !c
        .arg("addr")
        .arg("add")
        .arg(addr.to_string())
        .arg("dev")
        .arg(dev)
        .status()
        .map_or(false, |c| c.success())
    {
        Err("add addr fail")?
    }

    Ok(())
}

pub fn add_route(
    addr: &IpNet,
    dev: &str,
    table: &Option<String>,
    metric: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut c = Command::new("ip");
    if let IpNet::V6(_) = addr {
        c.arg("-6");
    };

    c.arg("route")
        .arg("add")
        .arg(addr.to_string())
        .arg("dev")
        .arg(dev);

    if let Some(table) = table {
        c.arg("table");
        c.arg(table);
    }

    if let Some(metric) = metric {
        c.arg("metric");
        c.arg(metric);
    }

    if c.status().map_or(false, |c| c.success()) {
        return Ok(());
    }

    Err("add route fail")?
}
