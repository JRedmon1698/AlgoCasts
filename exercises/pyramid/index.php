<?php

function pyramid($iNum){
    
}

function stringSlice($sStr, $iNum){
    $sSpaces = '';
    $iCounter = 1;
    $iLength = strlen($sStr);

    while($iCounter <= $iNum){
        $sSpaces .= 'f';
        $iCounter++;
    }

    $iBaseLength = $iLength - ($iNum * 2);

    return substr($sStr, $iNum, $iBaseLength);
}

// var_dump(stringSlice('#####', 2));
