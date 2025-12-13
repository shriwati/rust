use logbook::{read_log, write_log};

fn main() {
    let _result = write_log("hello world");
    match _result {
        Ok(_result) => {
            println!("log was written to stdout");
        }
        Err(error) => {
            println!("log was failed: {:?}", error);
        }
    }
    let lines = read_log();
    for line in lines {
       match &line {
           &_ => { println!("{:?}",&line);   }
       }
    }
}
