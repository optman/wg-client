use crate::config::Config;
use boringtun::crypto::{X25519PublicKey, X25519SecretKey};
use clap::{App, Arg};
use ipnet::IpNet;
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
        .arg(Arg::from_usage("-d, --daemon                        'run as daemon process'"))
        .arg(Arg::from_usage("-a, --local-ips... [ip/prefix]      'add ipv4/ipv6 addr to local interface'"))
        .arg(Arg::from_usage("-s, --allowed-ips... [network/prefix]  'remote allowed ips(for auto add route only)'"))
        .arg(Arg::from_usage("-T, --table [table_name]            'route table for allowed ips'"))
        .arg(Arg::from_usage("-M, --metric [metric]               'metric of the routes'"))
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

    let daemonize = if matches.is_present("daemon") {
        Some(true)
    } else {
        None
    };

    let addrs = match matches.values_of("local-ips") {
        Some(ips) => {
            let mut addrs = Vec::<IpNet>::new();
            for i in ips {
                addrs.push(i.parse()?);
            }

            Some(addrs)
        }
        None => None,
    };

    let routes = match matches.values_of("allowed-ips") {
        Some(ips) => {
            let mut addrs = Vec::<IpNet>::new();
            for i in ips {
                addrs.push(i.parse()?);
            }

            Some(addrs)
        }
        None => None,
    };

    let table = matches.value_of("table").map(Into::into);
    let metric = matches.value_of("metric").map(Into::into);

    Ok(Config {
        local_private_key: private_key,
        remote_public_key: public_key,
        remote_addr: remote_addr,
        ifname: ifname,
        mtu: mtu,
        keepalive: keepalive,
        reconnect_timeout: reconnect_timeout,
        fwmark: fwmark,
        log_level: log_level,
        daemonize: daemonize,
        addrs: addrs,
        routes: routes,
        table: table,
        metric: metric,
    })
}
