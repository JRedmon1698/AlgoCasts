<?php

function fizzBuzz($iNum){
    for($i = 0; $i <= $iNum; $i++){
        switch ($i) {
            case (($i % 3 === 0) && ($i % 5 === 0)):
                echo 'fizzbuzz';
                break;
            case ($i % 3 === 0):
                echo 'fizz';
                break;
            case ($i % 5 === 0):
                echo'buzz';
                break;
            default:
                echo $i;        
        }
        echo "\n";
    }
}

fizzBuzz(30);
