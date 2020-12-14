//! --- Day 12: Rain Risk ---
// --- Part Two ---
//
// Before you can give the destination to the captain, you realize that the
// actual action meanings were printed on the back of the instructions the
// whole time.
//
// Almost all of the actions indicate how to move a waypoint which is relative
// to the ship's position:
//
//     Action N means to move the waypoint north by the given value.
//     Action S means to move the waypoint south by the given value.
//     Action E means to move the waypoint east by the given value.
//     Action W means to move the waypoint west by the given value.
//     Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
//     Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
//     Action F means to move forward to the waypoint a number of times equal to the given value.
//
// The waypoint starts 10 units east and 1 unit north relative to the ship.
// The waypoint is relative to the ship; that is, if the ship moves, the
// waypoint moves with it.
//
// For example, using the same instructions as above:
//
//     F10 moves the ship to the waypoint 10 times (a total of 100 units east
//     and 10 units north), leaving the ship at east 100, north 10. The
//     waypoint stays 10 units east and 1 unit north of the ship.
//
//     N3 moves the waypoint 3 units north to 10 units east and 4 units north
//     of the ship. The ship remains at east 100, north 10.
//
//     F7 moves the ship to the waypoint 7 times (a total of 70 units east and
//     28 units north), leaving the ship at east 170, north 38. The waypoint
//     stays 10 units east and 4 units north of the ship.
//
//     R90 rotates the waypoint around the ship clockwise 90 degrees, moving it
//     to 4 units east and 10 units south of the ship. The ship remains at east
//     170, north 38.
//
//     F11 moves the ship to the waypoint 11 times (a total of 44 units east
//     and 110 units south), leaving the ship at east 214, south 72. The
//     waypoint stays 4 units east and 10 units south of the ship.
//
// After these operations, the ship's Manhattan distance from its starting
// position is 214 + 72 = 286.
//
// Figure out where the navigation instructions actually lead. What is the
// Manhattan distance between that location and the ship's starting position?

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

use std::fs;
use num;
use num::complex::{Complex};

#[derive(Parser)]
#[grammar = "directions.pest"]
pub struct DirectionsParser;

fn printship(posc: Complex<i32>, direction: Complex<i32>) {
    let north = Complex::new(0, 1);
    let east = Complex::new(-1, 0);
    let south = Complex::new(0, -1);
    let west = Complex::new(1, 0);
    // TODO: How to match this?
    let mut d = "Nothing";
    if direction == north {
        d = "North";
    } else if direction == east {
        d = "East";
    } else if direction == south {
        d = "South";
    } else if direction == west {
        d = "West";
    }
    println!("Currently at {},{}. Distance {}. Facing {}.", posc.re, posc.im, posc.re.abs() + posc.im.abs(), d);
    // println!("Currently at {},{}. Distance {}. Facing {:?}.", posc.re, posc.im, posc.re.abs() + posc.im.abs(), direction);
    // println!("Currently at {},{}. Distance {}. Facing {:?}. {}", posc.re, posc.im, posc.re.abs() + posc.im.abs(), d, direction);
}

fn main() {
    println!("Advent of Code 2020 Day 12!");

    // let filename = "inputexample.txt";
    let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let directions = DirectionsParser::parse(Rule::directions, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let north = Complex::new(0, 1);
    let east = Complex::new(-1, 0);
    let south = Complex::new(0, -1);
    let west = Complex::new(1, 0);
    let left = Complex::new(0, -1);
    let right = Complex::new(0, 1);
    let origin = Complex::new(0, 0);

    let mut posc = origin;
    let mut direction = east;
    printship(posc, direction);

    for amove in directions.into_inner() {
        println!("Move: {:?}", amove.as_str());
        match amove.as_rule() {
            Rule::moveforward => {
                let distance = amove.into_inner().as_str().parse::<i32>().unwrap();
                println!("Moving forward by {}!", distance);
                posc += distance * direction;
                printship(posc, direction);
            }
            Rule::moveangle => {
                let mut pairs = amove.into_inner();
                let leftright = pairs.next().unwrap().as_str();
                let angle = pairs.next().unwrap().as_str().parse::<i32>().unwrap() / 90;
                println!("Turning 90 degrees to the {} {} times!", leftright, angle);
                for _ in 0..angle {
                    match leftright {
                        "L" => {direction *= left}
                        "R" => {direction *= right}
                        _ => unreachable!()
                    }
                }
                printship(posc, direction);
            }
            Rule::movecompass => {
                let mut pairs = amove.into_inner();
                let compass = pairs.next().unwrap().as_str();
                let distance = pairs.next().unwrap().as_str().parse::<i32>().unwrap();
                println!("Moving {} in direction {}!", distance, compass);
                match compass {
                    "E" => {posc += distance * east}
                    "S" => {posc += distance * south}
                    "W" => {posc += distance * west}
                    "N" => {posc += distance * north}
                    _ => unreachable!()
                }
                printship(posc, direction);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

}
