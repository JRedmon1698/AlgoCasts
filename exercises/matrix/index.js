// --- Directions
// Write a function that accepts an integer N
// and returns a NxN spiral matrix.
// --- Examples
//   matrix(2)
//     [[1, 2],
//     [4, 3]]
//   matrix(3)
//     [[1, 2, 3],
//     [8, 9, 4],
//     [7, 6, 5]]
//  matrix(4)
//     [[1,   2,  3, 4],
//     [12, 13, 14, 5],
//     [11, 16, 15, 6],
//     [10,  9,  8, 7]]

function matrix(n) {
  let matrix = [];
  // for 1..n
  // iterate over range adding each num to inner arr
  // when innter arr reaches length n, start over
  // fill first row
  // fill last col
  // fill last row
  // fill first col - 1
  // fill 2nd row - 1
  // fill 2nd to last col - 2
  // fill 2nd to last row - 2
  let innerMatrix = [];
  for (let row = 0; row < n; row++) {
    for (let col = 0; col < n; col++) {
      
    }
    matrix.push(innerMatrix);
  }

  return matrix;
}

console.log(matrix(3));

module.exports = matrix;
