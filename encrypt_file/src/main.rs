use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};
use read_csv::read_file;
use rail_fence::rail_fence::{decrypt, encrypt};

fn main() {
    let file_name_with_path="/Users/shrisakrikar/Documents/data/pp-complete.csv";
    let encrypted_file_name_with_path="/Users/shrisakrikar/Documents/data/pp-complete.encrypted";
    // let decrypted_file_name_with_path="/Users/shrisakrikar/Documents/data/pp-complete.csv.decrypted";
    // encrypt file
    let t = Instant::now();
    match read_file(&file_name_with_path) {
        Ok(reader) => {
            println!("Encrypting file ---");
            let mut encrypted_file = File::create(Path::new(&encrypted_file_name_with_path)).unwrap();

            for each_line in reader.into_iter() {
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
    println!("File encrypted -- {} in {} sec",&encrypted_file_name_with_path,t.elapsed().as_secs().to_string());


    // decrypt file
    // match read_file(&encrypted_file_name_with_path) {
    //     Ok(reader) => {
    //         println!("Decrypting file ---");
    //         let mut decrypted_file = File::create(Path::new(&decrypted_file_name_with_path)).unwrap();
    //
    //         for each_line in reader.into_iter() {
    //             let mut decrypted_string = decrypt(&each_line,4);
    //             decrypted_string.push_str("\n");
    //             decrypted_file.write(decrypted_string.as_bytes()).unwrap();
    //         }
    //         decrypted_file.flush().unwrap();
    //     }
    //     Err(e) => {
    //         println!("{}",e.to_string());
    //     }
    // };
    // println!("File decrypted -- {}",&decrypted_file_name_with_path);

}
