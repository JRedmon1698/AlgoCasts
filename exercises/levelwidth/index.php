<?php

class Node {
    public $data;
    public $children;

    function __construct($data){
        $this->data = $data;
        $this->children = [];
    }

    function add($data){
        $this->children[] = new Node($data);
    }
}

$node = new Node(1);
$node->add(3);
$node->add(5);
var_dump($node);

