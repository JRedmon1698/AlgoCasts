<?php

function reverse($str){
    $aStrArr = str_split($str);
    // var_dump($aStrArr);

    $aResArr = [];
    $iSize = count($aStrArr);
    for($i = $iSize - 1; $i >= 0; $i--){
        $aResArr[] = $aStrArr[$i];
    }

    return implode('', $aResArr);
}

var_dump(reverse('hello'));
