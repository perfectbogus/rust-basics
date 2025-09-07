use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

fn main() {
    let mut graph = Graph::<i32>::new();

    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    graph.add_edge(1, 2);




}

#[derive(Debug)]
struct Graph<T: Clone + Eq + Hash> {
    nodes: HashMap<T, Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    neighbors: Vec<T>,
}

impl<T: Clone + Eq + Hash> Graph<T> {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, value: T) {
        self.nodes.insert(value.clone(), Box::new(Node { value, neighbors: Vec::new() }));
    }

    fn add_edge(&mut self, from: T, to: T) {
        if let Some(node) = self.nodes.get_mut(&from) {
            node.neighbors.push(to);
        }
    }

    fn neighbors(&self, node: &T) -> Option<&Vec<T>> {
        if let Some(node) = self.nodes.get(node) {
            Some(&node.neighbors)
        } else {
            None
        }
    }

    fn neighbors_concise(&self, node: &T) -> Option<&Vec<T>> {
        self.nodes.get(node).map(|node| &node.neighbors)
    }

    fn has_path(&self, start: &T, end: &T) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start.clone());

        while let Some(current) = queue.pop_front() {

            if current == *end {
                return true;
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());

            if let Some(neighbors) = self.neighbors(&current) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.clone());
                }
            }
        }

        false
    }
}
