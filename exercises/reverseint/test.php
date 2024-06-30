<?php

require_once('index.php');

use PHPUnit\Framework\TestCase;

class Test extends TestCase {
    public function testExists(){
        $this->assertTrue(function_exists('reverseInt'));
    }

    public function testOutput(){
        $this->assertEquals(reverseInt(123), 321);
        $this->assertEquals(reverseInt(111), 111);
        $this->assertEquals(reverseInt(-123), -321);
        $this->assertEquals(reverseInt(90), 9);
    }
}
