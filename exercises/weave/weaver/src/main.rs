fn main() {
    let mut queue_1: Queue<usize> = Queue::new();
    queue_1.add(1);
    queue_1.add(2);
    queue_1.add(3);
    // println!("full queue 1: {:?}", queue_1);

    // queue_1.remove();
    // println!("minus one: {:?}", queue_1);

    // let last = &queue_1.peek();
    // println!("last el: {:?}", last);

    let mut queue_2: Queue<String> = Queue::new();
    queue_2.add(String::from("one"));
    queue_2.add(String::from("two"));
    queue_2.add(String::from("three"));
    // println!("string queue: {:?}", queue_2);

    let weaved: Queue<MyType> = weave(queue_1, queue_2);
    println!("weaved: {:?}", weaved.data);
}

#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: Vec::new() }
    }

    pub fn add(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn remove(&mut self) {
        self.data.pop();
    }

    pub fn peek(&self) -> &T {
        let last_idx = self.data.len() - 1;
        &self.data[last_idx]
    }
}

fn weave<T>(q1: Queue<T>, q2: Queue<T>) -> Queue<T>
where
    T: Clone,
{
    let weaved = Queue::new();

    weaved
}
