pub mod linked_list {

    #[derive(Debug,  Clone)]
    pub struct Node<T> {
        pub element: Option<T>,
        pub next: Option<Box<Node<T>>>,
    }

    impl <T> Node< T> {
        pub fn new(data: Option<T>) -> Box<Node<T>> {
            Box::new(Node
            {
                element: data,
                next: None
            })
        }
    }
}
