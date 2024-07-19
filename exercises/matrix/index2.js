// function matrix(n) {
//   let container = [];
//   for (let i = 0; i < n; i++) {
//     container.push([]);
//   }

//   let startCol = 0;
//   let endCol = n - 1;

//   let startRow = 0;
//   let endRow = n - 1;

//   let counter = 1;

//   while (startRow <= endRow && startCol <= endCol) {
//     // incr over cols 
//     for (let i = startCol; i <= endCol; i++) {
//       container[startRow][i] = counter;
//       counter++;
//     }
//     startRow++;
    
//     // incr over rows
//     for (let i = startRow; i <= endRow; i++) {
//       container[i][endCol] = counter;
//       counter++;
//     }
//     endCol--;
  
//     // decr over cols
//     for (let i = endCol; i >= startCol; i--) {
//       container[endRow][i] = counter;
//       counter++;
//     }
//     endRow--;
  
//     // decr over rows    
//     for (let i = endRow; i >= startRow; i--) {
//       container[i][startCol] = counter;
//       counter++;
//     }
//     startCol++;
//   }

//   return container;
// }

function matrix(n) {
  let container = [];
  for (let i = 0; i < n; i++) {
    container.push([]);
  }

  let topRow = 0, bottomRow = n - 1, leftCol = 0, rightCol = n - 1;
  let number = 1;

  while (number <= n * n) {
  for (let i = leftCol; i <= rightCol; i++) {
    container[topRow][i] = number;
    number++;
  }
  topRow++;

  for (let i = topRow; i <= bottomRow; i++) {
    container[i][rightCol] = number;
    number++;
  }
  rightCol--;

  for (let i = rightCol; i >= leftCol; i--) {
    container[bottomRow][i] = number;
    number++;
  }
  bottomRow--;

  for (let i = bottomRow; i >= topRow; i--) {
    container[i][leftCol] = number;
    number++;
  }
  leftCol++;
  }
  return container;
}

console.log(matrix(3))
