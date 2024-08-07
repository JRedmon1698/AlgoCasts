// --- Directions
// Create a stack data structure.  The stack
// should be a class with methods 'push', 'pop', and
// 'peek'.  Adding an element to the stack should
// store it until it is removed.
// --- Examples
//   const s = new Stack();
//   s.push(1);
//   s.push(2);
//   s.pop(); // returns 2
//   s.pop(); // returns 1

class Stack {
  constructor() {
    this.data = [];
  }

  // dont use native push()
  push(val) {
    // before adding new element, push everything over to the left
    for (let i = this.data.length; i > 0; i--) {
      this.data[i] = this.data[i - 1];
    }

    // add new element to "front"
    this.data[0] = val;
  }

  pop() {
    if (this.data.length < 1) {
      return undefined;
    }

    let top = this.data[0];

    // similarly to push, shift everything to the right before removing top
    for (let i = 0; i < this.data.length - 1; i++) {
      this.data[i] = this.data[i + 1];
    }

    // clean up "hanging" index of top that has been removed
    this.data.length = this.data.length - 1;

    return top;
  }

  peek() {
    return this.data[0];
  }
}

let s = new Stack();
s.push(1);
s.push(2);
s.push(3);
console.log(s.data);

// s.pop();
console.log(s.pop());
console.log(s);

module.exports = Stack;
