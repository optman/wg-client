# wg-client
A simple wireguard client based on [boringtun](https://github.com/cloudflare/boringtun). Only support connect to one peer, but it can be configured to reconnect(with dns resolve again). Not support wg, only commandline parameter.

### Usage
```
wg-client 0.1.0
wireguard

USAGE:
    wg-client [OPTIONS] --ifname <ifname> --local-private-key <KEY> --remote <host:port> --remote-public-key <KEY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -F, --fwmark <fwmark_num>        fwmark set on vpn traffic
    -n, --ifname <ifname>            interface name
    -K, --keepalive <N>              seconds between keepalive
    -k, --local-private-key <KEY>    local private key
    -m, --mtu <mtu>                  mtu size [default: 1420]
    -R, --reconnect-timeo <N>        seconds after last handshake to reconnect
    -r, --remote <host:port>         host:port of remote to connect (brace with [] for bare IPv6)
    -p, --remote-public-key <KEY>    remote public key
    -v, --verbosity <log_level>      log verbosity [default: info]  [possible values: error, warn, info, debug, trace]
```

