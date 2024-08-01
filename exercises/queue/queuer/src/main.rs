fn main() {
    let mut q = Queue::new();
    println!("empty queue: {:#?}", q);

    q.add(1);
    q.add(2);
    q.add(3);
    println!("full queue: {:#?}", q);

    q.remove();
    println!("queue minus one: {:#?}", q);
}

#[derive(Debug)]
struct Queue {
    data: Vec<usize>,
}

impl Queue {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn add(&mut self, val: usize) {
        self.data.push(val);
    }

    fn remove(&mut self) {
        self.data.pop();
    }
}
