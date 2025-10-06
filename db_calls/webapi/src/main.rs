use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloParams {
    name: String,
}

#[get("/helloworld")]
async fn hello_world(query: web::Query<HelloParams>) -> impl Responder {
    api_call::helloworld(&query.name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:3000");
    println!("Try: http://127.0.0.1:3000/helloworld?name=Alice");

    HttpServer::new(|| {
        App::new()
            .service(hello_world)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
