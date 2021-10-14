use boringtun::crypto::{X25519PublicKey, X25519SecretKey};
use boringtun::noise::Tunn;
use nix::sys::socket::{setsockopt, sockopt};
use slog::{info, o, Drain, Logger};
use std::error::Error;
use std::os::unix::io::AsRawFd;
use std::panic;
use std::sync::Arc;
use tokio::net::UdpSocket;

mod client;
use client::Client;
mod cli;
mod config;
mod util;
use util::select_bind_addr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        std::process::exit(1);
    }));

    let config = cli::parse()?;
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(plain)
        .build()
        .filter_level(config.log_level);
    let drain = std::sync::Mutex::new(drain).fuse();
    let logger = Logger::root(drain, o!());

    let mut dev_config = tun::configure();
    dev_config.mtu(config.mtu).name(&config.ifname).up();
    let dev = tun::r#async::create_as_async(&dev_config)?;

    let listen_addr = select_bind_addr(&config.remote_addr)?;
    let socket = UdpSocket::bind(listen_addr).await?;
    if let Some(fwmark) = config.fwmark {
        setsockopt(socket.as_raw_fd(), sockopt::Mark, &fwmark)?;
    }

    let mut tun = Tunn::new(
        Arc::new(config.local_private_key.parse::<X25519SecretKey>().unwrap()),
        Arc::new(config.remote_public_key.parse::<X25519PublicKey>().unwrap()),
        None,
        config.keepalive,
        1,
        None,
    )?;
    tun.set_logger(logger.new(o!()));

    info!(
        logger,
        "wg-client created tunnel to {}.", config.remote_addr
    );

    Client::new(config, dev, socket, tun, logger.new(o!()))
        .run()
        .await?;

    Ok(())
}
