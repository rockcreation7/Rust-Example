use super::*;
use actix_web::dev::Service;
use actix_web::{http, test, web, App, Error};

#[actix_rt::test]
async fn test_index() -> Result<(), Error> {
    let app = App::new().route("/", web::get().to(index));
    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = app.call(req).await.unwrap();

    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, r##"Hello world! index handler"##);

    Ok(())
}

#[actix_rt::test]
async fn test_another() -> Result<(), Error> {
    let app = App::new().route("/index.html", web::get().to(|| async { "Closure!" }));
    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/index.html").to_request();
    let resp = app.call(req).await.unwrap();

    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, r##"Closure!"##);

    Ok(())
}