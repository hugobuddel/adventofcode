//! --- Day 5: Binary Boarding ---
// You board your plane only to discover a new problem: you dropped your
// boarding pass! You aren't sure which seat is yours, and all of the flight
// attendants are busy with the flood of people that suddenly made it through
// passport control.
//
// You write a quick program to use your phone's camera to scan all of the
// nearby boarding passes (your puzzle input); perhaps you can find your seat
// through process of elimination.
//
// Instead of zones or groups, this airline uses binary space partitioning to
// seat people. A seat might be specified like FBFBBFFRLR, where F means
// "front", B means "back", L means "left", and R means "right".
//
// The first 7 characters will either be F or B; these specify exactly one of
// the 128 rows on the plane (numbered 0 through 127). Each letter tells you
// which half of a region the given seat is in. Start with the whole list of
// rows; the first letter indicates whether the seat is in the front (0 through
// 63) or the back (64 through 127). The next letter indicates which half of
// that region the seat is in, and so on until you're left with exactly one
// row.
//
// For example, consider just the first seven characters of FBFBBFFRLR:
//
// Start by considering the whole range, rows 0 through 127.
// F means to take the lower half, keeping rows 0 through 63.
// B means to take the upper half, keeping rows 32 through 63.
// F means to take the lower half, keeping rows 32 through 47.
// B means to take the upper half, keeping rows 40 through 47.
// B keeps rows 44 through 47.
// F keeps rows 44 through 45.
// The final F keeps the lower of the two, row 44.
// The last three characters will be either L or R; these specify exactly one
// of the 8 columns of seats on the plane (numbered 0 through 7). The same
// process as above proceeds again, this time with only three steps. L means to
// keep the lower half, while R means to keep the upper half.
//
// For example, consider just the last 3 characters of FBFBBFFRLR:
//
// Start by considering the whole range, columns 0 through 7.
// R means to take the upper half, keeping columns 4 through 7.
// L means to take the lower half, keeping columns 4 through 5.
// The final R keeps the upper of the two, column 5.
// So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
//
// Every seat also has a unique seat ID: multiply the row by 8, then add the
// column. In this example, the seat has ID 44 * 8 + 5 = 357.
//
// Here are some other boarding passes:
//
// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820.
// As a sanity check, look through your list of boarding passes. What is the
// highest seat ID on a boarding pass?

// --- Part Two ---
// Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
//
// It's a completely full flight, so your seat should be the only missing
// boarding pass in your list. However, there's a catch: some of the seats at
// the very front and back of the plane don't exist on this aircraft, so
// they'll be missing from your list as well.
//
// Your seat wasn't at the very front or back, though; the seats with IDs +1
// and -1 from yours will be in your list.
//
// What is the ID of your seat?

use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn seat_id_from_partitioning(partitioning: &str) -> usize {
    let mut seat_id: usize = 0;
    let mut row: usize = 0;
    let mut column: usize = 0;
    for (i, letter) in partitioning.chars().enumerate() {
        if letter == 'B' {
            row += 2_usize.pow((6 - i).try_into().unwrap());
        }
        if letter == 'R' {
            column += 2_usize.pow((9 - i).try_into().unwrap());
        }
        seat_id = row * 8 + column;
        println!("i:{} letter:{} row:{} column:{} seat_id:{}", i, letter, row, column, seat_id);
    }
    seat_id
}

fn main() {
    println!("Advent of Code 2020 Day 5!");
    assert_eq!(seat_id_from_partitioning("FBFBBFFRLR"), 357);
    assert_eq!(seat_id_from_partitioning("BFFFBBFRRR"), 567);
    assert_eq!(seat_id_from_partitioning("FFFBBBFRRR"), 119);
    assert_eq!(seat_id_from_partitioning("BBFFBBFRLL"), 820);

    let mut seats = HashSet::new();
    for s in 1..890 {
        seats.insert(s);
    }

    if let Ok(passes) = read_lines("./input.txt") {
        let mut seat_max: usize = 0;
        for rpartitioning in passes {
            if let Ok(partitioning) = rpartitioning {
                let seat_id = seat_id_from_partitioning(&partitioning);
                if seat_id > seat_max {
                    seat_max = seat_id;
                }
                println!("Pass:{:?} seat:{} max:{}", partitioning, seat_id, seat_max);
                seats.remove(&seat_id);
            }
        }
    }

    println!("Seats:{:?}", seats);
    let mut seats_sorted = seats.into_iter().collect::<Vec<_>>();
    seats_sorted.sort();
    println!("Sorted:{:?}", seats_sorted);
}
