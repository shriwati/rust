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
    head_ptr:Option<Rc<Node<T>>>,
    start_ptr:Option<Rc<Node<T>>>,
    end_ptr:Option<Rc<Node<T>>>
}
pub struct Node<T>{
    item:T,
    next:Option<Rc<Node<T>>>
}

impl<T> LinkedList<T>{
    fn new() -> Self {
        // both ends set to None
        LinkedList {
            head_ptr:None,
            start_ptr:None,
            end_ptr:None }
    }

    pub fn create_node(&mut self, data:T){
      let node = Rc::new(Node{
          item:data,
          next:self.head_ptr.take() //None
      });
        self.head_ptr = Some(Rc::clone(&node));
        if self.start_ptr.is_none(){
            self.start_ptr = Some(Rc::clone(&node))
        }
        // end ptr
        self.end_ptr = Some(Rc::clone(&node))
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
        list.create_node(1);
        list.create_node(2);
        list.create_node(3);

        let  ptr = list.start_ptr;
        println!("value is {}",ptr.unwrap().item );

    }
}
