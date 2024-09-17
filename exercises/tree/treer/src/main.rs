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

    fn remove(&mut self, data: usize) {
        self.children.retain(|c| c.data != data);
    }
}

#[derive(Debug)]
struct Tree {
    root: Option<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn traverse_bf<F>(&self, func: F)
    where
        F: FnMut() -> (),
    {
        let node = self.root.as_ref().unwrap();
    }
}

fn main() {
    let mut node = Node::new(5);
    node.add(7);
    node.add(9);
    node.remove(9);

    let mut tree = Tree::new();
    tree.root = Some(node);
    println!("{:?}", tree);
}
