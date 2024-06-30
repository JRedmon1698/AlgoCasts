<?php

function chunks($aArr, $iSize){
    $aResArr = [];

    foreach($aArr as $iElem){
        $iLastIdx = count($aResArr) - 1;
        if($iLastIdx < 0 || count($aResArr[$iLastIdx]) == $iSize){
            $aResArr[] = [$iElem];
        } else {
            $aResArr[$iLastIdx][] = $iElem;
        }
    }

    return $aResArr;
}

chunks([1, 2, 3], 2);
