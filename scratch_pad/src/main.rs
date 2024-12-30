use std::process::exit;

fn main() {

    let mut input = String::new();

    loop {
        std::io::stdin().read_line(&mut input).unwrap();

        if &input.trim().to_lowercase() == "quit" {
            println!("{}", input);
            exit(1)
        }
        else {
            println!("{}", input);
            input.clear();
        }
    }


}
