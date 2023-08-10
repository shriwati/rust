use std::ops::Add;
use time::macros::{date, datetime, time};
use time::PrimitiveDateTime;
use std::time::Duration;


fn main() {
    let mut dt = PrimitiveDateTime::new(date!(2023-05-30),time!(0:00));
    println!("{:?}",  dt);
    let d = Duration::from_secs(100_000_000);
    dt = dt.add(d);
    println!("{:?}",  dt);
}

