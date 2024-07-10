<?php

function steps($iNum){
    $iCounter = 1;
    $sStr = '';

    while($iCounter <= $iNum){
        $sStr .= '#';
        $iCounter++;
    }

    $sDisplayStr = $sStr;
    for($i = strlen($sStr) - 1; $i >= 0; $i--){
        var_dump(substr($sDisplayStr, $i) . addSpace($i));
    }
}

function addSpace($iNum){
    $iCounter = 0;
    $sSpaces = '';
    while($iCounter < $iNum){
        $sSpaces = $sSpaces .= ' ';
        $iCounter++;
    }

    return $sSpaces;
}

// var_dump(addSpace(3));
var_dump(steps(3));
