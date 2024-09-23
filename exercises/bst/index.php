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

    function contains($data){
        if($data === $this->data){
            return $this;
        }

        if(($data < $this->data) && isset($this->left)){
            return $this->left->contains($data);
        }
        else if(($data > $this->data) && isset($this->right)){
            return $this->right->contains($data);
        }

        return null;
    }
}

$n = new Node(5);
$n->insert(10);
$n->insert(2);
$n->insert(-1);
$n->insert(27);

// var_dump($n);
var_dump($n->contains(2));
var_dump($n->contains(27));
var_dump($n->contains(1));
