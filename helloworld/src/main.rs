use std::io;
use std::io::Read;

fn main(){

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("empty number");

    let number:i32 =number.trim().parse().expect("not a number");

    println!("number was {number}");


}