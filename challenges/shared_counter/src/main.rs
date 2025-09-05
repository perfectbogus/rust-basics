// Super Simple Challenge: Shared Counter
// Goal: Just practice Rc<RefCell<T>> with the simplest possible example

use std::rc::Rc;
use std::cell::RefCell;

// TODO: Your implementation here!
#[derive(Debug, Clone)]
struct SharedCounter {
    value: Rc<RefCell<i32>>,
}

impl SharedCounter {
    fn new() -> Self {
        SharedCounter {
            value: Rc::new(RefCell::new(0))
        }
    }

    fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    fn add(&self, value: i32) {
        *self.value.borrow_mut() += value;
    }

    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_counter() {
        let counter = SharedCounter::new();
        assert_eq!(counter.get_value(), 0);
    }

    #[test]
    fn test_increment_counter() {
        let counter = SharedCounter::new();
        counter.increment();
        assert_eq!(counter.get_value(), 1);

        counter.increment();
        assert_eq!(counter.get_value(), 2);
    }

    #[test]
    fn test_shared_counter() {
        let counter = SharedCounter::new();

        // Create two "handles" to the same counter
        let counter1 = counter.clone();
        let counter2 = counter.clone();

        // Increment through counter1
        counter1.increment();

        // Check value through counter2
        assert_eq!(counter2.get_value(), 1);

        // Increment through counter2
        counter2.increment();

        // Check through original counter
        assert_eq!(counter.get_value(), 2);
    }

    #[test]
    fn test_add_amount() {
        let counter = SharedCounter::new();
        counter.add(5);
        assert_eq!(counter.get_value(), 5);

        counter.add(3);
        assert_eq!(counter.get_value(), 8);
    }
}

fn main() {
    let counter = SharedCounter::new();

    let counter_one = counter.clone();
    let counter_two = counter.clone();

    counter_one.increment();
    println!("counter value: {}", counter.get_value());

    counter_two.increment();
    println!("counter value: {}", counter.get_value());


}

