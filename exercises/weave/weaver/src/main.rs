fn main() {
    let mut queue_1: Queue = Queue::new();
    queue_1.add(Data::IntType(1));
    queue_1.add(Data::IntType(2));
    queue_1.add(Data::IntType(3));
    // println!("full queue 1: {:?}", queue_1);

    // queue_1.remove();
    // println!("minus one: {:?}", queue_1);

    // let last = &queue_1.peek();
    // println!("last el: {:?}", last);

    let mut queue_2: Queue = Queue::new();
    queue_2.add(Data::StringType(String::from("one")));
    queue_2.add(Data::StringType(String::from("two")));
    queue_2.add(Data::StringType(String::from("three")));
    // println!("string queue: {:?}", queue_2);

    let weaved: Queue = weave(queue_1, queue_2);
    println!("weaved: {:?}", weaved.data);
}

#[derive(Debug, Clone)]
enum Data {
    StringType(String),
    IntType(usize),
}

#[derive(Debug)]
struct Queue {
    data: Vec<Data>,
}

impl Queue {
    pub fn new() -> Self {
        Queue { data: Vec::new() }
    }

    pub fn add(&mut self, val: Data) {
        self.data.push(val);
    }

    pub fn remove(&mut self) {
        self.data.pop();
    }

    pub fn peek(&self) -> &Data {
        let last_idx = self.data.len() - 1;
        &self.data[last_idx]
    }
}

fn weave(q1: Queue, q2: Queue) -> Queue {
    let mut weaved = Queue::new();

    for (idx, el) in q1.data.into_iter().enumerate() {
        weaved.add(el);
        weaved.add(q2.data[idx].clone());
    }

    weaved
}
