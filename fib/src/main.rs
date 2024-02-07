

fn main() {
    // let fib :u32=1;
    let fib_num=30;
    let mut fib_series = fib_series(fib_num);
    println!("Fibonacci series is {:?}",fib_series);
    println!("Fibonacci number for {} is {:?}",fib_num,fib_series.pop())
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