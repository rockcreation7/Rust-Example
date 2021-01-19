
extern crate hyper; 
 
use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.

    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        io::stdout().write_all(&chunk?).await?;
    }

    Ok(())
}