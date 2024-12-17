use std::io::Empty;

// Define the Node struct
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>, // Use Option<Box<Node<T>>> to allow nodes to be nullable
}

// Define the LinkedList struct
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>, // Head of the linked list
}

// Implementation of methods for LinkedList
impl<T> LinkedList<T> {
    // Constructor
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Method to insert a new node at the beginning of the linked list
    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data: data,
            next: self.head.take(), // Take the current head and replace it with None
        };
        self.head = Some(Box::new(new_node)); // Wrap the new node in a Box and assign it as the new head
    }

    // Method to remove the first node from the linked list and return its data
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next; // Move the next node to the head
            node.data
        })
    }

    // Method to check if the linked list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
