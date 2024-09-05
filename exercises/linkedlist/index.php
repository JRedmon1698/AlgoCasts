<?php

class Node {
    public $data;
    public $next;
    
    function __construct($data, $next = null){
       $this->data = $data;
       $this->next = $next;
    }
}

class LinkedList {
    public $head;
    
    function __construct(){
       $this->head = null;
    }

    function insertFirst($data){
       $this->head = new Node($data, $this->head);
    }

    function size(){
        $count = 0;
        $current = $this->head;

        while(isset($current)){
            $current = $current->next;
            $count++;
        }

        return $count;
    }

    function getFirst(){
        return $this->head;
    }

    function getLast(){
        if(!isset($this->head)){
            return null;
        }

        $current = $this->head;
        while(isset($current->next)){
            $current = $current->next;
        }

        return $current;
    }

    function clear(){
        $this->head = null;
    }

    function removeFirst(){
        if(!isset($this->head)){
            return;
        }
        else {
            if(isset($this->head->next)){
                $this->head = $this->head->next;
            } else {
                $this->clear();
            }
        }
    }

    function removeLast(){
        if(!isset($this->head)){
            return;
        }

        if(!isset($this->head->next)){
            $this->head = null;
        }

        $current = $this->head->next;
        $previous = $this->head;
        while(isset($current->next)){
            $previous = $current;
            $current = $current->next;
        }

        $previous->next = null;
    }

    function insertLast($data){
        $last = $this->getLast();

        if(isset($last)){
            $last->next = new Node($data);
        }
        else {
            $this->head = new Node($data);
        }
    }
}

$list = new LinkedList();
$list->insertFirst(1);
$list ->insertFirst(2);
$list->insertFirst(3);
var_dump($list);
var_dump($list->insertLast(9));
var_dump('after', $list);
