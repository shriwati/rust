use r2_file_read::FileInfo;
use clap::{Parser};
#[derive(Debug)]
struct cmd_param{

}
fn main() {
    // let f = FileInfo::new("./cargo.toml".to_string());
    // let _ = f.read_file(&"l");

    let char_vec = ('a'..'働').collect::<Vec<char>>();
    println!("{:?}",char_vec.iter().any(|&char| char=='뷁'));
}