use std::time::Instant;

fn main() {
    println!("calculating GCD - recursion and loop");

    let n = Instant::now();
    println!("start time {:?}",Instant::now() );
    let gcd = gcd_recur(81,153);
    let m =Instant::now();
    println!("gcd is {:?}, time taken {:?}", gcd, m-n);


    let n = Instant::now();
    println!("start time {:?}",Instant::now() );
    let gcd = gcd_non_recurssion(81,153);
    let m =Instant::now();
    println!("gcd is {:?}, time taken {:?}", gcd, m-n);


}


fn gcd_non_recurssion(a:u32,b:u32)->u32{
    let mut _gcd = 0;
    let mut _temp = 0;
    let mut _a = a;
    let mut _b = b;
    while _b > 0 {
        _temp = _a % _b;
        if _temp >0 {
            _gcd = _temp;
            _a = _b;
            _b = _gcd;
        }
        else{
            break
        }
    }
    _gcd
}

fn gcd_recur(a:u32,b:u32)-> u32 {
    if b > 0{
        gcd_recur(b, a % b )
    }else {
        a
    }
}