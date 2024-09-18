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

    fn traverse_bf<F>(&self, mut func: F)
    where
        F: FnMut(&Node) -> (),
    {
        let mut nodes = vec![self.root.as_ref().unwrap()];
        while nodes.len() > 0 {
            let node = nodes.remove(0);
            for child in node.children.iter() {
                nodes.push(child);
            }

            func(node);
        }
    }

    fn traverse_df<F>(&self, mut func: F)
    where
        F: FnMut(&Node) -> (),
    {
        let mut nodes = vec![self.root.as_ref().unwrap()];
        while nodes.len() > 0 {
            let last_idx = nodes.len() - 1;
            let node = nodes.remove(last_idx);
            for child in node.children.iter() {
                nodes.push(child);
            }

            func(node);
        }
    }
}

fn main() {
    let mut node = Node::new(5);
    node.add(7);
    node.add(9);
    // node.remove(9);

    let mut tree = Tree::new();
    tree.root = Some(node);
    tree.traverse_df(|n| println!("data + 1: {:?}", n.data.clone() + 1));
    println!("{:?}", tree);
}
