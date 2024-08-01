struct Node {
    val: i32,
    next: Option<Node>
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val,
            next: None,
        }
    }
}

struct LinkedList {
    head: Node
}
fn main() {
    let x = Box::new(5);

}