use std::arch::aarch64::uint32x2_t;
use std::io::Error;

#[derive(Debug)]
enum Friend {

    HUMAN(Box<Human>),
    NIL
}
#[derive(Debug)]
struct Human{
    name:String,
    age:u8,
    current_thought:Option<String>,
    friend: Friend
}


impl Human{
    fn new(name:&str,age:u8)->Human{
        return  Human {
            name:name.to_string(),
            age:age,
            current_thought:None,
            friend:Friend::NIL
        }

    }
    fn with_friend(mut self,friend:Box<Human>)->Human{
        self.friend = Friend::HUMAN(friend);
        self
    }
    fn with_tought(mut self,thought:&str)->Human{
        self.current_thought= Some(thought.to_string());
        self
    }
}
fn main() {
    let developer_friend = Human::new("a friend",35);
    let developer = Human::new("shri",55)
        .with_tought("I love Rust")
        .with_friend(Box::new(developer_friend));

    println!("{:?}",developer);

    // let temp=fib_series(10);
    //
    // match temp {
    //     Ok(temp)=>println!("{:?}",temp),
    //     _=>println!("Error")
    // };
    //
    // let fib_series = fib_series(10);
    // println!("{:?}",fib_series.unwrap());

}

fn fib_series(num:usize)-> Result<Vec<u32>,Error>{
    let mut list:Vec<u32>=vec![];
    list.push(0);
    list.push(1);
    for j in 2..num+1{
        list.push(list[j-1] + list[j-2]);
    }



    Ok(list)
}

