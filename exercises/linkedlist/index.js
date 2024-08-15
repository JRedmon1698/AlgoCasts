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
    
  }
}

module.exports = { Node, LinkedList };

let l = new LinkedList();
// console.log('empty ll: ', l);
l.insertFirst('a');
l.insertFirst('b');
l.insertFirst('c');
// console.log(l);
// l.removeFirst();
// console.log(l);
// l.removeFirst();
// console.log(l);


