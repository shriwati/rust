use rail_fence::rail_fence::{decrypt, encrypt};
use read_csv::read_file;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time:: Instant;

fn main() {
    let file_name_with_path = std::env::args()
        .skip(1)
        .next()
        .expect("No file with path provided");
    let encrypted_file_name_with_path: String = String::from(&file_name_with_path) + ".enc";

    //encrypt file

    let t = Instant::now();
    match read_file(&file_name_with_path, Option::from(None)) {
        Ok(reader) => {
            println!("Encrypting file ---");
            let mut encrypted_file =
                File::create(Path::new(&encrypted_file_name_with_path)).unwrap();

            for each_line in reader.into_iter() {
                let mut encrypted_string = encrypt(&each_line, 4);
                encrypted_string.push_str("\n");
                encrypted_file.write(encrypted_string.as_bytes()).unwrap();
            }
            encrypted_file.flush().unwrap();
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    };
    println!(
        "File encrypted -- {} in {} sec",
        &encrypted_file_name_with_path,
        t.elapsed().as_secs().to_string()
    );

    let decrypted_file_name_with_path: String = String::from(&file_name_with_path) + ".dec";
    //decrypt file
    match read_file(&encrypted_file_name_with_path, None) {
        Ok(reader) => {
            println!("Decrypting file ---");
            let mut decrypted_file =
                File::create(Path::new(&decrypted_file_name_with_path)).unwrap();

            for each_line in reader.into_iter() {
                let mut decrypted_string = decrypt(&each_line, 4);
                decrypted_string.push_str("\n");
                decrypted_file.write(decrypted_string.as_bytes()).unwrap();
            }
            decrypted_file.flush().unwrap();
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    };
    println!("File decrypted -- {}", &decrypted_file_name_with_path);
}
