use async_trait::async_trait;
use std::error::Error;
use crate::credential::Credential;
use std::net::IpAddr;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[async_trait]
pub trait Protocol {
    async fn update(&self, domain_name: &str, ip_addr: &IpAddr, cred: &Credential) -> Result<()>;
}