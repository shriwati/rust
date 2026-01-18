use csv::StringRecord;
use crate::prices::StockPrices;
pub mod prices {
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    mod date_format {
        use chrono::NaiveDate;
        use serde::{self, Deserialize, Deserializer, Serializer};

        const FORMAT: &str = "%m/%d/%y";

        pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = date.format(FORMAT).to_string();
            serializer.serialize_str(&s)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct StockPrices {
        name: String,
        #[serde(with = "date_format")]
        trade_date: NaiveDate,
        open_price: f64,
        high_price: f64,
        low_price: f64,
        close_price: f64,
        volume: u64
    }

    impl StockPrices {
        pub fn new(
            name: String,
            trade_date: NaiveDate,
            open_price: f64,
            high_price: f64,
            low_price: f64,
            close_price: f64,
            volume: u64
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
}


pub fn read_csv(file_name: &str) {
  let full_path = std::path::Path::new(file_name);
    println!("{:?}", full_path);
    let file = csv::Reader::from_path(full_path);
    let header = StringRecord::from(vec![
        "name","trade_date","open_price",
        "high_price","low_price","close_price","volume"
    ]);
    //Date,Open,High,Low,Close,Volume
    match file {
        Ok(mut reader) => {
            for result in reader.records() {
                match result {
                    Ok(record) => {
                        let record:StockPrices = record.deserialize(Some(&header)).unwrap();
                            println!("{:?}", record);
                    },
                    Err(e) => println!("Error parsing CSV record: {}", e),
                }
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    };

}