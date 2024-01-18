use std::io::stdin;

fn main() {
    /*
        read input
        if input is X then quit
        else println (input)

    */

    println!("This will print your input again, until you type X, enjoy!");
    let mut x = String::new();
    stdin().read_line(&mut x).unwrap();

     while &x.trim().to_string()!="X" {

         println!("Your input was - {}",&x);
         x.clear();
         stdin().read_line(&mut x).unwrap();

    }

    println!("Sorry, you decided to leave !");
}
