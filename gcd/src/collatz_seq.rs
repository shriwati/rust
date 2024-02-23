use crate::collatz_seq;

pub fn collatz_seq(mut num:u32) ->u32{
    // generates the collatz sequence
    let mut seq:Vec<u32>=vec![];
    while(num !=1){
        match num % 2 {
            0 => { num  = num / 2 }
            _=>{ num = num * 3 +1}
        };
        seq.push(num);
    }
    seq.len() as u32

}

#[test]
fn test_collatz_length() {
    assert_eq!( collatz_seq::collatz_seq(3), 7);
}