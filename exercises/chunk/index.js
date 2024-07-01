// --- Directions
// Given an array and chunk size, divide the array into many subarrays
// where each subarray is of length size
// --- Examples
// chunk([1, 2, 3, 4], 2) --> [[ 1, 2], [3, 4]]
// chunk([1, 2, 3, 4, 5], 2) --> [[ 1, 2], [3, 4], [5]]
// chunk([1, 2, 3, 4, 5, 6, 7, 8], 3) --> [[ 1, 2, 3], [4, 5, 6], [7, 8]]
// chunk([1, 2, 3, 4, 5], 4) --> [[ 1, 2, 3, 4], [5]]
// chunk([1, 2, 3, 4, 5], 10) --> [[ 1, 2, 3, 4, 5]]

// function chunk(array, size) {
//   let resArr = [];
//   let newArrMarker = 0;

//   while (newArrMarker < array.length) {
//     resArr.push(array.slice(newArrMarker, newArrMarker + size));
//     newArrMarker += size;
//   }
//   return resArr;
// }

function chunk(array, size) {
  let resArr = [];

  array.forEach(n => {
    let lastElem = resArr[resArr.length - 1];

    if (!lastElem || lastElem.length === size) {
      resArr.push([n]);
    } else {
      lastElem.push(n);
    }
  });

  // console.log(resArr)
  return resArr;
}

chunk([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13], 5);

module.exports = chunk;
