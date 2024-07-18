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

  for (let i = 0; i < n; i++) {
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

/*
make vars to track:
  start & end col
  start & end row
  counter (this actually populates inner matrices)
  container matrix

iterate to n and fill container arr with n sub arrays
since we will be tracking which row/col we're on, start a while loop
  to check that start col & row <= end col & row
4 for loops, iterating over cols and rows twice, each:
  one loop each incrementing, one each decrementing (ORDER MATTERS)
  - incr col
  - incr row
  - decr col
  - decr row
after each loop, add or subtract start/end row/col:
  - incrementing over rows: decr end col
  - incrementing over cols: incr start row
  - decrementing over rows: incr start col
  - decrementing over cols: decr end row
*/
