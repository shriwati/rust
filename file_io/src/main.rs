use std::{fs, io};
use std::fs::File;
use std::io::{Read, stdin};

#[derive(Debug)]
struct FileInfo {
    name: String,
    file_type: String,
}

impl FileInfo {
    pub fn new(name: String, file_type: String) -> Self {
        FileInfo { name, file_type } // moved here
    }
    fn open_file(file_name: &str) -> io::Result<File> {
            File::open(&file_name)
    }

    pub fn read_text_file(&self) {
        println!("Reading file..{}", &self.name);
        let f = Self::open_file(&self.name);
        let mut data=String::new();
        // read string
        let _  = f.unwrap().read_to_string(&mut data);
        println!("Contents of the file '{}' were {:?}",&self.name, &data);
    }

    pub fn read(&self) -> std::io::Result<()> {
        // open the file for read only
        // read contents in to a vector
        // return readonly FileInfo
        println!("reading a file {}", &self.name);
        let f = Self::open_file(&self.name);
        // read bytes
        let mut data = Vec::new();
        let _ = f?.read_to_end(&mut data);
        println!("{:?}", &data);
        Ok(())
    }
}

fn main() {
    println!("Please type full path and file name to read");

    let mut line = &mut String::new();
    let _  = stdin().read_line(&mut line);
    loop {
        if &line.len() > &(0 as usize) {
            break;
        } else {
            line.clear();
            let _ = stdin().read_line(&mut line);
        }
    }
    println!("Input was {}", &line);

    if line.len() > 1 {
        let filename:String= line.trim().clone().to_string();
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{:?}", entry.path());
                }
            }
        }

        let f = FileInfo::new(filename, "csv".to_string());
        f.read_text_file();
    }
}
