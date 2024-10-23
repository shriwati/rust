use std::arch::aarch64::uint32x2_t;
use std::io::Error;

fn main() {

    let temp=fib_series(10);
    let s2;
    match temp {
        Ok(temp)=>println!("{:?}",temp),
        _=>println!("Error")
    };

    let fib_series = fib_series(10);
    println!("{:?}",fib_series.unwrap());

}

fn fib_series(num:usize)-> Result<Vec<u32>,Error>{
    let mut list:Vec<u32>=vec![];
    list.push(0);
    list.push(1);
    for j in 2..num+1{
        list.push(list[j-1] + list[j-2]);
    }
    Ok(list)
}

