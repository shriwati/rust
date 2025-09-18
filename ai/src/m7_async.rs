use std::io::{Error, ErrorKind};
use std::str::FromStr;
use tokio;
use serde_json;
use reqwest;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize,Debug)]
pub struct Currency{
    id:String,
    name:String,
    min_size:String,
}
//
// pub async fn get_jsondata(url:&str)->Result<Vec<Currency>,reqwest::Error>{
//     let json_response = reqwest::get(url)
//         .await?
//         .json::<serde_json::Value>()
//         .await?;
//     let currencies:Vec<Currency> = serde_json::from_value(json_response).unwrap();
//     Ok(currencies)
// }

pub async fn get_jsondata(url:&str)->Result<Value,Error>{
    let response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(
            std::io::ErrorKind::Other,
            "Error in get_jsondata - could not receive response from API"
        ))?
        .json::<Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other,"Could not parse JSON object"))?;
    
    Ok(response)
}

#[cfg(test)]

mod tests {
    use super::*;
    
    #[tokio::test]
    async fn async_function_test(){
        dbg!("hello");
        let url="https://api.coinbase.com/v2/currencies";
        let response = get_jsondata(&url).await;

        let response = response.unwrap()["data"].to_string();
        let arr:Vec<Currency> =serde_json::from_str(&response).unwrap();

        for item in arr{
            println!("ID:{}, Name:{}, Min Size:{}",item.id,item.name,item.min_size);
        }


        }

    }
    
