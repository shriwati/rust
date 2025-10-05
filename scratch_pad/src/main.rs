// use std::process::exit;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug,PartialEq)]
struct Clock {
    h:i32,
    m: i32,
}

impl Clock {
    fn new(hour: i32, min: i32) -> Clock {
        let mut temp_h = hour;
        if hour> 24{
            temp_h = hour % 24;
        }
        let mut c = Clock {h:temp_h, m:0};
         let clock = c.add_minutes(min);
        Clock {h:clock.h, m:clock.m}
    }
    pub fn add_minutes(&mut self,min: i32) ->Clock {
        let mut temp_min = self.m + min;
        if temp_min> 59 {
            self.h += temp_min / 60;
            temp_min = temp_min % 60;
            if self.h >= 24 {
                self.h = self.h % 24;
            }
        }
        Clock { h: self.h, m:temp_min }
    }

    // pub fn to_string(&self) -> String {
    //     self.h.to_string()+":"+&self.m.to_string()
    // }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.h, self.m)
    }
}
type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

fn main() {

    let words = vec!["hello".to_string()];
    let d = new_document(words);

    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    // The modification to `d2` does not affect `d`
    println!("{:?}",d2);

    // let clock1 = Clock::new(-12, -268);
    // let clock2= Clock::new(7, 32);
    // let h = -12 % 24;
    // let m = -268 % 60;
    // let k = (h+m) % 24;
    // println!("{}", clock1.to_string());
    // println!("{}", clock2.to_string());
    //
    //
    // assert_eq!(clock1, clock2);
    //
    // println!("{}", clock1.to_string());
    // // assert_eq!(Clock::new(0, 1441), Clock::new(0, 1));
    //
    //
    // //assert_eq!(clock.to_string(), "10:03");
    //
    //
    // let c = Clock::new(10, 1);
    // println!("{}",c.to_string());
    // println!("{}", Clock::new(10, 3).to_string());
    // println!("{}", Clock::new(9, 11).to_string());
}
// fn main() {
//     let c: Clock = Clock { h: 8, m: 10 };
//     print!("{}", c.to_string());
//
//     // let v = vec![10,20,30,60,50,75];
//     //
//     // let window = 3;
//     //
//     // let moving_avg :Vec<_>= v.windows(window)
//     //     .map(|v| v.iter().sum::<i32>() / window as i32).collect();
//     //
//     // println!("{:?}",moving_avg);
//     //
//     //
//     // let mut h=HashMap::from([(0,Some(20.2)),(1,None)
//     //     ,(2,Some(5.4)),(3,Some(7.5)),(7,Some(3.2))]);
//     //
//     // input_missing_values(&mut h);
//
//     // let mut input = String::new();
//     //
//     // loop {
//     //     std::io::stdin().read_line(&mut input).unwrap();
//     //
//     //     if &input.trim().to_lowercase() == "quit" {
//     //         println!("{}", input);
//     //         exit(1)
//     //     }
//     //     else {
//     //         println!("{}", input);
//     //         input.clear();
//     //     }
//     // }
// }
// fn input_missing_values(data:&mut HashMap<usize,Option<f32>>){
//
//     data.iter_mut().for_each(|(_,v)| {
//         if v.is_none() {
//             *v = Some(0.0);
//         }
//     })
//
// }
// fn find_min<T:PartialOrd + Copy>(v: &[T]) -> Option<&T> {
//     v.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
//
// }

#[test]
fn test_on_the_hour() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
}

#[test]


fn test_past_the_hour() {
    assert_eq!(Clock::new(11, 9).to_string(), "11:09");
}

#[test]


fn test_midnight_is_zero_hours() {
    assert_eq!(Clock::new(24, 0).to_string(), "00:00");
}

#[test]


fn test_hour_rolls_over() {
    assert_eq!(Clock::new(25, 0).to_string(), "01:00");
}

#[test]


fn test_hour_rolls_over_continuously() {
    assert_eq!(Clock::new(100, 0).to_string(), "04:00");
}

#[test]


fn test_sixty_minutes_is_next_hour() {
    assert_eq!(Clock::new(1, 60).to_string(), "02:00");
}

#[test]


fn test_minutes_roll_over() {
    assert_eq!(Clock::new(0, 160).to_string(), "02:40");
}

#[test]


fn test_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
}

#[test]


fn test_hours_and_minutes_roll_over() {
    assert_eq!(Clock::new(25, 160).to_string(), "03:40");
}

#[test]


fn test_hours_and_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
}

#[test]


fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
    assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
}

#[test]


fn test_negative_hour() {
    assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
}

#[test]


fn test_negative_hour_roll_over() {
    assert_eq!(Clock::new(-25, 00).to_string(), "23:00");
}

#[test]


fn test_negative_hour_roll_over_continuously() {
    assert_eq!(Clock::new(-91, 00).to_string(), "05:00");
}

#[test]


fn test_negative_minutes() {
    assert_eq!(Clock::new(1, -40).to_string(), "00:20");
}

#[test]


fn test_negative_minutes_roll_over() {
    assert_eq!(Clock::new(1, -160).to_string(), "22:20");
}

#[test]


fn test_negative_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
}

#[test]


fn test_negative_sixty_minutes_is_prev_hour() {
    assert_eq!(Clock::new(2, -60).to_string(), "01:00");
}

#[test]


