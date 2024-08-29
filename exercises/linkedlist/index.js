// --- Directions
// Implement classes Node and Linked Lists
// See 'directions' document

class Node {
  constructor(data, next = null) {
    this.data = data;
    this.next = next;
  }
}

class LinkedList {
  constructor() {
    this.head = null;
  }

  insertFirst(data) {
    let oldHead = this.head;
    let node = new Node(data, oldHead);
    this.head = node;
  }

  size() {
    let nodeCount = 0;
    let currentNode = this.head;

    while (currentNode) {
      nodeCount++;
      currentNode = currentNode.next;
    }

    return nodeCount;
  }

  getFirst() {
    return this.head;
  }

  getLast() {
    if (!this.head) {
      return null;
    }

    let currentNode = this.head;
    while (currentNode) {
      if (!currentNode.next) {
        return currentNode;
      }
      currentNode = currentNode.next;
    }
  }

  clear() {
    this.head.next = null;
    this.head = null;
  }

  removeFirst() {
    if (this.head.next) {
      let newHead = this.head.next;
      this.head = newHead;
    } else {
      this.head = null;
    }
  }

  removeLast() {
    if (!this.head) {
      return;
    }

    if (!this.head.next) {
      this.head = null;
      return;
    }

    let previous = this.head;
    let current = this.head.next;
    while (current.next) {
      previous = current;
      current = current.next;
    }

    previous.next = null;
  }

  insertLast(val) {
    let last = this.getLast();

    if (last) {
      last.next = new Node(val);
    } else {
      this.head = new Node(val);
    }
  }

  getAt(index) {
    let counter = 0;
    let node = this.head;
    while (node) {
      if (counter === index) {
        return node;
      }

      counter++;
      node = node.next;
    }
    return null;
  }

  removeAt(index) {
    if (!this.head) {
      return;
    }

    if (index === 0) {
      this.head = this.head.next;
      return;
    }
    
    let previous = this.getAt(index - 1);
    if (!previous || !previous.next) {
      return;
    }
    previous.next = previous.next.next;
  }

  insertAt(val, index) {
    if (!this.head) {
      this.head = new Node(val);
      return;
    }
  
    if (index === 0) {
      this.head = new Node(val, this.head);
      return;
    }

    if (index >= this.size()) {
      this.insertLast(val);
      return;
    }

    let previous = this.getAt(index - 1);
    let newNode = new Node(val, previous.next);
    previous.next = newNode;
  }

  forEach(fn) {
    let current = this.head;
    let counter = 0;

    while (current) {
      fn(current, counter);
      current = current.next;
      counter++;
    }
  }

  *[Symbol.iterator]() {
    let current = this.head;
    while (current) {
      yield current;
      current = current.next;
    }
  }
}

module.exports = { Node, LinkedList };

let l = new LinkedList();
l.insertLast('a');
l.insertLast('b');
l.insertLast('c');
l.insertAt('hi', 1);
console.log(`initial ll: ${JSON.stringify(l)}`);
// l.insertAt('z', 3);
// console.log(l.getLast());

