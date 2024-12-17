use actix_web::{web, App, HttpResponse, Responder, HttpServer};
use chrono;


async  fn hello()->impl Responder{
    //returns current server time in UTS
    HttpResponse::Ok().body(chrono::offset::Utc::now().time().to_string())
}

#[actix_web::main]
async fn  main()->std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/hello",web::get().to(hello))
    })
        .bind(("127.0.0.1",8080))?
        .run()
        .await

}