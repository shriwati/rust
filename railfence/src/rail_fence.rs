pub mod rail_fence {
    pub fn encrypt(text: &str, key: usize) -> String {
        let mut char_array = vec![vec![' '; text.len()]; key];

        let mut j = 0;
        let mut i = 0;
        let mut down: bool = false;
        for c in text.chars() {
            if i == 0 || i == key - 1 {
                down = !down;  // change the direction
            }
            char_array[i][j] = c;
            println!("{}", c);
            if down {
                i += 1
            } else {
                i -= 1;
            }

            j += 1;
        }
        let encrypted_msg: String = char_array
            .iter()
            .flatten()
            .filter(|c| { !c.is_whitespace() }).collect();
        encrypted_msg
    }

    pub fn decrypt(text: &str, key: u8) {
        let cols = text.len();
        let mut char_array = vec![vec![false; cols]; key as usize];
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut down: bool = false;

        for _c in text.chars() {
            if i == 0 || i == (key - 1) as usize {
                down = !down;
            }
            char_array[i][j] = true;

            if down {
                i += 1
            } else {
                i -= 1;
            }

            j += 1;
        }
        let mut index = 0;
        let mut decrypted_msg: Vec<(usize, char)> = Vec::new();

        for i in 0..key as usize {
            for j in 0..cols {
                if char_array[i][j] == true {
                    decrypted_msg.push((j, text.chars().nth(index).unwrap()));
                    index +=1;
                }


            }

        }
        // Sort by original position
        decrypted_msg.sort_by_key(|&(pos, _)| pos);


        println!("{:?}",decrypted_msg);
    }
}
