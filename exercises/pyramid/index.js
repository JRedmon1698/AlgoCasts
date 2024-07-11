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

function pyramid(n) {
  // start with base string that has n #'s
  // iterate from 0..n and slice off i to -i and add spaces

  let base_string = '';
  for (let i = 0; i < n; i++) {
    base_string += '#';
  } 
  // console.log(base_string)
  let legnth = base_string.length;

  let display_string = base_string;
  for (let i = length - 1; i >= 0; i--) {
    
  }
}

function addSpaces(n) {
  
}

pyramid(4);

module.exports = pyramid;
