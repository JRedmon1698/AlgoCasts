#[derive(Debug)]
struct Node {
    data: u8,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: u8) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, data: u8) {
        if data < self.data {
            if let Some(left) = &mut self.left {
                left.insert(data);
            } else {
                self.left = Some(Box::new(Node::new(data)));
            }
        } else if data > self.data {
            if let Some(right) = &mut self.right {
                right.insert(data);
            } else {
                self.right = Some(Box::new(Node::new(data)));
            }
        }
    }
}

fn validate(node: Node, _min: Option<u8>, _max: Option<u8>) -> bool {
    let min = None;
    let max = None;

    if let Some(max) = max {
        if node.data > max {
            return false;
        }
    }
    if let Some(min) = min {
        if node.data < min {
            return false;
        }
    }

    if let Some(left) = node.left {
        if !validate(*left, min, Some(node.data)) {
            return false;
        }
    }
    if let Some(right) = node.right {
        if !validate(*right, Some(node.data), max) {
            return false;
        }
    }

    true
}

fn main() {
    let mut node = Node::new(10);
    node.insert(15);
    node.insert(20);
    node.insert(1);
    node.insert(2);

    println!("{:#?}", node);
    println!("valid: {}", validate(node, None, None));
}
