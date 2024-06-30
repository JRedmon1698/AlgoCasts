// --- Directions
// Given an integer, return an integer that is the reverse
// ordering of numbers.
// --- Examples
//   reverseInt(15) === 51
//   reverseInt(981) === 189
//   reverseInt(500) === 5
//   reverseInt(-15) === -51
//   reverseInt(-90) === -9

function reverseInt(n) {
  let reverseIntStr = n.toString().split('').reverse().join('');

  let parsedInt = parseInt(reverseIntStr);

  // if original num is neg, return neg num
  return n < 0 ? -parsedInt : parsedInt;
}

// console.log(reverseInt(-1500))
module.exports = reverseInt;
