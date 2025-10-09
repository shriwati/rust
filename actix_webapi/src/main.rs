use ::log::trace;
use actix_web::{App, HttpServer};

mod api;
mod log;

#[actix_web::main]
async fn main() ->std::io::Result<()> {

    log::init_logging();
    trace!("starting server");
    HttpServer::new(|| {
        App::new()
            .service(api::say_hello)
            .service(api::health)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
    .await

}
