<?php

function palindrome($str){
    $aSplitStr = str_split($str);
    // var_dump($aSplitStr);

    $aRevArr = [];
    $iLength = count($aSplitStr);
    for($i = $iLength - 1; $i >=0; $i--){
        $aRevArr[] = $aSplitStr[$i];
    }

    $sRevStr = implode('', $aRevArr);

    return $str === $sRevStr;
}

var_dump(palindrome('abc'));
var_dump(palindrome('abccba'));