fn test_negative_one_twenty_minutes_is_two_prev_hours() {
    assert_eq!(Clock::new(1, -120).to_string(), "23:00");
}

#[test]


fn test_negative_hour_and_minutes_both_roll_over() {
    assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
}

#[test]


fn test_negative_hour_and_minutes_both_roll_over_continuously() {
    assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
}

#[test]


fn test_zero_hour_and_negative_minutes() {
    assert_eq!(Clock::new(0, -22).to_string(), "23:38");
}

//

// Clock Math

//

#[test]


fn test_add_minutes() {
    let clock = Clock::new(10, 0).add_minutes(3);

    assert_eq!(clock.to_string(), "10:03");
}

#[test]


fn test_add_no_minutes() {
    let clock = Clock::new(6, 41).add_minutes(0);

    assert_eq!(clock.to_string(), "06:41");
}

#[test]


fn test_add_to_next_hour() {
    let clock = Clock::new(0, 45).add_minutes(40);

    assert_eq!(clock.to_string(), "01:25");
}

#[test]


fn test_add_more_than_one_hour() {
    let clock = Clock::new(10, 0).add_minutes(61);

    assert_eq!(clock.to_string(), "11:01");
}

#[test]


fn test_add_more_than_two_hours_with_carry() {
    let clock = Clock::new(0, 45).add_minutes(160);

    assert_eq!(clock.to_string(), "03:25");
}

#[test]
fn test_add_across_midnight() {
    let clock = Clock::new(23, 59).add_minutes(2);

    assert_eq!(clock.to_string(), "00:01");
}

#[test]


fn test_add_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(1500);
    assert_eq!(clock.to_string(), "06:32");
}

#[test]


fn test_add_more_than_two_days() {
    let clock = Clock::new(1, 1).add_minutes(3500);

    assert_eq!(clock.to_string(), "11:21");
}

#[test]


fn test_subtract_minutes() {
    let clock = Clock::new(10, 3).add_minutes(-3);

    assert_eq!(clock.to_string(), "10:00");
}

#[test]


fn test_subtract_to_previous_hour() {
    let clock = Clock::new(10, 3).add_minutes(-30);

    assert_eq!(clock.to_string(), "09:33");
}

#[test]


fn test_subtract_more_than_an_hour() {
    let clock = Clock::new(10, 3).add_minutes(-70);

    assert_eq!(clock.to_string(), "08:53");
}

#[test]


fn test_subtract_across_midnight() {
    let clock = Clock::new(0, 3).add_minutes(-4);

    assert_eq!(clock.to_string(), "23:59");
}

#[test]


fn test_subtract_more_than_two_hours() {
    let clock = Clock::new(0, 0).add_minutes(-160);

    assert_eq!(clock.to_string(), "21:20");
}

#[test]


fn test_subtract_more_than_two_hours_with_borrow() {
    let clock = Clock::new(6, 15).add_minutes(-160);

    assert_eq!(clock.to_string(), "03:35");
}

#[test]


fn test_subtract_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(-1500);

    assert_eq!(clock.to_string(), "04:32");
}

#[test]


fn test_subtract_more_than_two_days() {
    let clock = Clock::new(2, 20).add_minutes(-3000);

    assert_eq!(clock.to_string(), "00:20");
}

//

// Test Equality

//

#[test]


fn test_compare_clocks_for_equality() {
    assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
}

#[test]


fn test_compare_clocks_a_minute_apart() {
    assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
}

#[test]


fn test_compare_clocks_an_hour_apart() {
    assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
}

#[test]


fn test_compare_clocks_with_hour_overflow() {
    assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
}

#[test]


fn test_compare_clocks_with_hour_overflow_by_several_days() {
    assert_eq!(Clock::new(99, 11), Clock::new(3, 11));
}

#[test]


fn test_compare_clocks_with_negative_hour() {
    assert_eq!(Clock::new(-2, 40), Clock::new(22, 40));
}

#[test]


fn test_compare_clocks_with_negative_hour_that_wraps() {
    assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));
}

#[test]


fn test_compare_clocks_with_negative_hour_that_wraps_multiple_times() {
    assert_eq!(Clock::new(-83, 49), Clock::new(13, 49));
}

#[test]


fn test_compare_clocks_with_minutes_overflow() {
    assert_eq!(Clock::new(0, 1441), Clock::new(0, 1));
}

#[test]


fn test_compare_clocks_with_minutes_overflow_by_several_days() {
    assert_eq!(Clock::new(2, 4322), Clock::new(2, 2));
}

#[test]


fn test_compare_clocks_with_negative_minute() {
    assert_eq!(Clock::new(3, -20), Clock::new(2, 40));
}

#[test]


fn test_compare_clocks_with_negative_minute_that_wraps() {
    assert_eq!(Clock::new(5, -1490), Clock::new(4, 10));
}

#[test]


fn test_compare_clocks_with_negative_minute_that_wraps_multiple() {
    assert_eq!(Clock::new(6, -4305), Clock::new(6, 15));
}

#[test]


fn test_compare_clocks_with_negative_hours_and_minutes() {
    assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));
}

#[test]


fn test_compare_clocks_with_negative_hours_and_minutes_that_wrap() {
    assert_eq!(Clock::new(-54, -11_513), Clock::new(18, 7));
}

#[test]


fn test_compare_full_clock_and_zeroed_clock() {
    assert_eq!(Clock::new(24, 0), Clock::new(0, 0));
}
