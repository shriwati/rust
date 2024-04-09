pub mod linked_list {
    use std::fmt;

    #[derive( Clone)]
    pub struct Node<T> {
        pub element: Option<T>,
        pub next: Option<Box<Node<T>>>,
    }

    impl <T> Node<T> {
        pub fn new() -> Node<T> {
            Node
            {
                element: None,
                next: None
            }
        }
    }

    impl<T> fmt::Display for Node<T>
        where
            T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
            let formatted_elements = self
                .element
                .iter()
                .map(|el| format!("{}", el))
                .collect::<Vec<_>>();
            let elements = formatted_elements.join(", ");
            write!(f, "[{:?}]",elements)
            }

        }
    }


