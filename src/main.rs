use std::io::{self, Write, Read};
use hyper_tls::HttpsConnector;
use hyper::Client;
use std::error::Error;
use std::net::TcpStream;
use toml::Value;
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
    let mut tcp = TcpStream::connect("www.baidu.com:80").unwrap();
    tcp.write("GET / HTTP/1.0\r\n\r\n".as_bytes());

    let mut resp = String::new();
    tcp.read_to_string(&mut resp);
    println!("{}\n", resp.as_str());
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create the runtime
    //let rt = Runtime::new()?;
    let mut rt = Runtime::new()?;
    // Spawn the root task
    rt.block_on(test_https("https://hyper.rs"));
    //test_https("");

    let value = "foo = 'bar'".parse::<Value>().unwrap();

    //assert_eq!(value["foo"].as_str(), Some("bar"));
    panic!("Oops!");
    Ok(())
}
