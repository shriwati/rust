use std::env;
use read_csv::read_file;
use rail_fence::rail_fence::encrypt;

fn main() {
    match read_file("./src/main.rs") {
        Ok(reader) => {
            println!("Encrypting file ---");
            for each_line in reader.into_iter() {
                println!("{}", each_line.to_string());
                let encrypted_string = encrypt(&each_line,4);
                println!("{}", &encrypted_string);
                println!("----------------------------------------------------------");
            }
        }
        Err(e) => {
            println!("{}",e.to_string());
        }
    };

}
