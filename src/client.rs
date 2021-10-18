use crate::config::Config;
use boringtun::noise::{errors::WireGuardError, Tunn, TunnResult};
use slog::{debug, error, info, warn, Logger};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tun::r#async::AsyncDevice;

const BUF_SIZE: usize = 1600;

pub(crate) struct Client {
    dev: AsyncDevice,
    socket: UdpSocket,
    tun: Box<Tunn>,
    socket_dst_buf: [u8; BUF_SIZE],
    logger: Logger,
    config: Config,
    last_connect: Option<Instant>,
}

impl Client {
    pub fn new(
        config: Config,
        dev: AsyncDevice,
        socket: UdpSocket,
        tun: Box<Tunn>,
        logger: Logger,
    ) -> Self {
        Self {
            config: config,
            dev: dev,
            socket: socket,
            tun: tun,
            socket_dst_buf: [0; BUF_SIZE],
            logger: logger,
            last_connect: None,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.connect(self.config.remote_addr.clone()).await?;
        self.last_connect = Some(Instant::now());

        let mut dev_buf = [0; BUF_SIZE];
        let mut socket_src_buf = [0; BUF_SIZE];

        let mut timer = tokio::time::interval(Duration::from_millis(250));
        loop {
            tokio::select! {
                   _ = timer.tick() =>{
                       self.update_timers().await;
                       self.watchdog().await;
                   }
                   len = self.dev.read(&mut dev_buf) => {
                       match len{
                          Ok(len) => {
                              self.local_recv(&dev_buf[..len]).await;
                          },
                          Err(e) => {
                              error!(self.logger, "dev read fail {:?}", e);
                          }
                       }
                   }

                   len = self.socket.recv(&mut socket_src_buf) => {
                       match len{
                          Ok(len) => {
                              self.remote_recv(&socket_src_buf[..len]).await;
                          }
                          Err(e) => error!(self.logger, "socket read fail {:?}", e),
                       }
                   }
            }
        }
    }

    async fn update_timers(&mut self) {
        match self.tun.update_timers(&mut self.socket_dst_buf) {
            TunnResult::WriteToNetwork(pkt) => {
                let _ = self.socket.send(pkt).await;
            }
            TunnResult::Done => {}
            TunnResult::Err(WireGuardError::ConnectionExpired) => {
                self.reconnect().await;
            }
            r => warn!(self.logger, "update timer result {:?}", r),
        }
    }

    async fn local_recv(&mut self, buf: &[u8]) {
        match self.tun.encapsulate(buf, &mut self.socket_dst_buf) {
            TunnResult::WriteToNetwork(pkt) => {
                let _ = self.socket.send(pkt).await;
            }
            TunnResult::Done => {}
            r => warn!(self.logger, "encapsulate result {:?}", r),
        }
    }

    async fn remote_recv(&mut self, buf: &[u8]) {
        let mut len = buf.len();
        loop {
            match self
                .tun
                .decapsulate(None, &buf[..len], &mut self.socket_dst_buf)
            {
                TunnResult::WriteToNetwork(pkt) => {
                    let _ = self.socket.send(pkt).await;
                    len = 0; //continue send empty pkt
                }
                TunnResult::Done => break,
                TunnResult::WriteToTunnelV4(pkt, _src) => {
                    let _ = self.dev.write(pkt).await;
                    break;
                }
                TunnResult::WriteToTunnelV6(pkt, _src) => {
                    let _ = self.dev.write(pkt).await;
                    break;
                }
                r => {
                    warn!(self.logger, "decapsulate result {:?}", r);
                    break;
                }
            }
        }
    }

    async fn reconnect(&mut self) {
        let reconn_to = match self.config.reconnect_timeout {
            Some(v) => Duration::from_secs(v as u64),
            None => return,
        };

        let reconn = self.last_connect.map_or(true, |last_connect| {
            Instant::now().duration_since(last_connect) > reconn_to
        });

        if reconn {
            info!(self.logger, "Reconnect...");
            self.last_connect = Some(Instant::now());
            let _ = self
                .socket
                .connect(&self.config.remote_addr)
                .await
                .map_err(|e| debug!(self.logger, "{:?}", e));
        }
    }

    async fn watchdog(&mut self) {
        let reconn_to = match self.config.reconnect_timeout {
            Some(v) => Duration::from_secs(v as u64),
            None => return,
        };

        let reconn = match self.tun.time_since_last_handshake() {
            Some(dur) => {
                let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                epoch_time - dur > reconn_to
            }
            None => true,
        };

        if reconn {
            self.reconnect().await;
        }
    }
}
