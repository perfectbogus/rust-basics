struct CircularQueue<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    front: usize,
    size: usize,
}

impl<T> CircularQueue<T> {
    fn new(capacity: usize) -> Self {
        let mut buffer  = Vec::with_capacity(capacity);
        buffer.extend((0..capacity).map(|_| None));
        CircularQueue {
            buffer,
            capacity,
            front: 0,
            size: 0
        }
    }

    fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.is_full() {
            Err("Circular Queue is full".to_string())
        } else {
            let back = (self.front + self.size) % self.capacity;
            self.buffer[back] = Some(item);
            self.size += 1;
            Ok(())
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let item = self.buffer[self.front].take();
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            item
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn peek(&self) -> Option<&T> {
        self.buffer[self.front].as_ref()
    }

    fn len(&self) -> usize {
        self.size
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = CircularQueue::new(3);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_circular_behavior() {
        let mut queue = CircularQueue::new(3);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        assert_eq!(queue.dequeue(), Some(1));
        queue.enqueue(4).unwrap();
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
    }

    #[test]
    fn test_full_queue() {
        let mut queue = CircularQueue::new(2);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        assert!(queue.enqueue(3).is_err());
    }

    #[test]
    fn test_peek() {
        let mut queue = CircularQueue::new(2);
        assert_eq!(queue.peek(), None);
        queue.enqueue(1).unwrap();
        assert_eq!(queue.peek(), Some(&1));
        queue.enqueue(2).unwrap();
        assert_eq!(queue.peek(), Some(&1));
    }
}
