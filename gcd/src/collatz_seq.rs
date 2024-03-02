use crate::collatz_seq;

pub fn collatz_seq(mut num:u32)-> Vec<u32>{
    // generates the collatz sequence
    let mut seq:Vec<u32>=vec![];
    if num > 0 {
        seq.push(num);
        while num > 1 {
            match num % 2 {
                0 => { num = num / 2 }
                _ => { num = num * 3 + 1 }
            };
            seq.push(num);
        }
    }
   seq

}

#[test]
fn test_collatz_length() {
    assert_eq!( collatz_seq::collatz_seq(11).len(),15);
}