
use std::io::{ BufRead};


#[derive(Default,Debug)]
pub struct Count{
    lines:usize,
    words:usize,
}

impl Count{
    pub fn lines(&self) -> usize{
        self.lines
    }
    pub fn words(&self) -> usize{
        self.words
    }
}
pub fn count_lines_and_words<T: BufRead>(mut input: T) ->Result<Count, std::io::Error> {
    let mut line = String::new();

    let mut count = Count::default();
    while input.read_line(&mut line)? > 0 {
         count.lines +=1;
         count.words += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}



// pub fn count_words<T: BufRead>(mut input: T) ->Result<usize, std::io::Error> {
//     let mut line=String::new();
//     let mut words = 0;
//
//     while input.read_line(&mut line)? > 0 {
//         words += line.split_whitespace().count();
//         line.clear();
//     }
//     Ok(words)
// }


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{count_lines_and_words};
    #[test]
    fn test_count_lines() {
        let lines= Cursor::new("input1\ninput2\ninput3\ninput4\ninput5\n");
        let count = count_lines_and_words(lines).unwrap();
        assert_eq!(count.lines, 5, "Line count failed to match");

    }
    #[test]
    fn test_count_lines_zero() {
        let lines= Cursor::new("input\n");
        let count = count_lines_and_words(lines).unwrap();
        assert_ne!(count.lines, 0, "Line count failed to match");
    }

    #[test]
    fn test_count_words() {
        let lines = Cursor::new("word1 word2 word 3\n");
        let count = count_lines_and_words(lines).unwrap();
        assert_eq!(count.words,4, "Words count failed to match");
    }
    #[test]
    fn test_count_words_zero() {
        let lines = Cursor::new("");
        let count = count_lines_and_words(lines).unwrap();
        assert_eq!(count.words, 0, "Words count failed to match");
    }
}