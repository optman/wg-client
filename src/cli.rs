use crate::config::Config;
use boringtun::crypto::{X25519PublicKey, X25519SecretKey};
use clap::{App, Arg};
use slog::Level;
use std::error::Error;
use std::result::Result;

const DEFAULT_MTU: i32 = 1420;

pub(crate) fn parse() -> Result<Config, Box<dyn Error>> {
    let default_mtu_str = DEFAULT_MTU.to_string();

    let matches = App::new("wg-client")
        .version(env!("CARGO_PKG_VERSION"))
        .about("wireguard")
        .arg(Arg::from_usage("-r, --remote <host:port>            'host:port of remote to connect (brace with [] for bare IPv6)'"))
        .arg(Arg::from_usage("-n, --ifname <ifname>               'interface name'"))
        .arg(Arg::from_usage("-k --local-private-key <KEY>        'local private key'"))
        .arg(Arg::from_usage("-p --remote-public-key <KEY>        'remote public key'"))
        .arg(Arg::from_usage( "-m, --mtu [mtu]                    'mtu size'") .default_value(&default_mtu_str))
        .arg(Arg::from_usage("-R, --reconnect-timeo [N]           'seconds after last handshake to reconnect'"))
        .arg(Arg::from_usage("-K, --keepalive [N]                 'seconds between keepalive'"))
        .arg(Arg::from_usage("-F, --fwmark [fwmark_num]           'fwmark set on vpn traffic'"))
        .arg(Arg::from_usage("-v, --verbosity [log_level]         'log verbosity'")
            .possible_values(&["error","warn", "info", "debug", "trace"])
            .default_value("info"))
        .get_matches();

    let remote_addr: String = matches.value_of("remote").map(Into::into).unwrap();
    let ifname: String = matches.value_of("ifname").map(Into::into).unwrap();

    let private_key = matches
        .value_of("local-private-key")
        .ok_or("not set")
        .and_then(|v| {
            v.parse::<X25519SecretKey>()?;
            Ok(v.to_string())
        })
        .map_err(|_| "invalid private-key")?;

    let public_key = matches
        .value_of("remote-public-key")
        .ok_or("not set")
        .and_then(|v| {
            v.parse::<X25519PublicKey>()?;
            Ok(v.to_string())
        })
        .map_err(|_| "invalid public-key")?;

    let mtu = match matches.value_of("mtu") {
        Some(v) => v.parse().map_err(|_| "invalid mtu")?,
        None => DEFAULT_MTU,
    };

    let keepalive: Option<u16> = match matches.value_of("keepalive") {
        Some(v) => Some(v.parse().map_err(|_| "invalid keepalive")?),
        None => None,
    };

    let reconnect_timeout: Option<u16> = match matches.value_of("reconnect-timeo") {
        Some(v) => Some(v.parse().map_err(|_| "invalid reconnect-timeo")?),
        None => None,
    };

    let fwmark: Option<u32> = match matches.value_of("fwmark") {
        Some(v) => Some(v.parse().map_err(|_| "invalid fwmark")?),
        None => None,
    };

    let log_level: Level = matches
        .value_of("verbosity")
        .unwrap()
        .parse()
        .map_err(|_| "invalid log verbosity")?;

    Ok(Config {
        log_level: log_level,
        local_private_key: private_key,
        remote_public_key: public_key,
        remote_addr: remote_addr,
        ifname: ifname,
        mtu: mtu,
        keepalive: keepalive,
        reconnect_timeout: reconnect_timeout,
        fwmark: fwmark,
    })
}
