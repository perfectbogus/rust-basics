fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct BST<T: Ord> {
    root: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> BST<T> {

    fn new() -> BST<T> {
        BST { root: None }
    }

    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => self.root = Some(Box::new(Node { value, left: None, right: None })),
            Some(root_node) => {
                Self::insert_recursive(root_node, value);
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node<T>>, value: T) {
        if value < node.value {
            match &mut node.left {
                None => { node.left = Some(Box::new(Node { value, left: None, right: None })); },
                Some(left_child) => { Self::insert_recursive(left_child, value); }
            }
        } else if value > node.value {
            match &mut node.right {
                None => { node.right = Some(Box::new(Node { value, left: None, right: None }));}
                Some(right_child) => { Self::insert_recursive(right_child, value);}
            }
        }
    }

    fn search_simpler(&self, value: &T) -> bool {
        let mut current = &self.root;

        loop {
            match current {
                None => return false,
                Some(node) => {
                    if value == &node.value {
                        return true;
                    } else if value < &node.value {
                        current = &node.left;
                    } else {
                        current = &node.right;
                    }
                }
            }
        }
    }

    fn search(&self, value: &T) -> bool {
        match &self.root {
            None => false,
            Some(node) => {
                if value == &node.value {
                    true
                } else if value < &node.value {
                    Self::search_recursive(&node.left, value)
                } else {
                    Self::search_recursive(&node.right, value)
                }
            }
        }
    }

    fn search_recursive(node_option: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node_option {
            None => false,
            Some(node) => {
                if value == &node.value {
                    true
                } else if value < &node.value {
                    Self::search_recursive(&node.left, value)
                } else {
                    Self::search_recursive(&node.right, value)
                }
            }
        }
    }

    fn height(&self) -> usize {
        match &self.root {
            None => 0,
            Some(current) => {
                1 + std::cmp::max(Self::height_recursive(&current.left), Self::height_recursive(&current.right))
            }
        }
    }

    fn height_recursive(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            None => { 0 }
            Some(current) => {
                1 + std::cmp::max(Self::height_recursive(&current.left), Self::height_recursive(&current.right))
            }
        }
    }

    fn in_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        Self::in_order_recursive(&self.root, &mut result);
        result
    }

    fn in_order_recursive(node: &Option<Box<Node<T>>>, result: &mut Vec<T>) {
        match node {
            None => return,
            Some(node) => {
                Self::in_order_recursive(&node.left, result);
                result.push(node.value.clone());
                Self::in_order_recursive(&node.right, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_operations() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);


        assert!(bst.search(&5));
        assert!(bst.search(&1));
        assert!(!bst.search(&10));

        assert!(bst.search_simpler(&5));
        assert!(bst.search_simpler(&1));
        assert!(!bst.search_simpler(&10));

        assert_eq!(bst.height(), 3);

        let sorted = bst.in_order_traversal();
        assert_eq!(sorted, vec![1, 3, 5, 7, 9]);
    }
}