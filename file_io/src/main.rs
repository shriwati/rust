use std::f64::consts::E;
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::Read;

    struct FileInfo {
        name: String,
    }

    struct ElementOccurance{
        element:u8,
        occurance:Vec<usize>
    }

impl ElementOccurance {
    pub fn new(item: u8) ->Self{
            ElementOccurance{
                element:item,
                occurance:Vec::new()
            }
    }

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


        pub fn read_file(&self, file_type: &str) {
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
            let _ = f.unwrap().read_to_string(&mut data);
            println!("Contents of the file '{}' were {:?}", &self.name, &data);
        }
        fn read_bytes(&self) {
            let f = Self::open_file(&self.name);
            let mut data = Vec::new();
            let _ = f.unwrap().read_to_end(&mut data);
            let ref_data = &data;
            println!("Contents of the file '{}' were {:?}", &self.name, &data)
        }
        fn search_item(item: u8, data:&Vec<u8>, start_loc:usize,locations:&mut ElementOccurance) {
            // returns aLL the locations found for the item
            let size = data.len();
            for i in 0..size{
                if item == data[i]{
                    locations.occurance.push(i);
                }
            }

        }

    }



fn main() {
    // let cli = Cli::parse();
    // let filename = cli.f;
    let f = FileInfo::new("./cargo.toml".to_string());
    // f.read_file(&cli.t.to_ascii_lowercase());
    f.read_file(&"b");


}
