<?php

function capitalize($sSentence){
    $aWords = explode(' ', $sSentence);
    $aCapitalizedWords = [];

    foreach($aWords as $word){
        $word = ucfirst($word);
        $aCapitalizedWords[] = $word;
    }

    return implode(' ', $aCapitalizedWords);
}

var_dump(capitalize('why, hello there!'));
