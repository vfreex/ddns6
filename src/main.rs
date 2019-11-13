//use std::env;
use std::io::{self, Write};
use hyper_tls::HttpsConnector;
use hyper::Client;


#[tokio::main]
async fn main() -> Result<(), hyper::Error>{
    // 4 is number of blocking DNS threads
    let https = HttpsConnector::new().unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://hyper.rs".parse().unwrap()).await?;
    let mut body = res.into_body();
    while let Some(next) = body.next().await {
        let chunk = next.unwrap();
        io::stdout().write_all(&chunk).unwrap();
    }
    Ok(())
}
