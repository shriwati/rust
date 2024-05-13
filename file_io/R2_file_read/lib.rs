use std::fs::File;
use std::io;
use std::io::Read;
use clap::Parser;


struct FileInfo {
    name: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short)]
    f: String,
    #[arg(short)]
    t: String,
}
impl FileInfo {
    pub fn new(name: String) -> Self {
        FileInfo { name } // moved here
    }
    fn open_file(file_name: &str) -> io::Result<File> {
        File::open(&file_name)
    }


    pub fn read_file(&self, file_type:&str) {


        let _ = match file_type {
            "c" => {
                println!("Reading file..'{}' as text.", &self.name);
                self.read_text_file();
            }
            "b" => {
                println!("Reading file..'{}' as bytes.", &self.name);
                self.read_bytes();
            }
            _ => (),
        };


        let f = Self::open_file(&self.name);
        let mut data = String::new();
        // read string
        let _ = f.unwrap().read_to_string(&mut data);
        println!("Contents of the file '{}' were {:?}", &self.name, &data);


    }
    fn read_text_file(&self) {
        let f = Self::open_file(&self.name);
        let mut data = String::new();
        // read string
        let _ = f.unwrap().read_to_string(&mut data);
        println!("Contents of the file '{}' were {:?}", &self.name, &data);
    }
    fn read_bytes(&self) {
        // open the file for read only
        // read contents in to a vector
        // return readonly FileInfo
        println!("reading a file {}", &self.name);
        let f = Self::open_file(&self.name);
        // read bytes
        let mut data = Vec::new();
        let _ = f.unwrap().read_to_end(&mut data);
        println!("Contents of the file '{}' were {:?}", &self.name, &data)
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
