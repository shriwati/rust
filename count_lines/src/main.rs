use std::{ fs::File, io::{BufReader}};
use std::path::Path;
use anyhow::{Result, Context,bail};
use count_lines::count_lines_and_words;
use clap::{Parser};

#[derive(Parser,Default,Debug)]
struct Args{
    /// counts words instead of lines
    #[clap(short,long)]
    words: bool,
    /// files to be scanned for line/word counts
    #[clap(required = true)]
    files:Vec<String>

}
fn main() -> Result<()>{

    let args = Args::parse();
    // read arguments
    for path in args.files {
        let p = Path::new(&path);
        let reader = BufReader::new(File::open(&path)?);
        let count = count_lines_and_words(reader).context(p.display().to_string())?;
        if args.words{
            println!( "{:#?} file has - {:#?} words",&p,count.words());
        }else {
            println!( "{:#?} file has - {:#?} lines and {:#?} words",&p,count.lines(),count.words());
        }

    }
    Ok(())
}