use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tokio::process::Command;
use std::str::{FromStr, Lines};

use serde::Deserialize;

use async_trait::async_trait;

use crate::ipaddr_source::{Ipv4AddrSource, Ipv6AddrSource};
use crate::ipaddr_source::error::AddressDeterminationError;
use crate::types::Result;

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct InterfaceSource {
    pub dev: String,
}

struct InterfaceAddressIterator<'a> {
    v: i32,
    lines: Lines<'a>,
}

impl<'a> InterfaceAddressIterator<'a> {
    fn new(ip_version: i32, s: &str) -> InterfaceAddressIterator {
        InterfaceAddressIterator { v: ip_version, lines: s.lines() }
    }
}

fn index_of(s: &str, pattern: &str, start: usize) -> Option<usize> {
    s[start..].find(pattern).map(|index| index + start)
}

impl<'a> std::iter::Iterator for InterfaceAddressIterator<'a> {
    type Item = IpAddr;

    fn next(&mut self) -> Option<Self::Item> {
        let pattern = match self.v {
            4 => "inet ",
            6 => "inet6 ",
            _ => return None
        };
        let lines = &mut self.lines;
        while let Some(line) = lines.next() {
            let mut pat_start = match line.find(pattern) {
                Some(index) => index,
                None => continue
            };
            pat_start += pattern.len();
            let pat_end = match index_of(line, "/", pat_start)
                .or_else(|| index_of(line, " ", pat_start)) {
                Some(index) => index,
                None => line.len()
            };
            let extracted_addr = &line[pat_start..pat_end];
            println!("Extracted Addr: {}", extracted_addr);
            if let Some(_) = line.find("temporary") {
                println!("Ignored because it is a temporary address");
                continue;
            }
            if let Some(_) = line.find("detached") {
                println!("Ignored because it is a detached address");
                continue;
            }
            if let Some(_) = line.find("deprecated") {
                println!("Ignored because it is a deprecated address");
                continue;
            }
            let addr = match IpAddr::from_str(extracted_addr) {
                Ok(addr) => addr,
                Err(_) => continue
            };
            let valid = match addr {
                IpAddr::V4(ipv4) => match ipv4.octets() {
                    [0, _, _, _ ] => false,
                    [10, _, _, _ ] => false,
                    [100, 64..=127, _, _ ] => false,
                    [127, _, _, _ ] => false,
                    [172, 16..=31, _, _ ] => false,
                    [169, 254, _, _ ] => false,
                    [192, 168, _, _ ] => false,
                    _ => true
                }
                IpAddr::V6(ipv6) => match ipv6.segments() {
                    [0x0000..=0x1fff, _, _, _, _, _, _, _ ] => false,
                    [0xfe80, _, _, _, _, _, _, _ ] => false,
                    [0xfc00..=0xfdff, _, _, _, _, _, _, _ ] => false,
                    _ => true
                }
            };
            if !valid {
                println!("Ignored because it is not a valid address");
                continue;
            }
            return Some(addr);
        }
        None
    }
}

async fn get_ip_addr_output(dev: &str) -> Result<String> {
    let output = Command::new("ip")
        .arg("addr").arg("show").arg("dev").arg(dev)
        .output();
    let output = output.await?;
    if !output.status.success() {
        return Err(Box::new(AddressDeterminationError("Failed to run `ip addr` command. Is iproute2 installed?".to_owned())))
    }
    Ok(String::from_utf8(output.stdout)?)
}

#[async_trait]
impl Ipv4AddrSource for InterfaceSource {
    async fn get_ipv4_address(&self) -> Result<Ipv4Addr> {
        let stdout = get_ip_addr_output(self.dev.as_str()).await?;
        let iter = InterfaceAddressIterator::new(4, stdout.as_str());
        for addr in iter {
            let addr = match addr {
                IpAddr::V4(ipv4) => ipv4,
                _ => panic!()
            };
            return Ok(addr);
        }
        Err(Box::new(AddressDeterminationError(format!("No viable IPv4 addresses configured on interface {}", self.dev).to_owned())))
    }
}

#[async_trait]
impl Ipv6AddrSource for InterfaceSource {
    async fn get_ipv6_address(&self) -> Result<Ipv6Addr> {
        let stdout = get_ip_addr_output(self.dev.as_str()).await?;
        let iter = InterfaceAddressIterator::new(6, stdout.as_str());
        for addr in iter {
            let addr = match addr {
                IpAddr::V6(ipv6) => ipv6,
                _ => panic!()
            };
            return Ok(addr);
        }
        Err(Box::new(AddressDeterminationError(format!("No viable IPv6 addresses configured on interface {}", self.dev).to_owned())))
    }
}