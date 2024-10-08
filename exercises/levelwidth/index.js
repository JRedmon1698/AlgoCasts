// --- Directions
// Given the root node of a tree, return
// an array where each element is the width
// of the tree at each level.
// --- Example
// Given:
//     0
//   / |  \
// 1   2   3
// |       |
// 4       5
// Answer: [1, 3, 2]

function levelWidth(root) {
  const stop = 'BALLAST';
  let counters = [0];
  let nodes = [root, stop];

  while (nodes.length > 1) {
    let node = nodes.shift();

    if (node === stop) {
      counters.push(0);
      nodes.push(stop);
    } else {
      nodes.push(...node.children);
      counters[counters.length - 1]++;
    }
  }

  return counters;
}

module.exports = levelWidth;
