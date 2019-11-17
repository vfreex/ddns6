use std::net::IpAddr;

use byteorder::{BigEndian, ByteOrder};
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::Deserialize;

use crate::ipaddr_source::interface::InterfaceSource;
use crate::ipaddr_source::web_source::WebSource;
use crate::types::Result;

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Config {
    pub version: String,
    pub log_level: String,
    pub entries: Vec<DdnsEntry>,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct DdnsEntry {
    pub  hostname: String,
    pub  provider: String,
    pub  username: String,
    pub  password: String,
    pub  ipv4: Option<IPAddress>,
    pub  ipv6: Option<IPAddress>,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug)]
#[derive(Clone)]
pub enum IPAddress {
    Static(IpAddr),
    AddressWithPrefixLength { addr: Box<IPAddress>, len: i32 },
    AddressWithNetmask { addr: Box<IPAddress>, mask: IpAddr },
    Composite(Vec<IPAddress>),
    Interface(InterfaceSource),
    Web(WebSource),
}

impl IPAddress {
    pub fn get_ip_address<'a>(&'a self) -> BoxFuture<'a, Result<IpAddr>> {
        async move {
            match self {
                IPAddress::Static(address) => Ok(address.clone()),
                IPAddress::AddressWithNetmask { addr, mask } => {
                    match addr.get_ip_address().await? {
                        IpAddr::V4(ipv4) =>
                            {
                                if let IpAddr::V4(v4mask) = mask {
                                    let l = BigEndian::read_u32(&ipv4.octets());
                                    let r = BigEndian::read_u32(&v4mask.octets());
                                    let a = l & r;
                                    let mut buf = [0; 4];
                                    BigEndian::write_u32(&mut buf, a);
                                    Ok(IpAddr::from(buf))
                                } else {
                                    panic!("Invalid netmask");
                                }
                            }
                        IpAddr::V6(ipv6) => {
                            if let IpAddr::V6(v6mask) = mask {
                                let l = BigEndian::read_u128(&ipv6.octets());
                                let r = BigEndian::read_u128(&v6mask.octets());
                                let a = l & r;
                                let mut buf = [0; 16];
                                BigEndian::write_u128(&mut buf, a);
                                Ok(IpAddr::from(buf))
                            } else {
                                panic!("Invalid netmask");
                            }
                        }
                    }
                }
                IPAddress::AddressWithPrefixLength { addr, len } => {
                    match addr.get_ip_address().await? {
                        IpAddr::V4(ipv4) => {
                            let mut prefix_length = *len as u32;
                            if *len < -32 || *len > 32 {
                                panic!("Prefix length is out of range [-32, 32].");
                            } else if *len < 0 {
                                prefix_length = (32 + *len) as u32;
                            }
                            let l = BigEndian::read_u32(&ipv4.octets());
                            let mut mask = (1 << 32 - prefix_length) - 1;
                            if *len > 0 {
                                mask = !mask;
                            }
                            let ans = l & mask;
                            let mut buf = [0; 4];
                            BigEndian::write_u32(&mut buf, ans);
                            Ok(IpAddr::from(buf))
                        }
                        IpAddr::V6(ipv6) => {
                            let mut prefix_length = *len as u128;
                            if *len < -128 || *len > 128 {
                                panic!("Prefix length is out of range [-128, 128].");
                            } else if *len < 0 {
                                prefix_length = (128 + *len) as u128;
                            }
                            let l = BigEndian::read_u128(&ipv6.octets());
                            let mut mask = (1 << 128 - prefix_length) - 1;
                            if *len > 0 {
                                mask = !mask;
                            }
                            let ans = l & mask;
                            let mut buf = [0; 16];
                            BigEndian::write_u128(&mut buf, ans);
                            Ok(IpAddr::from(buf))
                        }
                    }
                }
                IPAddress::Composite(addresses) => {
                    let mut ans = Box::pin(addresses.first().unwrap().get_ip_address()).await?;
                    if let IpAddr::V4(first) = ans {
                        for addr in addresses.iter().skip(1) {
                            if let IpAddr::V4(ip_addr) = addr.get_ip_address().await? {
                                let l = BigEndian::read_u32(&first.octets());
                                let r = BigEndian::read_u32(&ip_addr.octets());
                                let mut buf = [0; 4];
                                BigEndian::write_u32(&mut buf, l | r);
                                ans = IpAddr::from(buf)
                            } else {
                                panic!("expects {:?} to be IPv4.", addr)
                            }
                        }
                    } else if let IpAddr::V6(first) = ans {
                        for addr in addresses.iter().skip(1) {
                            if let IpAddr::V6(ip_addr) = addr.get_ip_address().await? {
                                let l = BigEndian::read_u128(&first.octets());
                                let r = BigEndian::read_u128(&ip_addr.octets());
                                let mut buf = [0; 16];
                                BigEndian::write_u128(&mut buf, l | r);
                                ans = IpAddr::from(buf)
                            } else {
                                panic!("expects {:?} to be IPv6.", addr)
                            }
                        }
                    }
                    Ok(ans)
                }
                IPAddress::Interface(interface) => {
                    interface.get_ip_address()
                }
                IPAddress::Web(web_source) => {
                    web_source.get_ipv4_address().await
                }
            }
        }.boxed()
    }
}

//impl fmt::Display for IPAddress {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{}", self.get_ip_address())
//    }
//}

