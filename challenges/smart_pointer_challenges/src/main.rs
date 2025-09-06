fn main() {
    println!("Hello, world!");
}

use std::fmt;

// =============================================================================
// EASY CHALLENGES (1-3): Basic Box<T> usage
// =============================================================================

// EASY CHALLENGE 1: Basic Heap Allocation
// Goal: Understand Box<T> basics and heap vs stack
mod easy_challenge_1 {
    use super::*;

    // TODO: Create a function that returns a large array on the heap
    // Hint: Large arrays on the stack can cause stack overflow
    fn create_large_array() -> Box<Vec<i32>> {
        Box::new(vec![0; 1_000_000])
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_heap_allocation() {
            // TODO: Create a boxed array of 1 million i32s filled with zeros
            let large_array = create_large_array();

            assert_eq!(large_array.len(), 1_000_000);
            assert_eq!(large_array[0], 0);
            assert_eq!(large_array[999_999], 0);
        }

        #[test]
        fn test_box_deref() {
            // TODO: Create a boxed integer and demonstrate dereferencing
            let boxed_num = Box::new(42);

            assert_eq!(*boxed_num, 42);
            assert_eq!(boxed_num.clone(), Box::new(42));
        }
    }
}

// EASY CHALLENGE 2: Simple Recursive Data Structure
// Goal: Build a basic linked list using Box<T>
mod easy_challenge_2 {
    use super::*;

    // TODO: Define a simple singly-linked list
    #[derive(Debug, PartialEq)]
    enum SimpleList<T> {
        Empty,
        Node(T, Box<SimpleList<T>>),
    }

    impl<T> SimpleList<T> {
        fn new() -> Self {
            SimpleList::Empty
        }

        fn push(&mut self, value: T) {
            let old_list = std::mem::replace(self, SimpleList::Empty);
            // TODO: Add element to front of list
            *self = SimpleList::Node(value, Box::new(old_list))
        }

        fn len(&self) -> usize {
            match self {
                SimpleList::Empty => 0,
                SimpleList::Node(value, next) => {
                    1 + next.len()
                }
            }
        }

        fn is_empty(&self) -> bool {
            match self {
                SimpleList::Empty => true,
                _ => false,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_simple_list() {
            let mut list = SimpleList::new();
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push(1);
            list.push(2);
            list.push(3);

            assert!(!list.is_empty());
            assert_eq!(list.len(), 3);
        }
    }
}

// EASY CHALLENGE 3: Box with Trait Objects
// Goal: Use Box<dyn Trait> for dynamic dispatch
mod easy_challenge_3 {
    use super::*;

    trait Animal {
        fn make_sound(&self) -> String;
        fn name(&self) -> &str;
    }

    // TODO: Implement Animal for Dog and Cat
    struct Dog { name: String }

    impl Animal for Dog {
        fn make_sound(&self) -> String {
            "bark".to_string()
        }

        fn name(&self) -> &str {
            &self.name
        }
    }
    struct Cat { name: String }

    impl Animal for Cat {
        fn make_sound(&self) -> String {
            "meow".to_string()
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    struct Bird { name: String, can_fly: bool }

    impl Animal for Bird {

        fn make_sound(&self) -> String {
            if self.can_fly {
                "chirp chirp".to_string()
            } else {
                "squark".to_string()
            }
        }

        fn name(&self) -> &str {
            &self.name
        }

    }
    // TODO: Create a Zoo that stores different animals using Box<dyn Animal>
    struct Zoo {
        animals: Vec<Box<dyn Animal>>,
    }

    impl Zoo {
        fn new() -> Self {
            Zoo {
                animals: Vec::new(),
            }
        }

        fn add_animal(&mut self, animal: Box<dyn Animal>) {
            // TODO: Add animal to zoo
            self.animals.push(animal);
        }

        fn all_sounds(&self) -> Vec<String> {
            // TODO: Return sounds of all animals
            self.animals.iter().map(|animal| {
                animal.make_sound()
            }).collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_zoo() {
            let mut zoo = Zoo::new();
            zoo.add_animal(Box::new(Dog { name: "Rex".to_string() }));
            zoo.add_animal(Box::new(Cat { name: "Whiskers".to_string() }));

            let sounds = zoo.all_sounds();
            assert_eq!(sounds.len(), 2);
            // Dogs bark, cats meow - implement as you like!
        }

        #[test]
        fn test_add_birds() {
            let mut zoo = Zoo::new();

            zoo.add_animal(Box::new(Dog { name: "Rex".to_string() }));
            zoo.add_animal(Box::new(Cat { name: "Whiskers".to_string() }));
            zoo.add_animal(Box::new( Bird { name: "Blue".to_string() , can_fly: true }));

            let sounds = zoo.all_sounds();
            assert_eq!(sounds.len(), 3);
        }
    }
}