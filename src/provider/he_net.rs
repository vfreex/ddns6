use std::net::IpAddr;

use async_trait::async_trait;

use crate::credential::Credential;
use crate::protocol::dyndns2::DynDns2;
use crate::protocol::protocol::Protocol;
use crate::provider::provider::Provider;
use crate::types::Result;

pub struct HeNetProvider {
    protocol: DynDns2,
}

impl HeNetProvider {
    pub fn new() -> HeNetProvider {
        let protocol = DynDns2::new("https://dyn.dns.he.net");
        HeNetProvider { protocol }
    }
}

#[async_trait]
impl Provider for HeNetProvider {
    async fn update(&self, domain_name: &str, ip_addr: IpAddr, cred: Credential) -> Result<()> {
        self.protocol.update(domain_name, &ip_addr, &cred).await
    }
}
