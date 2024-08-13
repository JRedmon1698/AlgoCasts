fn main() {
    let mut q = Queue::new();
    q.add(MyData::Int(1));
    q.add(MyData::String(String::from("two")));
    println!("q: {:?}", q);
    println!("peeked: {:?}", q.peek());

    let popped = q.remove();
    println!("popped: {:?}", popped);
    println!("{:?}", q);
}

#[derive(Debug)]
struct Stack {
    data: Vec<MyData>,
}

#[derive(Debug)]
struct Queue {
    data_a: Stack,
    data_b: Stack,
}

#[derive(Debug)]
enum MyData {
    Int(usize),
    String(String),
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, val: MyData) {
        self.data.push(val);
    }

    fn pop(&mut self) -> MyData {
        self.data.pop().unwrap()
    }

    fn peek(&self) -> Option<&MyData> {
        match self.data.iter().last() {
            Some(val) => Some(val),
            None => None,
        }
    }
}

impl Queue {
    fn new() -> Self {
        Queue {
            data_a: Stack::new(),
            data_b: Stack::new(),
        }
    }

    fn add(&mut self, val: MyData) {
        self.data_a.push(val);
    }

    fn remove(&mut self) -> MyData {
        match self.data_a.peek() {
            val => self.data_b.push(self.data_a.pop()),
            _ => (),
        }

        let removed = self.data_b.pop();

        match self.data_b.peek() {
            val => self.data_a.push(self.data_b.pop()),
            _ => (),
        }

        removed
    }

    fn peek(&mut self) -> Option<&MyData> {
        match self.data_a.peek() {
            val => {
                self.data_b.push(self.data_a.pop());
            }
            _ => (),
        }

        let peeked = match self.data_b.peek() {
            Some(val) => Some(val),
            None => None,
        };

        match self.data_b.peek().unwrap() {
            MyData::Int(i) => self.data_a.push(MyData::Int(i.clone())),
            MyData::String(s) => self.data_a.push(MyData::String(s.clone())),
        }

        peeked
    }
}
