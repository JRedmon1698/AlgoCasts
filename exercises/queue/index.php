<?php

class Queue {
    public $aData = [];

    public function __construct(){
        return $this->aData;
    }

    public function add($val){
        $this->aData[] = $val;
    }

    public function remove(){
        array_pop($this->aData);
    }
}

$oQueue = new Queue();
var_dump('empty queue: ', $oQueue);

$oQueue->add(1);
$oQueue->add(2);
$oQueue->add(3);
var_dump('full queue: ', $oQueue);

$oQueue->remove();
var_dump('queue minus one: ', $oQueue);
