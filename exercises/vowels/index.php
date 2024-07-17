<?php

const VOWELS = 'aeiou';

function vowels($str){
    $iCount = 0;
    $aSplitStr = str_split($str);

    foreach($aSplitStr as $sChar){
        if(strpos(VOWELS, strtolower($sChar))){
            $iCount++;
        }
    }

    return $iCount;
}

var_dump(vowels('HELLO THERE'));
