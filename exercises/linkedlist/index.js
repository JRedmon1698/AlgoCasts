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
    this.nodes = [];
    this.head = null;
    this.tail = null;
  }

  insertFirst(data) {
    let node = new Node(data);
    this.head = node;
    this.nodes.push(node);
    // nope. need to cover the case where there is only one node 
    // its head AND tail
  }

  size() {
    return this.nodes.length;
  }

  getFirst() {
    return this.head;
  }

  getLast() {
    return this.tail;
  }
}

module.exports = { Node, LinkedList };

let l = new LinkedList();
// console.log('empty ll: ', l);
l.insertFirst('one');
l.insertFirst('two');
// console.log(l);
// console.log(l.size())
