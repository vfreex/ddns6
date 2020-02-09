use std::fs;
use std::net::IpAddr;
use std::time::Duration;

use futures::FutureExt;
use futures::future::{BoxFuture, join_all};
use tokio::runtime;
use tokio::time;
use toml;

use ddns6::config::{Config, DdnsEntry};
use ddns6::credential::Credential;
use ddns6::provider::he_net::HeNetProvider;
use ddns6::provider::provider::Provider;
use ddns6::types::Ddns6Error;

type Result<T> = ddns6::types::Result<T>;

async fn ddns6_update<'a>(entry: &'a DdnsEntry, provider: &'a (dyn Provider + Sync)) -> Result<()> {
    let mut futs = Vec::new();
    let cred = Credential { username: entry.username.to_owned(), password: entry.password.to_owned() };
    let ipv4_fut: BoxFuture<Result<IpAddr>> = entry.ipv4.as_ref().unwrap().get_ipv4_address();
    let ipv6_fut: BoxFuture<Result<IpAddr>> = entry.ipv6.as_ref().unwrap().get_ipv6_address();
    let fut_fn = |ip_fut: BoxFuture<'a, Result<IpAddr>>| {
        let entry = entry.clone();
        let cred = cred.clone();
        async move {
            let ip_addr = ip_fut.await?;
//            if let Err(err) = ip_addr {
//                eprintln!("Error {}", &err);
//                return Err(err);
//            }
//            let ip_addr = ip_addr.unwrap();
            eprintln!("Updating {} -> {}", entry.hostname.as_str(), &ip_addr);
            match provider.update(&entry.hostname, ip_addr, cred.clone()).await {
                Ok(_) => Result::Ok(ip_addr),
                Err(err) => Err(Box::new(Ddns6Error(format!("Failed updating {} -> {}: {}", &entry.hostname, ip_addr, err)))
                    as Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>)
            }
        }.boxed()
    };
    futs.push(fut_fn(ipv4_fut));
    futs.push(fut_fn(ipv6_fut));
    for (index, result) in join_all(futs).await.iter().enumerate() {
        match result {
            Ok(ip) => eprintln!("Updated {} -> {}", entry.hostname.as_str(), ip),
            Err(err) => eprintln!("Error updating {}: {}", entry.hostname.as_str(), err)
        }
    }
    Ok(())
}

async fn ddns6_main() -> Result<()> {
    let he = HeNetProvider::new();
    let config_content = fs::read_to_string("./config/ddns6.toml")?;
    let config: Config = toml::from_str(&config_content)?;

    let mut interval = time::interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        //ddns6_update(&config, &he).await;
        let mut futs = Vec::new();
        for entry in config.entries.iter() {
            futs.push(ddns6_update(entry, &he));
        }
        let results = join_all(futs).await;
        for (index, result) in results.iter().enumerate() {
            let entry = &config.entries[index];
            match result {
                Ok(_) => eprintln!("Updated: {}", entry.hostname.as_str()),
                Err(err) => eprintln!("Update {} failed: {}", entry.hostname.as_str(), err)
            }
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
