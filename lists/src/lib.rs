use std::cell::{RefCell};
use std::rc::Rc;

type Ptr<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T>{
    prev : Ptr<T>,
    item:T,
    next: Ptr<T>
}

impl <T> Node<T> {
    pub fn new(element:T)->Self{
        Node{
            prev:None,
            item:element,
            next:None
        }

    }

}

#[derive(Debug)]
pub struct LinkNodes<T>{
    head: Ptr<T>,
    tail: Ptr<T>,
    count:i32
}

impl <T>LinkNodes<T> {

    pub fn new()->Self{
        LinkNodes{
            head:None,
            tail:None,
            count:0
        }
    }

    pub fn add_node(&mut self,element:T){
        let node = Rc::new(RefCell::new(Node::new(element)));

        if let Some(prev_tail)=self.tail.take(){
            // prev nodes next node points to new node
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            // new node's prev points to tail
            node.borrow_mut().prev = Some(prev_tail);
            // tail points to new node
            self.tail = Some(node);
        }else{
            // head points to new node
            self.head = Some(Rc::clone(&node));
            // tail points to new node
            self.tail = Some(node);
        }
        self.count +=1;

    }

}
