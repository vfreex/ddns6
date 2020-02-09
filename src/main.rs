use std::error::Error;
use std::fs;
use std::net::IpAddr;

use futures::FutureExt;
use futures::future::{BoxFuture, join_all};
use tokio::runtime;
use toml;

use ddns6::config::Config;
use ddns6::credential::Credential;
use ddns6::provider::he_net::HeNetProvider;
use ddns6::provider::provider::Provider;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

async fn ddns6_main<'a>() -> Result<()> {
    let he = HeNetProvider::new();
    let config_content = fs::read_to_string("./config/ddns6.toml")?;
    let config: Config = toml::from_str(&config_content)?;

    let mut futs = Vec::new();
    for entry in config.entries.iter() {
        let cred = Credential { username: entry.username.to_owned(), password: entry.password.to_owned() };
        let ipv4_fut: BoxFuture<Result<IpAddr>> = entry.ipv4.as_ref().unwrap().get_ipv4_address();
        let ipv6_fut: BoxFuture<Result<IpAddr>> = entry.ipv6.as_ref().unwrap().get_ipv6_address();
        let fut_fn = |ip_fut: BoxFuture<'a, Result<IpAddr>>| {
            let entry = entry.clone();
            let he = &he;
            let cred = cred.clone();
            async move {
                let ip_addr = ip_fut.await?;
                eprintln!("Updating {} -> {}", entry.hostname.as_str(), &ip_addr);
                he.update(&entry.hostname, ip_addr, cred.clone()).await
            }.boxed()
        };
        futs.push(fut_fn(ipv4_fut));
        futs.push(fut_fn(ipv6_fut));
    }
    for r in join_all(futs).await {
        if let Err(err) = r {
            eprintln!("Update failed: {}", err);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut rt = runtime::Builder::new()
        .basic_scheduler()
        .enable_time()
        .enable_io()
        .build()?;
    rt.block_on(ddns6_main())?;
    Ok(())
}
