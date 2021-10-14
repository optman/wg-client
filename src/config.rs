use slog::Level;

pub(crate) struct Config {
    pub(crate) log_level: Level,
    pub(crate) local_private_key: String,
    pub(crate) remote_public_key: String,
    pub(crate) remote_addr: String,
    pub(crate) ifname: String,
    pub(crate) mtu: i32,
    pub(crate) keepalive: Option<u16>,
    pub(crate) reconnect_timeout: Option<u16>,
    pub(crate) fwmark: Option<u32>,
}
