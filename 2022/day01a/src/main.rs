// Advent of Code 2022 Day 01a:
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
// input is a file of numbers, one per line, with a blank line to demarcate groupings (Elves).
//
// Sum the groupings to find the grouping (Elf) with the highest total. Print the largest sum.

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("puzzle.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => parse_iter_sum(s),
    }
}

fn parse_iter_sum(s :String) {
    let mut max_calories = 0;
    let elves: Vec<&str> = s.split("\n\n").collect();
    for elf in elves {
        let food: Vec<&str> = elf.split("\n").collect();
        let mut elf_total = 0;
        for item in food {
            elf_total += item.parse::<i32>().unwrap_or(0);
        }
        if elf_total > max_calories {
            max_calories = elf_total;
        }
    }
    println!("{}", max_calories);
}
