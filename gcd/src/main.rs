mod gcd;
mod collatz_seq;
mod matrix;
mod magnitude;
mod elevator;

use crate::elevator::{Direction, Elevator};
use crate::magnitude::{magnitude, normalize};

fn main() {

    // let i = Cell::new(10);
    //
    // println!("Cell is {}",i.get());
    // i.set(15);
    // println!("Cell is {}",i.get());
    let mut ele = Elevator::start();
    ele.move_elevator(Direction::Up,5);
    ele.move_elevator(Direction::Down,4);
    ele.elevator_is_on();

    println!("Magnitude of a unit vector: {}", magnitude(&vec![0.0, 1.0, 0.0]));
    let mut v = vec![1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

}
