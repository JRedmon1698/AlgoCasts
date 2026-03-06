// --- Directions
// Implement bubbleSort, selectionSort, and mergeSort

function bubbleSort(arr) {
  for (let i = 0; i < arr.length - 1; i++) {
    for (let k = 0; k < arr.length - i - 1; k++) {
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
  for (let i = 0; i < arr.length; i++) {
    let indexOfMin = i;

    for (let k = i + 1; k < arr.length; k++) {
      if (arr[indexOfMin] > arr[k]) {
        indexOfMin = k;
      }
    }

    if (indexOfMin !== i) {
      let lesser = arr[indexOfMin];
      arr[indexOfMin] = arr[i];
      arr[i] = lesser;
    }
  }

  return arr;
}

function mergeSort(arr) {
  if (arr.length === 1) {
    return arr;
  }

  const mid = Math.floor(arr.length / 2);
  let left = arr.slice(0, mid);
  let right = arr.slice(mid);

  return merge(mergeSort(left), mergeSort(right));
}

function merge(left, right) {
  const results = [];

  while (left.length && right.length) {
    if (left[0] < right[0]) {
      results.push(left.shift());
    } else {
      results.push(right.shift());
    }
  }

  return [...results, ...left, ...right];
}

let left = [4, -1, 10, 3, 7];
let right = [12, 13, 3, -5, 2, 5];
let merged = merge(left, right);
// console.log(merged);

function quickSort(arr, start, end) {
  start = start ?? 0;
  end = end ?? arr.length - 1;

  if (start < end) {
    const pivot = arr[end];
    let left = start - 1;

    for (let current = start; current < end; current++) {
      if (arr[current] <= pivot) {
        left++;
        [arr[left], arr[current]] = [arr[current], arr[left]];
      }
    }

    [arr[left + 1], arr[end]] = [arr[end], arr[left + 1]];

    const pivotIndex = left + 1;

    quickSort(arr, start, pivotIndex - 1);
    quickSort(arr, pivotIndex + 1, end);
  }

  return arr;
}

console.log(quickSort([2,3,1,1,8,5,4]));

module.exports = { bubbleSort, selectionSort, mergeSort, merge, quickSort };
