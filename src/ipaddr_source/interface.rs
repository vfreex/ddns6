use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::str::{FromStr, Lines};

use serde::Deserialize;

use crate::types::Result;

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct InterfaceSource {
    pub dev: String,
    pub v: i32,
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
                    [169, 254, _, _ ] => false,
                    _ => true
                }
                IpAddr::V6(ipv6) => match ipv6.segments() {
                    [0xfe80, _, _, _, _, _, _, _ ] => false,
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

impl InterfaceSource {
    pub fn get_ip_address(&self) -> Result<IpAddr> {
        let output = Command::new("ip")
            .arg("addr").arg("show").arg("dev").arg(&self.dev)
            .output().unwrap();
        if !output.status.success() {
            panic!("Error running ip addr, exited with {}", output.status)
        }
        let stdout = String::from_utf8(output.stdout).unwrap();
        let iter = InterfaceAddressIterator::new(self.v, &stdout);
        for addr in iter {
            return Ok(addr);
        }
        Ok(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
    }
}