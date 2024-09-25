// --- Directions
// Implement bubbleSort, selectionSort, and mergeSort

function bubbleSort(arr) {
  for (let i = 0; i < arr.length - 1; i++) {
    for (let k = 0; k < arr.length - i; k++) {
      if (arr[k] > arr[k + 1]) {
        let prev = arr[k];
        arr[k] = arr[k + 1];
        arr[k + 1] = prev;
      }
    }
  }
  return arr;
}

function selectionSort(arr) {

}

function mergeSort(arr) {

}

function merge(left, right) {

}

module.exports = { bubbleSort, selectionSort, mergeSort, merge };

let arr = [4, -1, 10, 3, 7];
let sorted = bubbleSort(arr);
console.log(sorted);
