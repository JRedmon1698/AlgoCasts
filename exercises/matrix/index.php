<?php

<<<<<<< HEAD
function matrix($iNum) {
    // make container and fill with $iNum arrays

    // inst all vars

    // iterate left to right

    // iterate top to bottom

    // iterate right to left

    // iterate bottom to top
=======
function matrix($iNum){
    // inst container
    $aContainer = [];
    // iterate to iNum and fill container with arrays
    for($i = 0; $i < $iNum; $i++){
        $aContainer[] = [];
    }

    // inst vars
    $iNumber = 1;
    $iLeftCol = 0;
    $iRightCol = $iNum - 1;
    $iTopRow = 0;
    $iBottomRow = $iNum - 1;

    // start while loop to check start col/row <= end col/row
    while($iNumber <= $iNum * $iNum){
        // for loop, incr cols, setting startRow
        for($i = $iLeftCol; $i <= $iRightCol; $i++){
            $aContainer[$iTopRow][$i] = $iNumber;
            $iNumber++;
        }
        $iTopRow++;

        // for loop, incr rows, setting endCol
        for($i = $iTopRow; $i <= $iBottomRow; $i++){
            $aContainer[$i][$iRightCol] = $iNumber;
            $iNumber++;
        }
        $iRightCol--;

        // for loop, decr cols, setting endRow
        for($i = $iRightCol; $i >= $iLeftCol; $i--){
            $aContainer[$iBottomRow][$i] = $iNumber;
            $iNumber++;
        }
        $iBottomRow--;

        // for loop, decr rows, setting startCol
        for($i = $iBottomRow; $i >= $iTopRow; $i--){
            $aContainer[$i][$iLeftCol] = $iNumber;
            $iNumber++;
        }
        $iLeftCol++;
    }

    return $aContainer;
>>>>>>> fd88b52e7cead0d0207bb8c6cafd2c855e0b1b1a
}

print_r(matrix(3));
