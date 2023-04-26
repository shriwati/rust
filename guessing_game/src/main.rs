use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {secret_number}");

    println!("Geuss the number!");

    println!("Please input your guess");



    loop {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=> continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less than secret.."),
            Ordering::Equal => {println!("You win.."); break;}
            Ordering::Greater => println!("Greater than")
        }
    }

}
