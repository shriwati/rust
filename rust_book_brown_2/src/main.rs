use std::io;
use std::cmp::Ordering;
use std::process::exit;
use rand::Rng;

fn main() {

    let r =rand::thread_rng().gen_range(1..=100);


    loop {
        println!("Guess a number, {{101 to Quit}}");

        let mut guess = String::new();
        let _ = io::stdin().read_line(&mut guess).expect("Failed to read line");
        let num: u32 = guess.trim().parse().expect("Not a valid number");
        println!("Your input was {}", &num);
        if num == 101 {
            println!("Thank you for playing");
            exit(0)
        }
        match num.cmp(&r) {
            Ordering::Less => {
                println!("{} is less than  secret number",&num);
            }
            Ordering::Equal => {
                println!("Wow..you guesses it correct {}",&num);
                exit(0);
            }
            Ordering::Greater => {
                println!("{} is greater than  secret number",&num);
            }
        }
    }
}
