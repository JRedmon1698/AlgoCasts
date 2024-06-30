// --- Directions
// Given a string, return the character that is most
// commonly used in the string.
// --- Examples
// maxChar("abcccccccd") === "c"
// maxChar("apple 1231111") === "1"

function maxChar(str) {
  let charMap = {};

  let splitStr = str.split('');

  let maxChar;
  let maxCharNum = 0;

  splitStr.forEach(c => {
    if (charMap[c] === undefined) {
      charMap[c] = 1;
    } else {
      charMap[c] += 1;
    }

    if (charMap[c] > maxCharNum) {
      maxCharNum = charMap[c];
      maxChar = c;
    }
  });

  return maxChar;
}

let start = performance.now();
console.log(maxChar('abcdefghijklmnaaaaa'))
let end = performance.now();
console.log(`Time: ${end - start}`);

module.exports = maxChar;
