fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("full stack: {:?}", stack);

    let popped = stack.pop();
    println!("popped: {:?}", popped);
    println!("after pop: {:?}", stack);
}

#[derive(Debug)]
struct Stack {
    data: Vec<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, v: usize) {
        self.data.push(v);
    }

    pub fn pop(&mut self) -> usize {
        self.data.pop().unwrap()
    }
}
