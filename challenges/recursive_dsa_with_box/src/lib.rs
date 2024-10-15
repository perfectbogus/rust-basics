/*
Challenge: Recursive Data Structure with Box
Your task is to implement a simple binary tree structure using Box.
The tree should support the following operations:

    1: Inserting a value
    2: Checking if a value exists in the tree
    3: Calculating the depth of the tree
 */
use std::cmp::max;

#[derive(Debug)]
struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn new(value: i32) -> BinaryTree {
        BinaryTree {
            value,
            right: None,
            left: None
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            match self.left {
                None => self.left = Some(Box::new(BinaryTree::new(value))),
                Some(ref mut left) => left.insert(value),
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(BinaryTree::new(value))),
                Some(ref mut right) => right.insert(value),
            }

        }
    }

    fn contains(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            self.left.as_ref().map_or(false, |left| left.contains(value))
        } else {
            self.right.as_ref().map_or(false, |right| right.contains(value))
        }
    }

    fn depth(&self) -> u32 {
        let left = self.left.as_ref().map_or(0, |left| left.depth());
        let right = self.right.as_ref().map_or(0, |right| right.depth());
        1 + max(left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut tree = BinaryTree::new(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);

        assert!(tree.contains(5));
        assert!(tree.contains(3));
        assert!(tree.contains(7));
        assert!(tree.contains(1));
        assert!(tree.contains(9));
        assert!(!tree.contains(4));
        assert!(!tree.contains(6));
    }

    #[test]
    fn test_depth() {
        let mut tree = BinaryTree::new(5);
        assert_eq!(tree.depth(), 1);

        tree.insert(3);
        tree.insert(7);
        assert_eq!(tree.depth(), 2);

        tree.insert(1);
        tree.insert(9);
        assert_eq!(tree.depth(), 3);
    }
}
