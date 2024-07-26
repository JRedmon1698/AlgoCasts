// --- Description
// Create a queue data structure.  The queue
// should be a class with methods 'add' and 'remove'.
// Adding to the queue should store an element until
// it is removed
// --- Examples
//     const q = new Queue();
//     q.add(1);
//     q.remove(); // returns 1;

class Queue {
  constructor() {
    this.data = [];
  }

  add(val) {
    this.data.unshift(val);
  }

  remove() {
      return this.data.pop();
  }
}

let q = new Queue();
q.add(1);
q.add(2);
q.add(3);

console.log(`Full queue: ${q.data}`);

q.remove();
console.log(`Queue minus 1 elem: ${q.data}`);

module.exports = Queue;
