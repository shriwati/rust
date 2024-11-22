use std::ptr::read;

mod read_csv;



fn main() {
    let relative_path = "/Users/shrisakrikar/Documents/data/boston_house.csv";
    let reader = read_csv::read_file(relative_path);
    for line in reader{
        println!("{:?}",line);
    }
    // if let Ok(line) = reader {
    //     println!("{:?}",line);
    // }
    //
}
