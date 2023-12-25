use std::{env, fs};
use regex::Regex;

#[derive(Debug)]
struct Arguments{
    target:String,
    replacement:String,
    file_name:String,
    output_filename:String
}

fn main() {

    let args = read_arg();
    println!("Arguments are {:?}",args);

    let data = read_file(&args.file_name);

    let replace_data = match replace(&args.target,&args.replacement,&data.trim_end()) {
        Ok(v)=>v,
        Err(_e)=>{
            eprintln!("Match {} not found",&args.target);
            std::process::exit(1);
        }
    };
    write_file(&args.output_filename,&replace_data);
}

fn read_file(filename:&str )->String{
    let data = match fs::read_to_string(&filename) {
        Ok(v)=>v,
        Err(_e)=>{
            eprintln!("Failed to read the file {}",&filename);
            std::process::exit(1)
        }
    };
    data
}
fn write_file(filename:&str,data:&str ){
    match fs::write(&filename,&data) {
        Ok(_)=>{},
        Err(_e)=>{
            eprintln!("Failed to write the file {}",&filename);
            std::process::exit(1);
        }
    }

}
fn replace(target:&str,replace:&str,text:&str)->Result<String,regex::Error>{
    let reg = Regex::new(target)?;
    Ok(reg.replace(text,replace).to_string())

}
fn read_arg()->Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        eprintln!("Wrong number of arguments passed.Need search_string, replacement_string
            ,file_to_search, file_output. Received only {}", args.len());
        std::process::exit(1);
    }

    Arguments{
        target:args[0].clone(),
        replacement:args[1].clone(),
        file_name:args[2].clone(),
        output_filename:args[3].clone()
    }

}

#[test]
fn test_replacement(){
    let args = Arguments{
        replacement:"World".to_string(),
        target:"Rust".to_string(),
        file_name:"hello.txt".to_string(),
        output_filename:"hello2.txt".to_string()
    };



}