use std::net::IpAddr;

use base64::encode;
use bytes::Buf;
use hyper::{Body, Client, client::{
    connect::dns::GaiResolver,
    HttpConnector,
}, header::AUTHORIZATION, Request, Uri};
use hyper_tls::HttpsConnector;

use async_trait::async_trait;

use crate::credential::Credential;
use crate::protocol::protocol::Protocol;
use crate::types::Result;

pub struct DynDns2 {
    server_url: String,
    client: Client<HttpsConnector<HttpConnector<GaiResolver>>>,
}

impl DynDns2 {
    pub fn new(server_url: &str) -> DynDns2 {
        let https = HttpsConnector::new();
        let client = Client::builder().keep_alive(true).build::<_, hyper::Body>(https);
        DynDns2 { server_url: server_url.to_owned(), client }
    }
}

#[async_trait]
impl Protocol for DynDns2 {
    async fn update(&self, domain_name: &str, ip_addr: &IpAddr, cred: &Credential) -> Result<()> {
        let url: Uri = format!("{}/nic/update?hostname={}&myip={}", &self.server_url, domain_name, ip_addr).parse()?;
        //eprintln!("https://{}:{}@{}/nic/update?hostname={}&myip={}", &cred.username, &cred.password, url.authority_part().unwrap().as_str(), domain_name, ip_addr);
        //dbg!(&url);
        let auth_header_val = "Basic ".to_owned() + &encode(&format!("{}:{}", &cred.username, &cred.password));
        let req = Request::get(url.to_string()).header(AUTHORIZATION, &auth_header_val).body(Body::empty())?;
        let res = self.client.request(req).await?;
        if !res.status().is_success() {
            let msg = format!("Error requesting {}: Got status code '{}'.", &url, res.status());
            return Err(msg.into());
        }
        let mut body = hyper::body::aggregate(res).await?;
        let content = String::from_utf8(body.to_bytes().to_vec())?;
        dbg!(&content);
        if !content.starts_with("nochg ") && !content.starts_with("good ") {
            let msg = format!("Failed to update DDNS entry {} to {}: {} returns '{}'.", domain_name, ip_addr, &url, &content);
            return Err(msg.into());
        }
        Ok(())
    }
}
