/*
this program reads one or more files text ( max 5) and combines them.

-- read arguments from the program
-- read each file
-- append to the vector
--- display entire vector

*/

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {

    let arg = env::args().skip(1);
    if arg.len()<=0 {
        println!("no input");
        std::process::exit(132);
    }else {

        println!("###############");
        for filename in arg {
            println!("{filename}");
            let mut data = String::new();
            read_file_return_string(&filename, &mut data);

            println!("{data}");
            println!("###############");
        }


    }

}

fn read_file_return_string(file_name:&str,data:&mut String){
    let  f = File::open(&file_name);
    let _ = f.unwrap().read_to_string(data);
}
