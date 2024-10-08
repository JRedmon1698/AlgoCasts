// --- Directions
// 1) Complete the task in weave/queue.js
// 2) Implement the 'weave' function.  Weave
// receives two queues as arguments and combines the
// contents of each into a new, third queue.
// The third queue should contain the *alterating* content
// of the two queues.  The function should handle
// queues of different lengths without inserting
// 'undefined' into the new one.
// *Do not* access the array inside of any queue, only
// use the 'add', 'remove', and 'peek' functions.
// --- Example
//    const queueOne = new Queue();
//    queueOne.add(1);
//    queueOne.add(2);
//    const queueTwo = new Queue();
//    queueTwo.add('Hi');
//    queueTwo.add('There');
//    const q = weave(queueOne, queueTwo);
//    q.remove() // 1
//    q.remove() // 'Hi'
//    q.remove() // 2
//    q.remove() // 'There'

const Queue = require('./queue');

const q1 = new Queue();
q1.add(1);
q1.add(2);
q1.add(3);

const q2 = new Queue();
q2.add('one');
q2.add('two');
q2.add('three');

function weave(sourceOne, sourceTwo) {
  let weavedQueue = new Queue();
  
  for (let i = sourceOne.data.length - 1; i >= 0; i--) {
    weavedQueue.add(sourceOne.data[i]);

    for (let j = sourceTwo.data.length - 1; j >= 0; j--) {
      weavedQueue.add(sourceTwo.data[i]);
      break;
    }
  };
  
  return weavedQueue;
}

let q3 = weave(q1, q2);
console.log(q3.remove());
console.log(q3.remove());

module.exports = weave;
