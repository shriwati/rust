use actix_web::{web, App, HttpServer, HttpRequest, Responder};
use serde::Deserialize;


async fn greet(req:HttpRequest) -> impl Responder {
    let name = req.match_info()
        .get("name")
        .unwrap_or("World");
        format!("Hello {}!", &name)
}
async fn index() -> impl Responder {
    "Hello world!"
}
#[tokio::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
            .route("/{name}", web::get().to(greet))
            .route("/index", web::get().to(index)))
    }).bind("127.0.0.1:8080")
    .unwrap()
    .run().await
}


