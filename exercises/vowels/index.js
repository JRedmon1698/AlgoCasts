// --- Directions
// Write a function that returns the number of vowels
// used in a string.  Vowels are the characters 'a', 'e'
// 'i', 'o', and 'u'.
// --- Examples
//   vowels('Hi There!') --> 3
//   vowels('Why do you ask?') --> 4
//   vowels('Why?') --> 0

function vowels(str) {
  const VOWELS = ['a', 'e', 'i', 'o', 'u'];
  let count = 0;
  
  for (let char of str) {
    if (VOWELS.indexOf(char.toLowerCase()) !== -1) {
      count++;
    }
  }

  return count;
}

console.log(vowels("AEIOU"));

module.exports = vowels;
