//! --- Day 12: Rain Risk ---
//
// Your ferry made decent progress toward the island, but the storm came in
// faster than anyone expected. The ferry needs to take evasive actions!
//
// Unfortunately, the ship's navigation computer seems to be malfunctioning;
// rather than giving a route directly to safety, it produced extremely
// circuitous instructions. When the captain uses the PA system to ask if
// anyone can help, you quickly volunteer.
//
// The navigation instructions (your puzzle input) consists of a sequence of
// single-character actions paired with integer input values. After staring at
// them for a few minutes, you work out what they probably mean:
//
//     Action N means to move north by the given value.
//     Action S means to move south by the given value.
//     Action E means to move east by the given value.
//     Action W means to move west by the given value.
//     Action L means to turn left the given number of degrees.
//     Action R means to turn right the given number of degrees.
//     Action F means to move forward by the given value in the direction the ship is currently facing.
//
// The ship starts by facing east. Only the L and R actions change the
// direction the ship is facing. (That is, if the ship is facing east and the
// next instruction is N10, the ship would move north 10 units, but would still
// move east if the following action were F.)
//
// For example:
//
// F10
// N3
// F7
// R90
// F11
//
// These instructions would be handled as follows:
//
//     F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
//     N3 would move the ship 3 units north to east 10, north 3.
//     F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
//     R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
//     F11 would move the ship 11 units south to east 17, south 8.
//
// At the end of these instructions, the ship's Manhattan distance (sum of the
// absolute values of its east/west position and its north/south position) from
// its starting position is 17 + 8 = 25.
//
// Figure out where the navigation instructions lead. What is the Manhattan
// distance between that location and the ship's starting position?

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

use std::fs;
use std::num;

#[derive(Parser)]
#[grammar = "directions.pest"]
pub struct DirectionsParser;

#[derive(Debug)]
enum Compass {
    East,
    South,
    West,
    North,
}

fn main() {
    println!("Advent of Code 2020 Day 12!");

    // let filename = "inputexample.txt";
    let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let directions = DirectionsParser::parse(Rule::directions, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let mut pos_ew: i32 = 0;  // East to West
    let mut pos_sn: i32 = 0;  // South to North
    let mut direction = Compass::East;

    for amove in directions.into_inner() {
        // println!("{:?}", amove);
        match amove.as_rule() {
            Rule::moveforward => {
                let distance = amove.into_inner().as_str().parse::<i32>().unwrap();
                println!("Moving forward by {}!", distance);
                match direction {
                    Compass::East => {pos_ew -= distance}
                    Compass::South => {pos_sn -= distance}
                    Compass::West => {pos_ew += distance}
                    Compass::North => {pos_sn += distance}
                }
                println!("Currently at {},{}. Distance {}. Facing {:?}.", pos_ew, pos_sn, pos_ew.abs() + pos_sn.abs(), direction);
            }
            Rule::moveangle => {
                let mut pairs = amove.into_inner();
                let leftright = pairs.next().unwrap().as_str();
                let angle = pairs.next().unwrap().as_str().parse::<i32>().unwrap() / 90;
                println!("Turning 90 degrees to the {} {} times!", leftright, angle);
                for _ in 0..angle {
                    match (direction, leftright) {
                        (Compass::East, "L") => {direction = Compass::North}
                        (Compass::East, "R") => {direction = Compass::South}
                        (Compass::South, "L") => {direction = Compass::West}
                        (Compass::South, "R") => {direction = Compass::East}
                        (Compass::West, "L") => {direction = Compass::South}
                        (Compass::West, "R") => {direction = Compass::North}
                        (Compass::North, "L") => {direction = Compass::West}
                        (Compass::North, "R") => {direction = Compass::East}
                        _ => unreachable!()
                    }
                }
                println!("Currently at {},{}. Distance {}. Facing {:?}.", pos_ew, pos_sn, pos_ew.abs() + pos_sn.abs(), direction);
            }
            Rule::movecompass => {
                let mut pairs = amove.into_inner();
                let compass = pairs.next().unwrap().as_str();
                let distance = pairs.next().unwrap().as_str().parse::<i32>().unwrap();
                println!("Moving {} in direction {}!", distance, compass);
                match compass {
                    "E" => {pos_ew -= distance}
                    "S" => {pos_sn -= distance}
                    "W" => {pos_ew += distance}
                    "N" => {pos_sn += distance}
                    _ => unreachable!()
                }
                println!("Currently at {},{}. Distance {}. Facing {:?}.", pos_ew, pos_sn, pos_ew.abs() + pos_sn.abs(), direction);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

}
