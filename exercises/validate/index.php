<?php

class Node {
    public $data;
    public $left;
    public $right;

    function __construct($data){
        $this->data = $data;
        $this->left = null;
        $this->right = null;
    }

    function insert($data){
        if(($data < $this->data) && isset($this->left)){
            $this->left->insert($data);
        }
        else if($data < $this->data){
            $this->left = new Node($data);
        }
        else if(($data > $this->data) && isset($this->right)){
            $this->right->insert($data);
        }
        else if($data > $this->data){
            $this->right = new Node($data);
        }
    }
}

function validate($node, $min = null, $max = null){
    if(isset($max) && $node->data > $max){
        return false;
    }
    if(isset($min) && $node->data < $min){
        return false;
    }

    if(isset($node->left) && !validate($node->left, $min, $node->data)){
        return false;
    }
    if(isset($node->right) && !validate($node->right, $node->data, $max)){
        return false;
    }

    return true;
}


$n = new Node(5);
$n->insert(10);
$n->insert(1);
$n->insert(-2);
$n->insert(90);
$n->insert(17);

var_dump(validate($n));
