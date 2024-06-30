<?php

function maxChars($str){
    $oCharMap = [];

    $aSplit = str_split($str);

    $sMaxChar = '';
    $iMaxCharNum = 0;

    foreach($aSplit as $c){
        if(!isset($oCharMap[$c])){
            $oCharMap[$c] = 1;
        }
        else {
            $oCharMap[$c] += 1;
        }

        if($oCharMap[$c] > $iMaxCharNum){
            $iMaxCharNum = $oCharMap[$c];
            $sMaxChar = $c;
        }
    }
    
    return $sMaxChar;
}

$start = microtime(true);
var_dump(maxChars("abca"));
$end = microtime(true);
var_dump("Time: ", $end - $start);

// var_dump(maxChars("1231"));
