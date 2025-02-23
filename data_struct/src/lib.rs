

pub mod queue {
    use std::mem::swap;
    pub struct Queue<T> {
        older: Vec<T>,
        younger: Vec<T>
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                older: Vec::new(),
                younger: Vec::new()
            }
        }
        pub fn push(&mut self, t: T) {
            self.younger.push(t);
        }
        pub fn pop(&mut self) -> Option<T> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }

            self.older.pop()
        }
        pub fn is_empty(&self) -> bool {
            self.younger.is_empty() && self.older.is_empty()
        }

    }

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_pop() {

        let mut q = queue::Queue::new();
        q.push('a');
        q.push('b');
        q.push('c');
        assert_eq!(q.pop(), Some('a'));
        assert_eq!(q.pop(), Some('b'));
        q.push('d');
        assert_eq!(q.pop(), Some('c'));
    }
}
