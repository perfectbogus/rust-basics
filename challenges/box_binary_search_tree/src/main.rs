use std::cmp::max;

#[derive(Debug)]
struct TreeNode<T: Ord> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T: Ord> {
    root: Option<Box<TreeNode<T>>>,
    size: usize,
}

impl<T: Ord> TreeNode<T> {
    fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                None => { self.left = Some(Box::new(TreeNode::new(value))); },
                Some(ref mut left) => { left.insert(value); },
            }
        } else {
            match self.right {
                None => { self.right = Some(Box::new(TreeNode::new(value))) },
                Some(ref mut right) => { right.insert(value); },
            }
        }
    }

    fn height(&self) -> usize {
        let left_height = self.left.as_ref().map(|node| node.height()).unwrap_or(0);
        let right_height = self.right.as_ref().map(|node| node.height()).unwrap_or(0);

        max(left_height, right_height) + 1
    }

    fn collect_values(&self) -> Vec<&T> {
        let mut values = Vec::new();

        // Add left subtree values
        if let Some(ref left) = self.left {
            values.extend(left.collect_values());
        }

        // Add current value
        values.push(&self.value);

        if let Some(ref right) = self.right {
            values.extend(right.collect_values());
        }

        values
    }

}

impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        Self { root: None, size: 0 }
    }

    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => { self.root = Some(Box::new(TreeNode::new(value))); },
            Some(root) => { root.insert(value) }
        }
        self.size += 1;
    }

    fn contains(&self, value: &T) -> bool {
        match &self.root {
            None => { false }
            Some(node) => {
                if &node.value == value {
                    true
                } else if value < &node.value {
                    node.left.as_ref().map_or(false, |left| left.contains(value))
                } else {
                    node.right.as_ref().map_or(false, |right| right.contains(value))
                }
            }
        }
    }

    fn remove(&mut self, value: &T) -> bool {
        // Helper function to find minimum value in a subtree
        fn get_min_value<T: Ord>(node: &Box<TreeNode<T>>) -> &T {
            let mut current = node;
            while let Some(ref left) = current.left {
                current = left;
            }
            &current.value
        }

        // Helper function to remove node recursively
        fn remove_recursive<T: Ord + Clone>(root: &mut Option<Box<TreeNode<T>>>, value: &T) -> bool {
            if let Some(node) = root {
                if value < &node.value {
                    remove_recursive(&mut node.left, value)
                } else if value > &node.value {
                    remove_recursive(&mut node.right, value)
                } else {
                    // Node found, handle removal
                    match (node.left.take(), node.right.take()) {
                        (None, None) => {
                            // Case 1: No children
                            *root = None;
                        }
                        (Some(left), None) => {
                            // Case 2a: Only left child
                            *root = Some(left);
                        }
                        (None, Some(right)) => {
                            // Case 2b: Only right child
                            *root = Some(right);
                        }
                        (Some(mut left), Some(right)) => {
                            // Case 3: Two children
                            // Find minimum value in right subtree
                            let min_val = get_min_value(&right).clone();
                            // Replace current node's value
                            node.value = min_val;
                            // Restore children
                            node.left = Some(left);
                            node.right = Some(right);
                            // Remove the duplicate
                            return remove_recursive(&mut node.right, &min_val);
                        }
                    }
                    true
                }
            } else {
                false
            }
        }

        // Start removal from root
        let found = remove_recursive(&mut self.root, value);
        if found {
            self.size -= 1;
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
    }
}

fn main() {
    println!("Hello, world!");
}
