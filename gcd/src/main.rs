mod gcd;

use std::time::Instant;

fn main() {
    println!("calculating GCD - recursion and loop");

    let n = Instant::now();
    println!("start time {:?}",Instant::now() );
    let gcd = gcd::gcd_recur(81,153);
    let m =Instant::now();
    println!("gcd is {:?}, time taken {:?}", gcd, m-n);


    let n = Instant::now();
    println!("start time {:?}",Instant::now() );
    let gcd = gcd::gcd_non_recurssion(81,153);
    let m =Instant::now();
    println!("gcd is {:?}, time taken {:?}", gcd, m-n);


}

