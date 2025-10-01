
pub mod rail_fence {
    pub fn encrypt_text(text: &str, key: usize) -> String {
        let mut char_array = vec![vec!['ł'; text.len()]; key];
        let mut j = 0;
        let mut i = 0;
        let mut direction:i32 = -1;

        // if key ==0 then nothing to do
        if key as i32 == 0 {
            return text.to_string();
        }

        for c in text.chars() {  // for each char in string
            if i == 0 || i == key - 1 {     // if we reached end of row index, reverse the direction
                direction = direction * - 1;
            }
            char_array[i][j] = c;       // add char to the vector
            i = ( i as i32 +  direction) as usize;
            j += 1;
        }

        // filter out special character "ł"
        // flatten out rows and append
        let encrypted_msg: String = char_array
            .iter()
            .flatten()
            .filter(|c| { c.to_string()!="ł"})
            .collect();
        encrypted_msg
    }

    pub fn decrypt_text(text: &str, key: u8)->String {
        let cols = text.len();
        let mut char_array = vec![vec![false; cols]; key as usize];
        let mut decrypted_string=vec![String::new(); key as usize];

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut direction:i32 = -1;
        let mut index = 0;
        let mut decrypted_msg: Vec<(usize, char)> = Vec::new();

        // if key ==0 then nothing to do
        if key as i32 == 0 {
            return text.to_string();
        }

        for _c in text.chars() {    // for each char in encrypted string
            if i == 0 || i == (key - 1) as usize {
                direction = direction * - 1;
            }
            char_array[i][j] = true;
            i = ( i as i32 +  direction) as usize;
            j += 1;
        }
        // Create correct position based on the vector position
        for i in 0..key as usize {
            for j in 0..cols {
                if char_array[i][j] == true {
                    decrypted_msg.push((j, text.chars().nth(index).unwrap()));
                    index +=1;
                }
            }
        }

        // Sort by original positions
        decrypted_msg.sort_by_key(|&(pos, _)| pos);
        // extract char from the sorted tupple and build a string
        decrypted_msg.iter().for_each(|&(_pos, c)| {
                decrypted_string.push(c.to_string());
        });

        decrypted_string.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::{rail_fence::encrypt_text};

    #[test]
    fn encrypt_test() {
        let encrypt_string = encrypt_text("HELLO WORLD ", 3);
        assert_eq!(encrypt_string, "HOREL OL LWD");
    }
    #[test]
    fn decrypt_test() {
        use super::{rail_fence::decrypt_text};
        let decrypt_test = decrypt_text("HOREL OL LWD", 3);
        assert_eq!(decrypt_test, "HELLO WORLD ");
    }

}