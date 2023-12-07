<?php

// $input = file('./test2_input.txt');
$input = file('./puzzle.txt');

$total = 0;
foreach($input as $line) {
    preg_match('/^\D*(\d)/', $line, $firstDigit);
    preg_match('/(\d)\D*$/', $line, $lastDigit);
    $number = $firstDigit[1] * 10 + $lastDigit[1];
    $total += $number;
}
print "total 1 - " . $total . "\n";

// $input = file('./test2_input.txt');

$total = 0;
foreach($input as $line) {
    $regex_string_nums = 'one|two|three|four|five|six|seven|eight|nine';
    preg_match("/{$regex_string_nums}|\d/", $line, $stringFirst);
    preg_match("/" . strrev($regex_string_nums) . '|\d/', strrev($line), $stringLast);
    $firstDigit = parsestringNumber($stringFirst[0]);
    $lastDigit = parsestringNumber(strrev($stringLast[0]));
    $number = $firstDigit * 10 + $lastDigit;
    $total += $number;
}
print "total 2 - " . $total . "\n";


function parsestringNumber($stringNumber)
{
    switch ($stringNumber) {
    // case 'zero':
    //     return 0;
    case 'one':
        return 1;
    case 'two':
        return 2;
    case 'three':
        return 3;
    case 'four':
        return 4;
    case 'five':
        return 5;
    case 'six':
        return 6;
    case 'seven':
        return 7;
    case 'eight':
        return 8;
    case 'nine':
        return 9;
        break;

    default:
        return (int) $stringNumber;
    }
}

