<?php

require_once('index.php');

use PHPUnit\Framework\TestCase;

class Test extends TestCase {
    public function testExists(){
        $this->assertTrue(function_exists('palindrome'));
    }

    public function testPalindrome(){
        $this->assertFalse(palindrome('abc'));
        $this->assertFalse(palindrome(' aba'));
        $this->assertTrue(palindrome('aba'));
        $this->assertTrue(palindrome('1001'));
        $this->assertTrue(palindrome('fish hsif'));
        $this->assertFalse(palindrome('Aba'));
    }
}
