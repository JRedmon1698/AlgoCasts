// --- Directions
// Write a function that accepts a positive number N.
// The function should console log a step shape
// with N levels using the # character.  Make sure the
// step has spaces on the right hand side!
// --- Examples
//   steps(2)
//       '# '
//       '##'
//   steps(3)
//       '#  '
//       '## '
//       '###'
//   steps(4)
//       '#   '
//       '##  '
//       '### '
//       '####'

function steps(n) {
  let str = '';
  let counter = 1;

  while (counter <= n) {
    str += '#';
    counter++;
  }

  let logStr = str;
  for (let i = str.length - 1; i >= 0; i--) {
    console.log(logStr.slice(i) + addSpace(i));
  }
}

function addSpace(n) {
  let str = '';
  let counter = 0;
  while (counter < n) {
    str += ' ';
    counter++;
  }

  return str;
}

// function steps(n) {
//   for (let row = 0; row < n; row++) {
//     let stair = '';

//     for (let col = 0; col < n; col++) {
//       if (col <= row) {
//         stair += '#';
//       } else {
//         stair += ' ';
//       }
//     }
//     console.log(stair);
//   }
// }


console.log(steps(3))

module.exports = steps;
