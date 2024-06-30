// --- Directions
// Given a string, return a new string with the reversed
// order of characters
// --- Examples
//   reverse('apple') === 'leppa'
//   reverse('hello') === 'olleh'
//   reverse('Greetings!') === '!sgniteerG'

function reverse(str) {
  if (str.length < 2) {
    return str;
  }

  let strArr = str.split('');
  let resStr = '';

  for (let i = strArr.length - 1; i >= 0; i--) {
    resStr += strArr[i]
  }

  return resStr;
}

module.exports = reverse;
