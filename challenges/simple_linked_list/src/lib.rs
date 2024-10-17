use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
    value: T,
    next: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct LinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<T> {
        unimplemented!()
    }

    fn len(&self) -> usize {
        unimplemented!()
    }

    fn to_vec(&self) -> Vec<&T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        list.push_front(1);
        assert_eq!(list.len(), 1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_to_vec() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.to_vec(), vec![&1, &2, &3]);
    }
}