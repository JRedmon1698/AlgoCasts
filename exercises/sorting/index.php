<?php

function bubbleSort($arr){
    for($i = 0; $i < count($arr) - 1; $i++){
        for($k = 0; $k < count($arr) - $i - 1; $k++){
            if($arr[$k] > $arr[$k + 1]){
                $previous = $arr[$k];
                $arr[$k] = $arr[$k + 1];
                $arr[$k + 1] = $previous;
            }
        }
    }

    return $arr;
}

$arr = [10, 0, 3, -2, 5];

var_dump(bubbleSort($arr));
