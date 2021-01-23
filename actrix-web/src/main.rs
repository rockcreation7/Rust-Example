use actix_web::{post, web, Responder, middleware, App, HttpRequest, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world! index handler"
}

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
async fn json_index(item: web::Json<MyObj>) -> String  {
    println!("model: {:?}", &item);
    "HttpResponse::Ok().json(item.0)".to_string() // <- send response
}
 
#[post("/{id}/{name}/index.html")]
async fn example_basic(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id);
    web::Json(
        MyObj{name: "Name".to_string(), number: 3} 
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Closure!" }))
            .service(web::resource("/").to(index)) 
            .service(example_basic)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod main_test;