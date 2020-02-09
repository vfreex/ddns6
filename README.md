# DDNS6 - One dynamic DNS client for multiple hosts

DDNS6 is a dynamic DNS client for updating dynamic DNS entries for multiple hosts.

*Note: The project is currently in an early stage of development. Don't use in production!*

DDNS6 supports updating dynamic DNS entries for accounts on many dynamic DNS services.
It has the following great advantages:

- Built for IPv4/IPv6 dual-stack.
- One instance for multiple hosts. DDNS6 supports configuring address prefix and suffix separately and combining them together to batch update dynamic DNS entries for multiple hosts.
- Cross-platform. Running on Linux, OpenWrt/LEDE, EdgeOS/Vyatta, Windows, and macOS.
- Great performance with low resource cost by being native and using asynchronous I/O.

## Getting Started

### Build
```sh
$ cargo build --release
```
The resulting binary is at `./target/release/ddns6`.

### Configuration

*NOTE: As of this writing, only _he.net_ provider is supported.*

This is a simple example:
The domain name `1.example.com` is hosted on <https://dns.he.net>.
The IPv4 address is obtained from web, and IPv6 address is a combination of
prefix from local interface `br0` and suffix of static value `::1`.

```toml
# config/ddns6.toml
[[entries]]
hostname="1.example.com"
username="1.example.com"
password="your-password"
provider="he.net"
ipv4={web="_"}
ipv6=[{addr={dev="br0"}, len=64}, "::1"]
```

### Run
```sh
./ddns6
```
Current version of DDNS6 updates the DNS records for domain names every 5 minutes.