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
  let counter = 1;
  let startCol = 0;
  let startRow = 0;
  let endCol = n - 1; 
  let endRow = n -1;
  let matrix = [];

  for(let i = 0; i < n; i++) {
    matrix.push([]);
  }

  while (startCol <= endCol && startRow <= endRow) {
    // top row
    for (let i = startCol; i <= endCol; i++) {
      matrix[startRow][i] = counter;
      counter++;
    }
    startRow++;

    // right column
    for (let k = startRow; k <= endRow; k++) {
      matrix[k][endCol] = counter;
      counter++;
    }
    endCol--;

    // bottom row
    for (let j = endCol; j >= startCol; j--) {
      matrix[endRow][j] = counter;
      counter++;     
    }
    endRow--;

    // left column
    for (let l = endRow; l >= startRow; l--) {
      matrix[l][startCol] = counter;
      counter++;
    }
    startCol++;
  }

  return matrix;
}

console.log(matrix(3));

module.exports = matrix;
