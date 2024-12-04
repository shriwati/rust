
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;


    pub fn read_file(filename_with_path: &str) -> std::io::Result<Vec<String>> {
        //get absolute path
        let abs_file_path = get_absolute_path(filename_with_path)?;
        let file = File::open(abs_file_path)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    fn get_absolute_path(path_str: &str) -> Result<String, std::io::Error> {
        let path = Path::new(path_str);
        let absolute_path = fs::canonicalize(path)?; // Resolve the absolute path
        Ok(absolute_path.to_string_lossy().to_string()) // Convert to string
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_read_file() {

        }
    }