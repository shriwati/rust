pub mod prices {
    use chrono::NaiveDate;
    use csv::StringRecord;
    use serde::{Deserialize, Serialize};
    use date_format_serde::date_format_naive;
    use log::{error, info};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct StockPrices {
        name: String,
        #[serde(with = "date_format_naive")]
        pub trade_date: NaiveDate,
        pub open_price: f64,
        pub high_price: f64,
        pub low_price: f64,
        pub close_price: f64,
        pub volume: u64,
    }

    impl StockPrices {
        pub fn new(
            name: String,
            trade_date: NaiveDate,
            open_price: f64,
            high_price: f64,
            low_price: f64,
            close_price: f64,
            volume: u64,
        ) -> StockPrices {
            StockPrices {
                name,
                trade_date,
                open_price,
                high_price,
                low_price,
                close_price,
                volume,
            }
        }

    }
    pub fn find_price_for_date(stock_prices: &Vec<StockPrices>, date: NaiveDate) -> Option<&StockPrices> {
        // find the record for passed in trade_date
        stock_prices.iter()
            .find(|stock_price| stock_price.trade_date == date)
    }

    pub fn find_avg_price_for_dates(stock_prices:&Vec<StockPrices>,from_date:NaiveDate,to_date:NaiveDate)->Option<f64>{

        let (sum,count) = stock_prices.iter()
            .filter(|stock_price| stock_price.trade_date >= from_date && stock_price.trade_date <= to_date)
            .fold((0.0,0),|(sum,count), sp|( sum + sp.close_price,count + 1));
            if count == 0 {
                 None
            }else {
                Some(sum / count as f64)
            }
    }

    pub fn read_price(file_name: &str) ->Result<Vec<StockPrices>,csv::Error> {
        info!("reading csv file: {}", file_name);
        // log every step of the way
        // Open the file with full path
        let mut stock_prices: Vec<StockPrices> = Vec::new();
        let header = StringRecord::from(vec![
            "name", "trade_date", "open_price", "high_price", "low_price", "close_price", "volume",
        ]);
        // get a reader
        // check if file exists
        if !std::path::Path::new(file_name).exists() {
            return Err(csv::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found")));
        }
        let mut rdr = csv::Reader::from_path(file_name);
        // if OK
        match &mut rdr {
            Ok(rdr) => {
                // for each record in reader
                for record in rdr.records() {
                    match record {
                        Ok(record) => {
                            info!("Deserializing record: {:?}", record);
                            match record.deserialize(Some(&header)) {
                                Ok(stock_price) => {
                                    // if reader is OK
                                    info!("Deserialized record: {:?}", &stock_price);
                                    // populate the StockPrices struct and push it to the vector
                                    stock_prices.push(stock_price);
                                }

                                Err(e) => error!("error deserializing record: {:?} - {} ", record, e),
                            }
                        }
                        Err(e) => {
                            error!("error parsing CSV record: {}", e);
                        }
                    }
                }
            }
            // else report file not found
            Err(e) => {
                error!("error reading csv file: {} - error {}", &file_name, e);
            }
        }
        Ok(stock_prices)
    }

}

#[cfg(test)]
mod test{
    use chrono::NaiveDate;
    use crate::prices::{read_price,find_avg_price_for_dates,find_price_for_date};
    #[test]
    fn test_read_price() {
        let file_name = "/Users/shrisakrikar/Documents/data/AAPL.csv";
        let stock_prices = read_price(&file_name).unwrap();
        assert!(!stock_prices.is_empty());
    }
    #[test]
    fn find_avg_price_for_date(){
        let file_name = "/Users/shrisakrikar/Documents/data/AAPL.csv";
        let stock_prices = read_price(&file_name).unwrap();
        let from_date = NaiveDate::from_ymd_opt(2025,1,1).unwrap();
        let to_date = NaiveDate::from_ymd_opt(2025,3,31).unwrap();
        let price = find_avg_price_for_dates(&stock_prices, from_date,to_date );
        assert!(price.is_some());
    }
    #[test]
    fn no_price(){
        let file_name = "/Users/shrisakrikar/Documents/data/AAPL.csv";
        let stock_prices = read_price(&file_name).unwrap();
        let from_date = NaiveDate::from_ymd_opt(2025,1,1).unwrap();
        let price = find_price_for_date(&stock_prices,from_date);
        assert!(price.is_none());
    }

    #[test]
    fn find_price(){
        let file_name = "/Users/shrisakrikar/Documents/data/AAPL.csv";
        let stock_prices = read_price(&file_name).unwrap();
        let from_date = NaiveDate::from_ymd_opt(2025,1,2).unwrap();
        let price = find_price_for_date(&stock_prices,from_date);
        assert_eq!(price.unwrap().close_price, 243.85);
    }

}


