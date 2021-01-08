// reqwest_client/src/main.rs
use serde::{Deserialize};

#[derive(Deserialize)]
struct Simple {
   origin: String
}

fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::blocking::get("http://httpbin.org/get")?.json::<Simple>()?;
    /*
    println!("Status: {}", res.status());
    println!("Headers:{:#?}", res.headers());
    */
 
    // let body = res.json::<Simple>().await?;
    println!("{}", res.origin);

    let res = reqwest::blocking::get("http://httpbin.org/get")?.json::<Simple>()?;
    println!("{}", res.origin);

    let res = reqwest::blocking::get("http://httpbin.org/get")?.json::<Simple>()?;
    println!("{}", res.origin);

    Ok(())
}

/*
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
*/
