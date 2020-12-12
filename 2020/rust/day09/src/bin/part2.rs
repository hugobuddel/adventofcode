//! --- Day 9: Encoding Error ---
// --- Part Two ---
//
// The final step in breaking the XMAS encryption relies on the invalid number
// you just found: you must find a contiguous set of at least two numbers in
// your list which sum to the invalid number from step 1.
//
// Again consider the above example:
//
// 35
// 20
// 15
// 25
// 47
// 40
// 62
// 55
// 65
// 95
// 102
// 117
// 150
// 182
// 127
// 219
// 299
// 277
// 309
// 576
//
// In this list, adding up all of the numbers from 15 through 40 produces the
// invalid number from step 1, 127. (Of course, the contiguous set of numbers
// in your actual list might be much longer.)
//
// To find the encryption weakness, add together the smallest and largest
// number in this contiguous range; in this example, these are 15 and 47,
// producing 62.
//
// What is the encryption weakness in your XMAS-encrypted list of numbers?

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashMap;

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Advent of Code 2020 Day 9!");

    let filename = "inputexample.txt";
    let number_bad = 127;

    // let filename = "input.txt";
    // let number_bad = 18272118;

    if let Ok(lines) = read_lines(filename) {
        let mut numbers: Vec<i32> = Vec::new();
        for line in lines {
            let num_new = line.unwrap().parse::<i32>().unwrap();
            numbers.push(num_new);
            let mut sum: i32 = numbers.iter().sum();
            // println!("Trying: {} {}", num_new, sum);
            if sum == number_bad {
                let largest = numbers.iter().max().unwrap();
                let smallest = numbers.iter().min().unwrap();
                println!(
                    "Found1: {}, {} {} {}",
                    number_bad,
                    largest,
                    smallest,
                    largest + smallest,
                );
                println!("{:?}", numbers);
            }
            while sum > number_bad {
                numbers.remove(0);
                sum = numbers.iter().sum();
                if sum == number_bad {
                    let largest = numbers.iter().max().unwrap();
                    let smallest = numbers.iter().min().unwrap();
                    println!(
                        "Found1: {}, {} {} {}",
                        number_bad,
                        largest,
                        smallest,
                        largest + smallest,
                    );
                    println!("{:?}", numbers);
                }
            }
        }
    }
}
