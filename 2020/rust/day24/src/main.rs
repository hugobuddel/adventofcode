//! --- Day 24: Lobby Layout ---
//
// Your raft makes it to the tropical island; it turns out that the small crab
// was an excellent navigator. You make your way to the resort.
//
// As you enter the lobby, you discover a small problem: the floor is being
// renovated. You can't even reach the check-in desk until they've finished
// installing the new tile floor.
//
// The tiles are all hexagonal; they need to be arranged in a hex grid with a
// very specific color pattern. Not in the mood to wait, you offer to help
// figure out the pattern.
//
// The tiles are all white on one side and black on the other. They start with
// the white side facing up. The lobby is large enough to fit whatever pattern
// might need to appear there.
//
// A member of the renovation crew gives you a list of the tiles that need to
// be flipped over (your puzzle input). Each line in the list identifies a
// single tile that needs to be flipped by giving a series of steps starting
// from a reference tile in the very center of the room. (Every line starts
// from the same reference tile.)
//
// Because the tiles are hexagonal, every tile has six neighbors: east,
// southeast, southwest, west, northwest, and northeast. These directions are
// given in your list, respectively, as e, se, sw, w, nw, and ne. A tile is
// identified by a series of these directions with no delimiters; for example,
// esenee identifies the tile you land on if you start at the reference tile
// and then move one tile east, one tile southeast, one tile northeast, and one
// tile east.
//
// Each time a tile is identified, it flips from white to black or from black
// to white. Tiles might be flipped more than once. For example, a line like
// esew flips a tile immediately adjacent to the reference tile, and a line
// like nwwswee flips the reference tile itself.
//
// Here is a larger example:
//
// sesenwnenenewseeswwswswwnenewsewsw
// neeenesenwnwwswnenewnwwsewnenwseswesw
// seswneswswsenwwnwse
// nwnwneseeswswnenewneswwnewseswneseene
// swweswneswnenwsewnwneneseenw
// eesenwseswswnenwswnwnwsewwnwsene
// sewnenenenesenwsewnenwwwse
// wenwwweseeeweswwwnwwe
// wsweesenenewnwwnwsenewsenwwsesesenwne
// neeswseenwwswnwswswnw
// nenwswwsewswnenenewsenwsenwnesesenew
// enewnwewneswsewnwswenweswnenwsenwsw
// sweneswneswneneenwnewenewwneswswnese
// swwesenesewenwneswnwwneseswwne
// enesenwswwswneneswsenwnewswseenwsese
// wnwnesenesenenwwnenwsewesewsesesew
// nenewswnwewswnenesenwnesewesw
// eneswnwswnwsenenwnwnwwseeswneewsenese
// neswnwewnwnwseenwseesewsenwsweewe
// wseweeenwnesenwwwswnew
//
// In the above example, 10 tiles are flipped once (to black), and 5 more are
// flipped twice (to black, then back to white). After all of these
// instructions have been followed, a total of 10 tiles are black.
//
// Go through the renovation crew's list and determine which tiles they need to
// flip. After all of the instructions have been followed, how many tiles are
// left with the black side up?

use std::fs;
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
// use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "renovations.pest"]
pub struct RenovationsParser;

fn main() {
    println!("Advent of Code 2020 Day 24!");

    // let filename = "inputexample.txt";
    let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");


    // let unparsed_file = "nwwswee\n".to_string();
    // nw w sw e e
    //   2211
    //  334400
    //

    let tilefile = RenovationsParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    // true is flipped
    let mut flipped: HashMap<(i32, i32), bool> = HashMap::new();

    let mut renovations = tilefile.into_inner();
    for renovation in renovations.next().unwrap().into_inner() {
        let (mut x, mut y) = (0, 0);
        println!("Renovation: {}", renovation.as_str());
        for step in renovation.into_inner() {
            match step.as_str() {
                "e" => {y += 1;}
                "ne" => {y += 1; x -= 1;}
                "nw" => {x -= 1;}
                "w" => {y -= 1;}
                "se" => {x += 1;}
                "sw" => {x += 1; y -= 1;}
                _ => unreachable!()
            }
            println!("Step {}, {}, {}", step.as_str(), x, y);
            // *flipped.entry((x, y)).or_insert(false) = ! flipped.get(&(x, y)).unwrap_or(&false);
        }
        *flipped.entry((x, y)).or_insert(false) = ! flipped.get(&(x, y)).unwrap_or(&false);
        println!("Flipped: {:?}", flipped);
    }

    let total: usize = flipped.iter().map(|x| (if *x.1 {1} else {0})).sum();
    println!("Black: {}", total);

    let x_min: i32 = flipped.iter().map(|x| x.0.0).min().unwrap();
    let x_max: i32 = flipped.iter().map(|x| x.0.0).max().unwrap();
    let y_min: i32 = flipped.iter().map(|x| x.0.1).min().unwrap();
    let y_max: i32 = flipped.iter().map(|x| x.0.1).max().unwrap();
    println!("x_min, x_max: {}, {}; y_min, y_max: {}, {}", x_min, x_max, y_min, y_max);
    for x in x_min..=x_max {
        let mut line = match x.abs() % 2 {
            0 => " ".to_string(),
            1 => "".to_string(),
            _ => unreachable!(),
        };
        let mut line2 = line.clone();
        for y in y_min..=y_max {
            let t = match flipped.get(&(x, y)) {
                Some(true) => " BB",
                Some(false) => " WW",
                None => " ..",
            };
            line += t;
            line2 += format!("{:2}{:1}", x, y.abs()).as_str();
        }
        println!("={:3} {}", x, line);
        println!("={:3} {}", x, line2);
        println!("=");
    }
}
