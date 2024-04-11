
pub mod linked_list {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Node<T> {
        pub prev: Option<Box<Node<T>>>,
        pub element: T,
        pub next: Option<Box<Node<T>>>,
    }


    impl<T> Node<T> {
        pub fn new(data: T) -> Option<Box<Node<T>>> {
            Some(Box::new(Node{prev:None,element:data,next:None}))
        }
        pub fn add_next(&mut self, data: T) -> &Option<Box<Node<T>>> {
            self.next = Node::new(data);
            let x = &self.next;
            x
        }
    }

    // impl<T> fmt::Display for Node<T>
    //     where
    //         T: fmt::Display,
    // {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         let formatted_elements = self
    //             .element
    //             .iter()
    //             .map(|el| format!("{}", el))
    //             .collect::<Vec<_>>();
    //         let elements = formatted_elements.join(", ");
    //         write!(f, "[{:?}]", elements)
    //     }
    // }
}
