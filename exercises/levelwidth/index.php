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

function levelWidth($root){
    $counts = [0];
    $stop = 'ballast';
    $nodes = [$root, $stop];

    while(count($nodes) > 1){
        $node = array_shift($nodes);

        if($node === $stop){
            array_push($counts, 0);
            array_push($nodes, $stop);
        }
        else {
            array_push($nodes, ...$node->children);
            $counts[count($counts) - 1]++;
        }
    }
    return $counts;
}


$node = new Node(1);
$node->add(3);
$node->add(5);
var_dump($node);

$res = levelWidth($node);
var_dump($res);
