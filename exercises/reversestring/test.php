<?php

require_once('index.php'); // Include the file containing the reverse function

use PHPUnit\Framework\TestCase;

class ReverseStringTest extends TestCase {
    public function testExists(){
        $this->assertTrue(function_exists('reverse'));
    }
    
    public function testReverse(){
        $this->assertEquals('olleh', reverse('hello'));
        $this->assertEquals('dcba  ', reverse('  abcd'));
    }
}

