// --- Directions
// Write a function that accepts a string.  The function should
// capitalize the first letter of each word in the string then
// return the capitalized string.
// --- Examples
//   capitalize('a short sentence') --> 'A Short Sentence'
//   capitalize('a lazy fox') --> 'A Lazy Fox'
//   capitalize('look, it is working!') --> 'Look, It Is Working!'

function capitalize(str) {
  // easy impl:
  // split each sent at space
  // iterate over arr of words
  // iterate over each word
  // capitalize first char
  // join arr on space to create final string

  let words = str.split(' ');
  // console.log('arr of words: ', words);

  for (i = 0; i < words.length; i++) {
    let capFirstChar = words[i][0].toUpperCase();
    let capWord = `${capFirstChar}${words[i].substr(1)}`;
    words[i] = capWord;
  }

  return words.join(' ');
}

console.log(capitalize('i love breakfast at bill miller bbq'));
console.log(capitalize('hi there, how is it going?'));


module.exports = capitalize;
