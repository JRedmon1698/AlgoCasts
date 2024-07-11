// --- Directions
// Write a function that accepts a positive number N.
// The function should console log a pyramid shape
// with N levels using the # character.  Make sure the
// pyramid has spaces on both the left *and* right hand sides
// --- Examples
//   pyramid(1)
//       '#'
//   pyramid(2)
//       ' # '
//       '###'
//   pyramid(3)
//       '  #  '
//       ' ### '
//       '#####'
// 4
// ' 1 '
// ' 3 '
// ' 5 '
// ' 7 '

function pyramid(n) {
  if (n === 1) {
    console.log('#');
    return;
  }

  let base_string = '';
  let base_string_length = (n * 2) - 1;

  for (let i = 0; i < base_string_length; i++) {
    base_string += '#';
  } 

  let length = base_string.length;
  // since we're iterating backwards, dont start slicing until we have something to slice
  let startingPoint = Math.floor(length / 2);
  let displayString = base_string;
  
  for (let i = startingPoint; i >= 0; i--) {
    console.log(returnStrSlice(displayString, i));
  }
}

function returnStrSlice(str, n) {
  let spaces = '';
  let length = str.length;

  let counter = 1;
  while(counter <= n) {
    spaces += ' ';
    counter++;
  }

  return `${spaces}${str.slice(n, length - n)}${spaces}`;
}
// console.log('slice', returnStrSlice('#######', 3))

pyramid(6);

module.exports = pyramid;
