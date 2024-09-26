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

// var_dump(bubbleSort($arr));

function selectSort($arr){
    for($i = 0; $i < count($arr) - 1; $i++){
        $idxOfMin = $i;

        for($k = $i + 1; $k < count($arr) - 1; $k++){
            if($arr[$k] < $arr[$idxOfMin]){
                $idxOfMin = $k;
            }
        }

        if($idxOfMin !== $i){
            $lesser = $arr[$idxOfMin];
            $arr[$idxOfMin] = $arr[$i];
            $arr[$i] = $lesser;
        }
    }

    return $arr;
}

// var_dump(selectSort($arr));

function mergeSort($arr){
    if(count($arr) === 1){
        return $arr;
    }

    $midpoint = floor(count($arr) / 2);
    $left = array_slice($arr, 0, $midpoint);
    $right = array_slice($arr, $midpoint);

    return merge(mergeSort($left), mergeSort($right));
}

function merge($left, $right){
    $result = [];

    while(!empty($left) && !empty($right)){
        if($left[0] < $right[0]){
            $result[] = array_shift($left);
        } else {
          $result[] = array_shift($right);  
        }
    }

    return [...$result, ...$left, ...$right];
}

var_dump(mergeSort($arr));
