use actix_web::{get, web, Responder, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use log::{info, debug};



#[derive(Deserialize)]
pub struct NameQuery {
    name: String,
}

#[get("/hello")]
async fn say_hello(query: web::Query<NameQuery>) -> impl Responder {
    info!("Received hello request for name: {}", &query.name);
    HttpResponse::Ok().body(format!("Hello, {}!", &query.name))
}

#[get("/health")]
async fn health() -> impl Responder {
    debug!("Health check endpoint called");
    let timestamp = Utc::now().to_rfc2822();
    HttpResponse::Ok().body(format!("UTC = {}", timestamp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_say_hello() {
        let app = test::init_service(App::new().service(say_hello)).await;
        let req = test::TestRequest::get()
            .uri("/hello?name=World")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, World!");
    }

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.starts_with("UTC="));
    }
}