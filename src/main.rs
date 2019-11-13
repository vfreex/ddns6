
use std::io::{self, Write};
use hyper_tls::HttpsConnector;
use hyper::Client;
use std::error::Error;
use tokio::runtime::current_thread::Runtime;

async fn test_https(url: &str) {
    let https = HttpsConnector::new().unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(url.parse().unwrap()).await.unwrap();
    let mut body = res.into_body();
    while let Some(next) = body.next().await {
        let chunk = next.unwrap();
        io::stdout().write_all(&chunk).unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create the runtime
    //let rt = Runtime::new()?;
    let mut rt = Runtime::new()?;
    // Spawn the root task
    rt.block_on(test_https("https://hyper.rs"));
    Ok(())
}
