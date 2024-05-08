use std::error::Error;

fn divide_by_zero(numerator: i32, denominator: i32) -> Result<f64, &'static str> {
    if denominator == 0 {
        return Err("Can not divide by zero");
    }
    Ok((numerator / denominator) as f64)
}

fn main() {
    let result = divide_by_zero(100, 10);

    match result {
        Ok(result) => println!("Answer is {}", result),
        Err(error) => {
            println!("Error occurred {},--source", error.source())
        },
    }
}
