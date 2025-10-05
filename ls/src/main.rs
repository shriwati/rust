use std::env::args;
use std::fs;

fn main() {
    let dir_name = args().skip(1).take(1);
    for name in dir_name {
        print_directories(&name);
    }
}

fn print_directories(dir_name: &str) {
    println!("listing directories under - {}", &dir_name);
    if let Ok(entries) = fs::read_dir(dir_name) {
        for entry in entries {
            let dir = entry.unwrap();
            let dir_type = dir.file_type().unwrap();
            if dir_type.is_dir() {
                println!("{:?}", dir.path());
            }
        }
    } else {
        println!("No directory found under {}", dir_name);
    }
}
