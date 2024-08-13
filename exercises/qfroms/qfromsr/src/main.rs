fn main() {
    // let mut s = Stack::new();
    // s.push(MyData::Int(1));
    // s.push(MyData::Int(2));
    // s.push(MyData::String(String::from("howdy")));
    // println!("peeked: {:?}", s.peek());
    // println!("stack: {:?}", s);
    // let popped = s.pop();
    // println!("popped: {:?}", popped);
    // println!("s: {:?}", s);

    let mut q = Queue::new();
    q.add(MyData::Int(1));
    q.add(MyData::String(String::from("two")));
    println!("q: {:?}", q);
    println!("peeked: {:?}", q.peek());

    // let popped = q.remove();
    // println!("popped: {:?}", popped);
    // println!("{:?}", q);
}

#[derive(Debug, Clone)]
struct Stack {
    data: Vec<Option<MyData>>,
}

#[derive(Debug)]
struct Queue {
    data_a: Stack,
    data_b: Stack,
}

#[derive(Debug, Clone)]
enum MyData {
    Int(usize),
    String(String),
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, val: MyData) {
        self.data.push(Some(val));
    }

    fn pop(&mut self) -> Option<MyData> {
        self.data.pop().unwrap()
    }

    fn peek(&self) -> Option<MyData> {
        // let len = self.data.len();
        // self.data[len - 1]
        // let last = match self.data.last() {
        //     Some(val) => Some(val.clone().unwrap()),
        //     None => None,
        // };

        let last = self.data.last().to_owned();
        match last {
            Some(val) => {
                return Some(val.to_owned().unwrap());
            }
            None => None,
        }

        // last.clone().as_ref()
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

    // fn remove(&mut self) -> Option<MyData> {
    //     while let Some(val) = self.data_a.peek() {}
    // }

    fn peek(&mut self) -> Option<MyData> {
        while let Some(val) = self.data_a.peek() {
            self.data_b.push(self.data_a.pop().unwrap());
        }

        let peeked = match self.data_b.peek() {
            Some(val) => Some(val),
            _ => None,
        };
        // let peeked = self.data_b.peek().map(|v| v);

        while let Some(val) = self.data_b.peek() {
            self.data_a.push(self.data_b.clone().pop().unwrap());
        }

        peeked
    }

    // fn remove(&mut self) -> MyData {
    //     // match self.data_a.peek() {
    //     //     val => {
    //     //         println!("{:?}", val);
    //     //         self.data_b.push(self.data_a.pop());
    //     //     }
    //     //     _ => (),
    //     // }
    //     while let Some(val) = self.data_a.pop() {
    //         self.data_b.push(self.data_a.pop().unwrap());
    //     }
    //     // while self.data_a.peek() {
    //     //     self.data_b.push(self.data_a.pop());
    //     // }

    //     // println!("data A pre-removed capture: {:?}", self.data_a);
    //     // println!("data B pre-removed captrue: {:?}", self.data_b);

    //     let removed = self.data_b.pop().unwrap();

    //     // match self.data_b.peek() {
    //     //     val => self.data_a.push(self.data_b.pop()),
    //     //     _ => (),
    //     // }
    //     while let Some(val) = self.data_b.pop() {
    //         self.data_a.push(self.data_b.pop().unwrap());
    //     }
    //     // while self.data_b.peek().is_some() {
    //     //     self.data_a.push(self.data_b.pop());
    //     // }

    //     // println!("data A post-removed capture: {:?}", self.data_a);
    //     // println!("data B post-removed captrue: {:?}", self.data_b);

    //     removed
    // }

    // fn peek(&mut self) -> &MyData {
    //     match self.data_a.peek() {
    //         val => {
    //             self.data_b.push(self.data_a.pop().unwrap());
    //         }
    //         _ => (),
    //     }

    //     let peeked = self.data_b.peek();
    //     // let peeked = match self.data_b.peek() {
    //     //     Some(val) => Some(val),
    //     //     None => None,
    //     // };

    //     match self.data_b.peek() {
    //         MyData::Int(i) => self.data_a.push(MyData::Int(i.clone())),
    //         MyData::String(s) => self.data_a.push(MyData::String(s.clone())),
    //     }

    //     peeked
    // }
}
