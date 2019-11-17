use std::convert::Infallible;
use std::net::{IpAddr, ToSocketAddrs};
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

use crate::types::Result;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct WebSource {
    web: String,
    #[serde(skip)]
    client: i32,
}

impl WebSource {
    pub async fn get_ipv4_address(&self) -> Result<IpAddr> {
        let url = Uri::from_static("http://ip.sb");
        let resolver = tower::service_fn(|name: Name| async {
            let domain_name = name;
            let addrs_iter = (domain_name.to_string() + ":0").to_socket_addrs().unwrap();
            let resolved_ips: Vec<IpAddr> = addrs_iter.filter(|addr| addr.is_ipv4())
                .map(|ref addr| addr.ip()).collect();
            Ok::<_, Infallible>(resolved_ips.into_iter())
        });
        let http = HttpConnector::new_with_resolver(resolver);
        //let http = HttpConnector::new();
        let tls = native_tls::TlsConnector::new()?;
        let connector = HttpsConnector::from((http, tls.into()));
        let client = Client::builder().keep_alive(false).build::<_, Body>(connector);
        let request = Request::get(url).header("User-Agent", "curl/7.64.1").body(Body::empty())?;
        let response = client.request(request).await?;
        let mut body = hyper::body::aggregate(response).await?;
        let content = String::from_utf8(body.to_bytes().to_vec())?;
        println!("yuxzhu: Got {}", &content);
        Ok(IpAddr::from_str(content.trim())?)
    }
}
