
 

extern crate hyper;  
use hyper::{body::HttpBody as _, Client, Body, Method, Request, Uri, http::Error};
 
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

    post().await;

    Ok(())
}

async fn post() -> Result<(),  Box<dyn std::error::Error + Send + Sync>> {

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;


    let client = Client::new();

    // POST it...
    let mut resp = client.request(req).await?;

    while let Some(chunk) = resp.body_mut().data().await {
        io::stdout().write_all(&chunk?).await?;
    }
    
    println!("Response: {}", resp.status());

    // We'll send it in a second...
    Ok(())
}