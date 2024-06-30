<?php

function reverseInt($int){    
    $sInt = strval($int);
    $aStrArr = str_split($sInt);

    $aRevArr = [];
    $iLength = count($aStrArr);
    for($i = $iLength - 1; $i >= 0; $i--){
        $aRevArr[] = $aStrArr[$i];
    }

    $iReversed = intval(implode('', $aRevArr));

    return $int < 0 ? -$iReversed : $iReversed;
}

var_dump(reverseInt(123));
var_dump(reverseInt(-120));
