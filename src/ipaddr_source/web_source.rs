use std::convert::Infallible;
use std::net::{IpAddr, ToSocketAddrs, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use bytes::Buf;
use hyper::Body;
use hyper::Client;
use hyper::client::connect::dns::Name;
use hyper::client::HttpConnector;
use hyper::Request;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use async_trait::async_trait;

use crate::ipaddr_source::{Ipv4AddrSource, Ipv6AddrSource};
use crate::types::Result;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct WebSource {
    #[serde(skip)]
    web: String,
}

#[async_trait]
impl Ipv4AddrSource for WebSource {
    async fn get_ipv4_address(&self) -> Result<Ipv4Addr> {
        let url = Uri::from_static("http://ip.sb");
        let resolver = tower::service_fn(|name: Name| async {
            let domain_name = name;
            let addrs_iter = (domain_name.to_string() + ":0").to_socket_addrs().unwrap();
            let resolved_ips: Vec<IpAddr> = addrs_iter.filter(|addr| addr.is_ipv4())
                .map(|ref addr| addr.ip()).collect();
            Ok::<_, Infallible>(resolved_ips.into_iter())
        });
        let http = HttpConnector::new_with_resolver(resolver);
        let tls = native_tls::TlsConnector::new()?;
        let connector = HttpsConnector::from((http, tls.into()));
        let client = Client::builder().keep_alive(false).build::<_, Body>(connector);
        let request = Request::get(url).header("User-Agent", "curl/7.64.1").body(Body::empty())?;
        let response = client.request(request).await?;
        let mut body = hyper::body::aggregate(response).await?;
        let content = String::from_utf8(body.to_bytes().to_vec())?;
        Ok(Ipv4Addr::from_str(content.trim())?)
    }
}

#[async_trait]
impl Ipv6AddrSource for WebSource {
    async fn get_ipv6_address(&self) -> Result<Ipv6Addr> {
        let url = Uri::from_static("http://ip.sb");
        let resolver = tower::service_fn(|name: Name| async {
            let domain_name = name;
            let addrs_iter = (domain_name.to_string() + ":0").to_socket_addrs().unwrap();
            let resolved_ips: Vec<IpAddr> = addrs_iter.filter(|addr| addr.is_ipv6())
                .map(|ref addr| addr.ip()).collect();
            if resolved_ips.is_empty() {
                return Err(format!("Failed to resolve AAAA record for {}", &domain_name));
            }
            Ok::<_, String>(resolved_ips.into_iter())
        });
        let http = HttpConnector::new_with_resolver(resolver);
        let tls = native_tls::TlsConnector::new()?;
        let connector = HttpsConnector::from((http, tls.into()));
        let client = Client::builder().keep_alive(false).build::<_, Body>(connector);
        let request = Request::get(url).header("User-Agent", "curl/7.64.1").body(Body::empty())?;
        let response = client.request(request).await?;
        let mut body = hyper::body::aggregate(response).await?;
        let content = String::from_utf8(body.to_bytes().to_vec())?;
        Ok(Ipv6Addr::from_str(content.trim())?)
    }
}
