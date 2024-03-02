mod gcd;
mod collatz_seq;
mod matrix;
mod magnitude;

use crate::magnitude::{magnitude, normalize};

fn incr(n:&mut i32){
    *n +=1
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}


fn main() {

    println!("Magnitude of a unit vector: {}", magnitude(&vec![0.0, 1.0, 0.0]));
    let mut v = vec![1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

}
