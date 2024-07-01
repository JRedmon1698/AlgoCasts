<?php

require_once('index.php');

use PHPUnit\Framework\TestCase;

class Test extends TestCase {
    public function testExists(){
        $this->assertTrue(function_exists('chunks'));
    }

    public function testOutput(){
        $this->assertEquals(chunks([1, 2, 3], 2), [[1, 2], [3]]);
    }
}
