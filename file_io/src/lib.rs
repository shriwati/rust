use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub struct FileInfo {
    name: String,
    data: Option<Vec<String>>,
}

impl FileInfo {
    pub fn new(name: String) -> Self {
        FileInfo { name, data: None } // moved here
    }
    fn open_file(file_name: &str) -> io::Result<File> {
        let f = File::open(&file_name);
        f
    }

    pub fn read_file(&self, file_type: &str) -> std::io::Result<()> {
        let _ = match file_type {
            "c" => {
                self.read_text_file()?;
            }
            "b" => {
                self.read_bytes();
            }
            "l" => {
                self.read_lines();
            }

            _ => (),
        };

        Ok(())
    }
    fn read_text_file(&self) -> std::io::Result<()> {
        let f = Self::open_file(&self.name);
        let mut data = String::new();
        let _ = f.unwrap().read_to_string(&mut data)?;
        println!("Contents of the file '{}' were {:?}", &self.name, &data);

        Ok(())
    }
    fn read_bytes(&self) -> std::io::Result<()> {
        let f = Self::open_file(&self.name);
        let mut data = Vec::new();
        let _ = f.unwrap().read_to_end(&mut data)?;
        println!("Contents of the file '{}' were {:?}", &self.name, &data);

        Ok(())
    }
    fn read_lines(&self) {
        let f = Self::open_file(&self.name);

        let f = BufReader::new(f.unwrap());
        println!("Contents of the file '{}' were", &self.name);
        for line in f.lines() {
            let chars: Vec<char> = line.unwrap().chars().filter(|x| *x == 'a').collect();
            for c in chars {
                println!("{}", c);
            }
        }
    }
}

#[test]
fn open_cargo_file() {
    let f = FileInfo::new("Cargo.toml".to_string());
    let _ = f.read_file(&"l");
}
