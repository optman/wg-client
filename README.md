# wg-client
A simple wireguard client based on [boringtun](https://github.com/cloudflare/boringtun). Only support connect to one peer, but it can be configured to reconnect(with dns resolve again). Not support wg, only commandline parameter.

### Usage
```
wg-client 0.1.0
wireguard

USAGE:
    wg-client [FLAGS] [OPTIONS] --ifname <ifname> --local-private-key <KEY> --remote <host:port> --remote-public-key <KEY>

FLAGS:
    -d, --daemon     run as daemon process
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --allowed-ips <network/prefix>...    remote allowed ips(for auto add route only)
    -F, --fwmark <fwmark_num>                fwmark set on vpn traffic
    -n, --ifname <ifname>                    interface name
    -K, --keepalive <N>                      seconds between keepalive
    -a, --local-ips <ip/prefix>...           add ipv4/ipv6 addr to local interface
    -k, --local-private-key <KEY>            local private key
    -M, --metric <metric>                    metric of the routes
    -m, --mtu <mtu>                          mtu size [default: 1420]
    -R, --reconnect-timeo <N>                seconds after last handshake to reconnect
    -r, --remote <host:port>                 host:port of remote to connect (brace with [] for bare IPv6)
    -p, --remote-public-key <KEY>            remote public key
    -T, --table <table_name>                 route table for allowed ips
    -v, --verbosity <log_level>              log verbosity [default: info]  [possible values: error, warn, info, debug,
                                             trace]
```

### Examples

WSL2 not support ipv6 and kernal wireguard, use a wg-client to create a tunnel (through ipv4) to the home router(connected to ipv6) to reach ipv6 internet.

WSL2
```
wg-client --ifname wg0 --local-private-key {PRIVATE KEY} --remote-public-key {ROUTER WG PUBLIC KEY} --remote {ROUTER WG SERVER ADDR} -a fd00:1::2/64 -s ::/0 -d
```

Router:
```
#add wireguard support, add a ipv6 addrs fd00:1::1/64(optional), add client pubkey, allowed-ips fd00::1::/64
#add ipv6 nat
ip6tables -t nat -A POSTROUTING -s fd00:1::0/64 -o eth0.2 -j MASQUERADE

```



