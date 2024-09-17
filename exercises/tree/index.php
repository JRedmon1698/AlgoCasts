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

    function remove($data){
        $this->children = array_filter($this->children, function($child){
            $child !== $data;
        });
    }
}

class Tree {
    public $root;

    function __construct(){
        $this->root = null;
    }

    function traverseBF($func){
        $nodes = [$this->root];

        while(!empty($nodes)){
            $node = array_shift($nodes);
            array_push($nodes, ...$node->children);

            $func($node);
        }
    }

    function traverseDF($func){
        $nodes = [$this->root];

        while(!empty($nodes)){
            $node = array_shift($nodes);
            array_unshift($nodes, ...$this->children);

            $func($node);
        }
    }
}

$t = new Tree();
$t->root = new Node(1);
$t->root->add(2);
$t->root->add(3);
// var_dump($t);

