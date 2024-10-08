// --- Directions
// 1) Create a node class.  The constructor
// should accept an argument that gets assigned
// to the data property and initialize an
// empty array for storing children. The node
// class should have methods 'add' and 'remove'.
// 2) Create a tree class. The tree constructor
// should initialize a 'root' property to null.
// 3) Implement 'traverseBF' and 'traverseDF'
// on the tree class.  Each method should accept a
// function that gets called with each element in the tree

class Node {
  constructor(data) {
    this.data = data;
    this.children = [];
  }

  add(data) {
    this.children.push(new Node(data));
  }

  remove(data) {
    this.children = this.children.filter(d => d.data !== data);
  }
}

class Tree {
  constructor() {
    this.root = null;
  }

  traverseBF(func) {
    let nodes = [this.root];

    while (nodes.length) {
      let node = nodes.shift();
      nodes.push(...node.children);
      func(node);
    }
  }

  traverseDF(func) {
    let nodes = [this.root];

    while (nodes.length) {
      let node = nodes.shift();
      nodes.unshift(...node.children);
      func(node);
    }
  }
}

module.exports = { Tree, Node };

let n = new Node(1);
console.log(n);
n.add(2);
console.log(n)
