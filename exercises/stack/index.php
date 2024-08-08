<?php

class Stack {
    public $aData = [];

    public function __construct(){
        return $this->aData;
    }

    public function push($val){
        for($i = count($this->aData); $i > 0; $i--){
            $this->aData[$i] = $this->aData[$i - 1];
        }

        $this->aData[0] = $val;
    }

    public function pop(){
        if(count($this->aData) < 1){
            return null;
        }

        $top = reset($this->aData);

        for($i = 0; $i <= count($this->aData) - 1; $i++){
            $this->aData[$i] = $this->aData[$i + 1];  
        }

        return $top;
    }
}

$oStack = new Stack();
$oStack->push(1);
$oStack->push(2);
$oStack->push(3);
var_dump($oStack);
