use std::io::{stdin, BufRead};

fn main() {

    let input = stdin().lock();
    let count = input.lines().count();
    println!("{} lines", count);
    
}
