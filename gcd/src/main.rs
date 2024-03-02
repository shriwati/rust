mod gcd;
mod collatz_seq;
mod matrix;



use std::time::Instant;
fn incr(n:&mut i32){
    *n +=1
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}



fn main() {

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = matrix::transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
