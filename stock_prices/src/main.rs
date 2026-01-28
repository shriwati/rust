use log::{ info};
use log4rs;
use chrono::NaiveDate;
use stock_prices::prices::{find_avg_price_for_dates, find_price_for_date,read_price};

fn main()->Result<(),csv::Error> {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
        info!("starting up");
        let file_name = "/Users/shrisakrikar/Documents/data/AAPL.csv";
        // get absolute path
        let stock_prices= read_price(&file_name)?;
        let avg_price = find_avg_price_for_dates(&stock_prices
                                                 ,NaiveDate::from_ymd_opt(2025,1,1).unwrap()
                                                 ,NaiveDate::from_ymd_opt(2025,12,31).unwrap());

        let stock_price = find_price_for_date(&stock_prices,NaiveDate::from_ymd_opt(2025,12,31).unwrap());

        match stock_price {
                Some(stock_price) => println!("Price for Apple is {:.4} ", stock_price.close_price),
                None => println!("Price did not receive: ", ),
        }

        match avg_price {
                Some(price) => println!("Avg price for Apple is {:.4} ", price),
                None => println!("Average price did not receive: ", ),
        }
        Ok(())
    }

