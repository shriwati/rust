pub mod linked_list {

    #[derive(Debug)]
    pub struct Node<T>{
        pub element:Option<T>,
        pub next:Option<Box<Node<T>>>
    }

    impl <T> Node<T>{
        pub fn new(data:Option<T>) ->Node<T>{
            Node{element:data,next: None } // create new node
        }
    }


}