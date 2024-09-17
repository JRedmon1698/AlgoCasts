#[derive(Debug)]
struct Node {
    data: usize,
    children: Vec<Node>,
}

#[derive(Debug)]
struct Tree {}

impl Node {
    fn new(data: usize) -> Self {
        Node {
            data,
            children: Vec::new(),
        }
    }

    fn add(&mut self, data: usize) {
        self.children.push(Node::new(data));
    }

    fn remove(&mut self, data: usize) {
        self.children.retain(|c| c.data != data);
    }
}

fn main() {
    let mut node = Node::new(5);
    node.add(7);
    node.add(9);
    node.remove(9);
    println!("{:?}", node);
}
