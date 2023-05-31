use std::io;
use std::io::{Read, stdin};

fn main(){


    println!(" Temp Conversion (C) to F OR (F) to C and E(X)it. Please make a choice");
    let stdin = io::stdin();
    loop {
        let mut choice = String::new();

        stdin.read_line(&mut choice).expect("TODO: panic message");
        if choice.trim() == "X" {
            println!("Exiting");
            break;
        }
        match choice.trim() {
            "C" => {
                let temp = stdin.read_line( temp).unwrap();
                println!("please enter temp in celcius to convert to farenheight");
                println!("{}",c_to_f(temp.parse().unwrap()))
            },
            "F" => {
                println!("please enter temp from farenheight to celcius");
                let mut  temp = stdin.read_line(&mut temp).unwrap();
                println!("{}",c_to_f(choice.parse().unwrap()))
            },
            _ => {},
        }
    };
}
fn f_to_c(f: f32) ->f32{
    //T(°C) = (68°F - 32) × 5/9 = 20 °C
    (f - 32.0) * (5.0/9.0)
}
fn c_to_f(c: f32) ->f32{
    //T(°F) = T(°C) × 9/5 + 32
    (c * 9.0/5.0)+32.0
}