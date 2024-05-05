use std::cell::RefCell;
use std::rc::Rc;


/*
create nodes that can be
1. linked with
2. can be traversed
3. can be sorted based on data

contains:
data,
ptr - > pointing to new node or None

*/

pub struct LinkedList<T>{
    start:Option<Rc<RefCell<Node<T>>>>,
    end:Option<Rc<RefCell<Node<T>>>>
}
pub struct Node<T>{
    item:T,
    next:Option<Rc<RefCell<Node<T>>>>
}

impl<T> LinkedList<T>{
    fn new() -> Self {
        // both ends set to None
        LinkedList {
            start:None,
            end:None }
    }

    pub fn create_node(&mut self, data:T){
      let node = Rc::new(RefCell::new(Node{
          item:data,
          next:self.end.take() //None
      }));
        if self.start.is_none() { //assigned only once
            self.start= Some(node.clone());
        }
        self.end = Some(node.clone());
        node.borrow_mut().next = self.end.clone();
    }

}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut list = LinkedList::new();
        list.create_node(1); //start points to 1 and end points to 1. And 1 points to end
        list.create_node(2); //start points to 1. 1.Next points to 2. 2.Next points to end. End points to 2.
        list.create_node(3); //start points to 1. .Next points to 3. 3.Next points to end. End points to 3.



        println!("value is {}", list.end.unwrap().borrow().item);

    }
}
