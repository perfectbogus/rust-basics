use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    id: u32,
    data: String,
    children: Vec<Rc<RefCell<Node>>>,
    parents: Vec<Weak<RefCell<Node>>>
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>
}

fn main() {
    println!("Hello, world!");
}
