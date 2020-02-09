use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use byteorder::{BigEndian, ByteOrder};
use futures::future::BoxFuture;
use futures::future::try_join_all;
use futures::FutureExt;
use serde::Deserialize;

use crate::ipaddr_source::{Ipv4AddrSource, Ipv6AddrSource};
use crate::ipaddr_source::error::AddressDeterminationError;
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
    pub  ipv4: Option<IPAddressConf>,
    pub  ipv6: Option<IPAddressConf>,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug)]
#[derive(Clone)]
pub enum IPAddressConf {
    Static(IpAddr),
    AddressWithPrefixLength { addr: Box<IPAddressConf>, len: i32 },
    AddressWithNetmask { addr: Box<IPAddressConf>, mask: IpAddr },
    Composite(Vec<IPAddressConf>),
    Interface(InterfaceSource),
    Web(WebSource),
}

impl IPAddressConf {
    pub fn get_ipv4_address(&self) -> BoxFuture<Result<IpAddr>> {
        async move {
            match self {
                IPAddressConf::Static(address) => {
                    match address {
                        IpAddr::V4(_) => Ok(address.clone()),
                        _ => Err(Box::new(AddressDeterminationError) as Box<dyn Error + Sync + Send>),
                    }
                }
                IPAddressConf::AddressWithNetmask { addr, mask } => {
                    let ipv4 = match addr.get_ipv4_address().await? {
                        IpAddr::V4(addr) => addr,
                        _ => panic!(),
                    };
                    let v4mask = match mask {
                        IpAddr::V4(addr) => addr,
                        _ => panic!(),
                    };
                    let l = BigEndian::read_u32(&ipv4.octets());
                    let r = BigEndian::read_u32(&v4mask.octets());
                    let ans = l & r;
                    Ok(IpAddr::V4(Ipv4Addr::from(ans)))
                }
                IPAddressConf::AddressWithPrefixLength { addr, len } => {
                    let ipv4 = match addr.get_ipv4_address().await? {
                        IpAddr::V4(addr) => addr,
                        _ => panic!(),
                    };
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
                    Ok(IpAddr::V4(Ipv4Addr::from(ans)))
                }
                IPAddressConf::Composite(addresses) => {
                    let mut ans: u32 = 0;
                    let futs = addresses.iter().map(|addr| addr.get_ipv4_address());
                    for addr in try_join_all(futs).await? {
                        let ip_addr = match addr {
                            IpAddr::V4(addr) => addr,
                            _ => panic!(),
                        };
                        let val = BigEndian::read_u32(&ip_addr.octets());
                        ans |= val;
                    }
                    Ok(IpAddr::V4(Ipv4Addr::from(ans)))
                }
                IPAddressConf::Interface(interface) => {
                    interface.get_ipv4_address().await.map(|addr| IpAddr::V4(addr))
                }
                IPAddressConf::Web(web_source) => {
                    web_source.get_ipv4_address().await.map(|addr| IpAddr::V4(addr))
                }
            }
        }.boxed()
    }

    pub fn get_ipv6_address(&self) -> BoxFuture<Result<IpAddr>> {
        async move {
            match self {
                IPAddressConf::Static(address) => {
                    match address {
                        IpAddr::V6(_) => Ok(address.clone()),
                        _ => Err(Box::new(AddressDeterminationError) as Box<dyn Error + Sync + Send>),
                    }
                }
                IPAddressConf::AddressWithNetmask { addr, mask } => {
                    let ipv6 = match addr.get_ipv6_address().await? {
                        IpAddr::V6(addr) => addr,
                        _ => panic!(),
                    };
                    let v6mask = match mask {
                        IpAddr::V6(addr) => addr,
                        _ => panic!(),
                    };
                    let l = BigEndian::read_u128(&ipv6.octets());
                    let r = BigEndian::read_u128(&v6mask.octets());
                    let ans = l & r;
                    Ok(IpAddr::V6(Ipv6Addr::from(ans)))
                }
                IPAddressConf::AddressWithPrefixLength { addr, len } => {
                    let ipv6 = match addr.get_ipv6_address().await? {
                        IpAddr::V6(addr) => addr,
                        _ => panic!(),
                    };
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
                    Ok(IpAddr::V6(Ipv6Addr::from(ans)))
                }
                IPAddressConf::Composite(addresses) => {
                    let mut ans: u128 = 0;
                    let futs = addresses.iter().map(|addr| addr.get_ipv6_address());
                    for addr in try_join_all(futs).await? {
                        let ip_addr = match addr {
                            IpAddr::V6(addr) => addr,
                            _ => panic!(),
                        };
                        let val = BigEndian::read_u128(&ip_addr.octets());
                        ans |= val;
                    }
                    Ok(IpAddr::V6(Ipv6Addr::from(ans)))
                }
                IPAddressConf::Interface(interface) => {
                    interface.get_ipv6_address().await.map(|addr| IpAddr::V6(addr))
                }
                IPAddressConf::Web(web_source) => {
                    web_source.get_ipv6_address().await.map(|addr| IpAddr::V6(addr))
                }
            }
        }.boxed()
    }
}
