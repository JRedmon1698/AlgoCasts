class Stack {
  constructor() {
    this.data = [];
  }

  push(record) {
    this.data.push(record);
  }

  pop() {
    return this.data.pop();
  }

  peek() {
    return this.data[this.data.length - 1];
  }
}

let stack = new Stack();
stack.push(1);
stack.push(2);

console.log(stack);

console.log(stack.peek());

stack.pop();
console.log(stack);

module.exports = Stack;
