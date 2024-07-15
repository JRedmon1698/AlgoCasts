<?php

function pyramid($iNum){
    if($iNum == 1){
        return '#';
    }

    $sBase = '';
    $iBaseLength = ($iNum * 2) - 1;
    for($i = 0; $i < $iBaseLength; $i++){
        $sBase .= '#';
    }

    $iLength = strlen($sBase);
    $sDisplayStr = $sBase;
    $iStart = floor($iLength / 2);
    for($i = $iStart; $i >= 0; $i--){
        var_dump(stringSlice($sDisplayStr, $i));
    }
}

function stringSlice($sStr, $iNum){
    $sSpaces = '';
    $iCounter = 1;
    $iLength = strlen($sStr);

    while($iCounter <= $iNum){
        $sSpaces .= ' ';
        $iCounter++;
    }

    $iBaseLength = $iLength - ($iNum * 2);

    return $sSpaces.substr($sStr, $iNum, $iBaseLength).$sSpaces;
}

// var_dump(stringSlice('#####', 2));
var_dump(pyramid(3));
