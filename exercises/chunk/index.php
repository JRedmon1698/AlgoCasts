<?php

// function chunks($aArr, $iSize){
//     $aResArr = [];

//     foreach($aArr as $iElem){
//         $iLastIdx = count($aResArr) - 1;
//         if($iLastIdx < 0 || count($aResArr[$iLastIdx]) == $iSize){
//             $aResArr[] = [$iElem];
//         } else {
//             $aResArr[$iLastIdx][] = $iElem;
//         }
//     }

//     return $aResArr;
// }

function chunks($aArr, $iSize){
    $aResArr = [];
    $iNewArrMarker = 0;

    while($iNewArrMarker < count($aArr)){
        $aResArr[] = array_slice($aArr, $iNewArrMarker, $iSize);
        $iNewArrMarker += $iSize;
    }

    return $aResArr;
}

chunks([1, 2, 3], 2);
