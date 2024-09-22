#[derive(Debug)]
struct Node {
    data: usize,
    children: Vec<Node>,
}

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
}

fn level_width(root: &Node) -> Vec<usize> {
    let stop = String::from("ballast");
    let nodes: Vec<Node> = [root, stop];
    let counts = [0_usize];
}

fn main() {
    let mut node = Node::new(3);
    node.add(5);
    node.add(7);
    println!("{:?}", node);
}
