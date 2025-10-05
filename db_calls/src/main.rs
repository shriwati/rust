use anyhow::Result;
use log::{error, info};
use serde::Deserialize;

#[derive(Debug, Deserialize, clickhouse::Row)]
struct PropertyPriceResult {
    town: String,
    price:u32
}

async fn query_uk_property_prices() -> Result<Vec<PropertyPriceResult>> {
    info!("Connecting to ClickHouse database...");

    let connection = dal::create_connection("127.0.0.1",8123,"uk_property","default").await?;

    info!("Testing database connection...");
    let is_connected = connection.test_connection().await?;
    if !is_connected {
        return Err(anyhow::anyhow!("Failed to connect to ClickHouse database"));
    }
    info!("Database connection successful!");

    let sql = String::from("select  town,max(price)  price
from uk_property.uk_price_paid
group by  town
order by price desc"
);

    info!("Executing query: {}", &sql);

    let client = connection.get_client();
    let results = client
        .query(&sql)
        .fetch_all::<PropertyPriceResult>()
        .await?;

    info!("Query executed successfully. Found {} results.", results.len());
    Ok(results)
}

fn print_property_results(results: &[PropertyPriceResult]) {
    println!("\n{:-<80}", "");
    println!("{:30} | {:>15} ",  "Town","Max Price (£)");
    println!("{:-<80}", "");

    for result in results {
        println!(
            "{:30}{:50}",
            result.town,
            result.price,
        );
    }
    println!("{:-<80}", "");
    println!("Total records: {}", results.len());
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting UK Property Price Analysis...");

    match query_uk_property_prices().await {
        Ok(results) => {
            info!("Successfully retrieved property price data");
            print_property_results(&results);
        }
        Err(e) => {
            error!("Failed to query property prices: {}", e);
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    info!("Application completed successfully");
    Ok(())
}
