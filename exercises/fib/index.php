<?php

// function fib($iNum){
//     if($iNum == 0){
//         return 0;
//     }
//     if($iNum == 1){
//         return 1;
//     }

//     return fib($iNum - 1) + fib($iNum - 2);
// }

function fib($iNum){
    if($iNum == 0){
        return 0;
    }
    if($iNum == 1){
        return 1;
    }

    $iCurrent = 0;
    $iPrevious = 1;
    $iSecondPrevious = 0;
    for($i = 2; $i <= $iNum; $i++){
        $iCurrent = $iPrevious + $iSecondPrevious;
        $iSecondPrevious = $iPrevious;
        $iPrevious = $iCurrent;
        
    }

    return $iCurrent;
}

var_dump(fib(6));
