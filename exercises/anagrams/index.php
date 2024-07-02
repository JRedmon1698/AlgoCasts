<?php

function anagrams($sA, $sB){
    $aCharArrA = array_map(function($c){
        return strtolower($c);
    }, array_filter(str_split($sA), function($c){
        return ctype_alpha($c);
    }));
    sort($aCharArrA);

    $aCharArrB = array_map(function($c){
        return strtolower($c);
    }, array_filter(str_split($sB), function($c){
        return ctype_alpha($c);
    }));
    sort($aCharArrB);

    return $aCharArrA == $aCharArrB;
}

var_dump(anagrams('Hello There!', 'threelleho'));
