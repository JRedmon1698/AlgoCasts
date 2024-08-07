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
    this.current = 0;
  }

  push(val) {
    // console.log("current: ", this.current)
    this.data[this.current] = val;
    this.current++;
  }

  pop() {
    if (this.data.length < 1) {
      return undefined;
    }

    let top = this.data[];
    this.current--;
    return top;
  }
}

let s = new Stack();
s.push(1);
s.push(2);
s.push(3);
console.log(s.data);

s.pop();
console.log(s.pop());
console.log(s);

module.exports = Stack;
