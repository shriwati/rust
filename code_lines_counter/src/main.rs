// 1. list all the files from the specified directory with specified extension
/*
*/
use std::io::BufRead;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please specify the directory");
        return;
    }
    let dir_name = &args[1];
    println!("Looking for *.rs files in {}", &dir_name);
    println!("--------------------------------------");
    let list_of_files = get_list_of_files(&dir_name, "rs");
    for entry in list_of_files {
        println!("File '{}' has {} lines", &entry,read_lines_from_the_file(&entry));
    }
}
fn read_lines_from_the_file(file_name: &str)->usize {
    let file = std::fs::File::open(file_name).unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap());
    let mut code_lines:Vec<String> = vec![];
    for line in lines {
        if ! line.starts_with("//") 
            && ! line.is_empty() 
            && ! line.starts_with("/*")
            && ! line.starts_with("*/") 
        {
            code_lines.push(line);
        }
    }
    code_lines.len()
}
