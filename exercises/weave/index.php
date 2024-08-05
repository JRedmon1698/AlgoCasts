<?php

class Queue {
    public $aData = [];

    public function __construct(){
        return $this->aData;
    }

    public function add($iVal){
        array_unshift($this->aData, $iVal);
    }

    public function remove(){
        array_pop($this->aData);
    }

    public function peek(){
        return end($this->aData);
    }
}

$oQueue = new Queue();

$oQueue->add(1);
$oQueue->add(2);
$oQueue->add(3);

$oQueueTwo = new Queue();
$oQueueTwo->add('one');
$oQueueTwo->add('two');
$oQueueTwo->add('three');

function weave($oQueueA, $oQueueB){
    $oWeavedQueue = new Queue();

    for($i = count($oQueueA->aData) - 1; $i >= 0; $i--){
            $oWeavedQueue->add($oQueueA->aData[$i]);
            
        for($j = count($oQueueB->aData) - 1; $j >= 0; $j--){
            $oWeavedQueue->add($oQueueB->aData[$i]);
            break;
        }
    }

    return $oWeavedQueue;
}

var_dump(weave($oQueue, $oQueueTwo));
