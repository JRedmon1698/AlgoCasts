<?php

function matrix($iNum){
    // inst container
    $aContainer = [];
    // iterate to iNum and fill container with arrays
    for($i = 0; $i < $iNum; $i++){
        $aContainer[] = [];
    }

    var_dump($aContainer);

    // inst vars
    $iNum = 1;
    $iLeftCol = 0;
    $iRightCol = 0;
    $iTopRow = 0;
    $iBottomRow = 0;

    // start while loop to check start col/row <= end col/row
    while($iNum <= $iNum * $iNum){
        // for loop, incr cols, setting startRow
        for($i = $iLeftCol; $i <= $iRightCol; $i++){
            $aContainer[$iTopRow][$i] = $iNum;
            $iNum++;
        }
        $iTopRow++;

        // for loop, incr rows, setting endCol
        for($i = $iTopRow; $i <= $iBottomRow; $i++){
            $aContainer[$i][$iRightCol] = $iNum;
            $iNum++;
        }
        $iRightCol--;

        // for loop, decr cols, setting endRow
        for($i = $iRightCol; $i >= $iLeftCol; $i--){
            $aContainer[$iBottomRow][$i] = $iNum;
            $iNum++;
        }
        $iBottomRow--;

        // for loop, decr rows, setting startCol
        for($i = $iBottomRow; $i >= $iTopRow; $i--){
            $aContainer[$i][$iLeftCol] = $iNum;
            $iNum++;
        }
        $iLeftCol++;
    }

    return $aContainer;
}

var_dump(matrix(3));
