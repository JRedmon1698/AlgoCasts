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

    fn contains(&self, data: u8) -> bool {
        if data == self.data {
            return true;
        }

        if data < self.data {
            if let Some(left) = &self.left {
                return left.contains(data);
            }
        } else if data > self.data {
            if let Some(right) = &self.right {
                return right.contains(data);
            }
        }

        false
    }
}

fn main() {
    let mut node = Node::new(5);
    &node.insert(10);
    &node.insert(1);
    &node.insert(17);
    &node.insert(3);

    println!("{:#?}", node);
    println!("contains 3: {}", node.contains(3));
    println!("contains 11: {}", node.contains(11));
}
