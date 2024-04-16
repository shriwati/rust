// pub mod first;

use std::mem;

pub struct List{
    head:Link,
}
enum Link{
    Empty,
    More(Box<Node>)
}

struct Node{
    element:i32,
    next: Link
}


impl List{
    pub fn new()->Self{
        List{ head : Link::Empty} // return head that points to Empty Link
    }

    pub fn push(&mut self,data:i32){
        let new_node=Box::new(Node{
            element:data,
            next: mem::replace(&mut self.head,Link::Empty) // temporarily replacing it with empty ptr
        });
        self.head = Link::More(new_node); // now point it to new node
    }
}

