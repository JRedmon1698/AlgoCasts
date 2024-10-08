// --- Directions
// Print out the n-th entry in the fibonacci series.
// The fibonacci series is an ordering of numbers where
// each number is the sum of the preceeding two.
// For example, the sequence
//  [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
// forms the first ten entries of the fibonacci series.
// Example:
//   fib(4) === 3

// function fib(n) {
//   if (n === 0) {
//     return 0;
//   }
//   if (n === 1) {
//     return 1;
//   }

//   let curr = 0;
//   let previous = 1;
//   let secondPrevious = 0;

//   for (let i = 2; i <= n; i++) {
//     // console.log(curr)
//     curr = previous + secondPrevious;
//     secondPrevious = previous;
//     previous = curr;
//   }

//   return curr;
// }

function fib(n) {
  if (n === 0) {
    return 0;
  }
  if (n === 1) {
    return 1;
  }

  return fib(n - 1) + fib(n - 2);
}

function memoize(func) {
  let cache = {};
  return function(...args) {
    if (cache[args]) {
      return cache[args];
    }

    let result = func.apply(this, args);
    cache[args] = result;

    return result;
  }
}

fib = memoize(fib);

console.time('fib');
console.log(fib(50));
console.timeEnd('fib');
module.exports = fib;
