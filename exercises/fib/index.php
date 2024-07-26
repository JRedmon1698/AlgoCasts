<?php

function fib($iNum){
    if($iNum == 0){
        return 0;
    }
    if($iNum == 1){
        return 1;
    }

    return fib($iNum - 1) + fib($iNum - 2);
}

// function fib($iNum){
//     if($iNum == 0){
//         return 0;
//     }
//     if($iNum == 1){
//         return 1;
//     }

//     $iCurrent = 0;
//     $iPrevious = 1;
//     $iSecondPrevious = 0;
//     for($i = 2; $i <= $iNum; $i++){
//         $iCurrent = $iPrevious + $iSecondPrevious;
//         $iSecondPrevious = $iPrevious;
//         $iPrevious = $iCurrent;
        
//     }

//     return $iCurrent;
// }

function memoize(callable $func){
    $oCache = [];
    return function(...$aArgs) use($func, &$oCache){
        $key = json_encode($aArgs, JSON_NUMERIC_CHECK);
        if(isset($oCache[$key])){
            return $oCache[$key];
        }

        $oResult = call_user_func_array($func, $aArgs);
        $oCache[$key] = $oResult;

        return $oResult;
    };
}

$mMemoizedFib = memoize('fib');
$iStartTime = microtime(true);
var_dump($mMemoizedFib(50));
// var_dump(fib(50));
$iEndTime = microtime(true);
var_dump($iEndTime - $iStartTime);
