use std::ptr::read;

mod read_csv;



fn main() {
    let relative_path = "/Users/shrisakrikar/Documents/data/pp-complete.csv";
    let reader = read_csv::read_file(relative_path);
    for line in reader{
        println!("{:?}",line);
    }

}
