/*

*/

pub struct LinkedList<T> {
    // start:Option<Box<Node<T>>>,  // always point to fist element
    head: Option<Box<Node<T>>> // head keeps moving as we add more element
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // return Linked List , head
    pub fn new() -> Self {
        LinkedList { head: None}
    }

    pub fn add_front(&mut self, data: T) {
        let node = Node {
            item: data,
            next: self.head.take(),
        };
        if self.head.is_none() {
            self.head = Option::from(Box::new(node));
        }
    }
}
impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.item
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_list() {
        let mut list: LinkedList<String> = LinkedList::new();
        list.add_front(String::from("one"));
        list.add_front(String::from("two"));
        list.add_front(String::from("three"));

        for i in list{
            println!("{}",i);
        }
        // list.print_list()
    }
}
