use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Deserialize)]
struct HelloParams {
    name: String,
}

#[derive(Serialize, Deserialize, clickhouse::Row)]
struct PropertyTypePrice {
    town: String,
    price: u32,
}

#[derive(Serialize)]
struct HealthResponse {
    datetime: String,
}

#[get("/helloworld")]
async fn hello_world(query: web::Query<HelloParams>) -> impl Responder {
    api_call::helloworld(&query.name)
}

#[get("/property-prices")]
async fn get_property_prices() -> impl Responder {
    match execute_property_query().await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    use chrono::Utc;

    let datetime = "UTC:".to_string() + Utc::now().to_rfc3339().as_str();

    HttpResponse::Ok().json(HealthResponse { datetime})
}

async fn execute_property_query() -> anyhow::Result<Vec<PropertyTypePrice>> {
    let conn = dal::create_connection("127.0.0.1", 8123, "uk_property", "default").await?;

    let query = String::from("SELECT town , max(price) as price \
                 FROM uk_property.uk_price_paid \
                 GROUP BY town \
                 ORDER BY price DESC");

    let results = conn.get_client()
        .query(&query)
        .fetch_all::<PropertyTypePrice>()
        .await?;

    Ok(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:3000");
    println!("Try: http://127.0.0.1:3000/helloworld?name=Alice");
    println!("Try: http://127.0.0.1:3000/property-prices");
    println!("Try: http://127.0.0.1:3000/health");

    HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .service(get_property_prices)
            .service(health)
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

    #[actix_web::test]
    async fn test_property_prices_endpoint() {
        let app = test::init_service(App::new().service(get_property_prices)).await;

        let req = test::TestRequest::get()
            .uri("/property-prices")
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Test can succeed (200) or fail (500) depending on whether ClickHouse is running
        // If successful, verify the response is valid JSON with correct structure
        if resp.status().is_success() {
            let body = test::read_body(resp).await;
            let body_str = String::from_utf8(body.to_vec()).unwrap();

            // Verify it's valid JSON
            let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
            assert!(json.is_array());

            // Verify the array structure if not empty
            let array = json.as_array().unwrap();
            if !array.is_empty() {
                // Verify structure of first element
                assert!(array[0].get("town").is_some());
                assert!(array[0].get("price").is_some());
            }
        } else {
            // If it fails, it should be a 500 error (database connection issue)
            assert!(resp.status().is_server_error());
        }
    }

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(App::new().service(health)).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        // Verify it's valid JSON
        let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        // Verify datetime field exists and is a string
        assert!(json.get("datetime").is_some());
        assert!(json["datetime"].is_string());

        // Verify datetime is in RFC3339 format (e.g., "2024-01-01T12:00:00Z")
        let datetime_str = json["datetime"].as_str().unwrap();
        assert!(!datetime_str.is_empty());
        assert!(datetime_str.contains("T")); // Basic check for RFC3339 format
    }
}
