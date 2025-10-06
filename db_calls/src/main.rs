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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_hello_world_endpoint() {
        let app = test::init_service(App::new().service(hello_world)).await;

        let req = test::TestRequest::get()
            .uri("/helloworld?name=Alice")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "welcome to Rust Alice");
    }

    #[actix_web::test]
    async fn test_hello_world_with_different_name() {
        let app = test::init_service(App::new().service(hello_world)).await;

        let req = test::TestRequest::get()
            .uri("/helloworld?name=Bob")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "welcome to Rust Bob");
    }

    #[actix_web::test]
    async fn test_hello_world_with_special_characters() {
        let app = test::init_service(App::new().service(hello_world)).await;

        let req = test::TestRequest::get()
            .uri("/helloworld?name=John%20Doe")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "welcome to Rust John Doe");
    }
}
