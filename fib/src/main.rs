use clap::{Parser};
#[derive(Parser)]
struct Fib{
    #[clap(short='f',long,value_parser, default_value="5")]
    fib:usize
}
fn main() {


    let fib = Fib::parse();

    println!("Fib number is {}",fib.fib);
    let fib_num= fib.fib;

    // let fib_num=30;
    let mut fib_series = fib_series(fib_num);
    println!("Fibonacci series is {:?}",fib_series);
    let num = fib_series.pop().unwrap() as i32;
    println!("Fibonacci number for {} is {:?}",fib_num, num)

}
fn fib_series(n:usize)->Vec<u32>{
    let mut fib_series:Vec<u32>=vec![];
    fib_series.push(0);
    fib_series.push(1);

    for i in 2..n+1{

        fib_series.push(fib_series[i-1] + fib_series[i-2]);

   }
    fib_series
}
