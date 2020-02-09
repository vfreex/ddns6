use std::net::{Ipv4Addr, Ipv6Addr};

use async_trait::async_trait;

use crate::types::Result;

pub mod error;
pub mod interface;
pub mod web_source;

#[async_trait]
pub trait Ipv4AddrSource {
    async fn get_ipv4_address(&self) -> Result<Ipv4Addr>;
}

#[async_trait]
pub trait Ipv6AddrSource {
    async fn get_ipv6_address(&self) -> Result<Ipv6Addr>;
}
