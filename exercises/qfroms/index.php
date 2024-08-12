<?php

class Stack {
    protected $aData;

    function __construt(){
        $this->aData = [];
    }

    function push($val){
        $this->aData[] = $val;
    }

    function pop(){
        return array_pop($this->aData);
    }

    function peek(){
        return end($this->aData);
    }
}

class Queue {
    protected $oDataA;
    protected $oDataB;

    function __construct(){
        $this->oDataA = new Stack();
        $this->oDataB = new Stack();
    }

    function add($val){
        $this->oDataA->push($val);
    }

    function remove(){
        while($this->oDataA->peek()){
            $this->oDataB->push($this->oDataA->pop());
        }

        $iPopped = $this->oDataB->pop();

        while($this->oDataB->peek()){
            $this->oDataA->push($this->oDataB->pop());
        }

        return $iPopped;
    }

    function peek(){
        while($this->oDataA->peek()){
            $this->oDataB->push($this->oDataA->pop());
        }

        $iPeeked = $this->oDataB->peek();

        while($this->oDataB->peek()){
            $this->oDataA->push($this->oDataB->pop());
        }

        return $iPeeked;
    }
}
