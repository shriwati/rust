
use std::{fs::File,io::Write};
// use std::env::Args;
use std::io::BufRead;
use anyhow::{Context, Result};
use time::OffsetDateTime;
pub fn write_log(msg: &str) -> Result<()> {
    let mut f = File::options()
        .append(true)
        .create(true)
        .open("log.txt").with_context(|| "failed to open log.txt")?;
    let _ = f.lock();

    let now = OffsetDateTime::now_utc();
    // writeln!(f, "{:?}\t\t{:?}", now,msg)?;
    let _ = f.write_all(format!("{}\t{}\n",now,msg).as_bytes());
    let _ = f.unlock();
    Ok(())
}
pub fn read_log()->Result<Vec<String>> {
    let mut result = Vec::new();

    let f = File::open("log.txt").with_context(|| "failed to open log.txt")?;
    for line in std::io::BufReader::new(f).lines() {
        result.push(line?);
    }
    Ok(result)
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::fs;
    use std::io::prelude::*;
    use anyhow::Context;


    #[test]
    fn test_write_file() -> Result<()> {
        write_log("Hello, world!")?;
        Ok(())
    }
    #[test]
    fn test_read_file() -> Result<()> {
        let mut f = fs::File::open("test.txt")?;
        let mut contents = String::new();
        let _ =f.read_to_string(&mut contents).with_context(|| "Failed to read Cargo.toml")?;
        assert!(contents.contains("Hello,"));
        Ok(())
    }

    #[test]
    fn test_read_file_fail() -> Result<()> {
        let _f = fs::File::open("test.txt.doesnotexist").with_context(|| "Failed to open Cargo.toml");
        // let mut contents = String::new();
        // f.read_to_string(&mut contents).with_context(|| "Failed to read Cargo.toml");
        //
        Ok(())
    }


}

