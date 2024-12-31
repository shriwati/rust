use std::fs::File;
use std::io::Write;
use std::path::Path;
use read_csv::read_file;
use rail_fence::rail_fence::{decrypt, encrypt};

fn main() {
    // encrypt file

    match read_file("./src/main.rs") {
        Ok(reader) => {
            println!("Encrypting file ---");
            let mut encrypted_file = File::create(Path::new("./src/encrypted.enc")).unwrap();

            for each_line in reader.into_iter() {
                println!("{}", each_line.to_string());
                let mut encrypted_string = encrypt(&each_line,4);
                encrypted_string.push_str("\n");
                encrypted_file.write(encrypted_string.as_bytes()).unwrap();
            }
            encrypted_file.flush().unwrap();
        }
        Err(e) => {
            println!("{}",e.to_string());
        }
    };

    // decrypt file
    match read_file("./src/encrypted.enc") {
        Ok(reader) => {
            println!("Decrypting file ---");
            let mut decrypted_file = File::create(Path::new("./src/decrypted.text")).unwrap();

            for each_line in reader.into_iter() {
                let mut decrypted_string = decrypt(&each_line,4);
                decrypted_string.push_str("\n");
                decrypted_file.write(decrypted_string.as_bytes()).unwrap();
            }
            decrypted_file.flush().unwrap();
        }
        Err(e) => {
            println!("{}",e.to_string());
        }
    };


}
