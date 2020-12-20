//! --- Day 20: Jurassic Jigsaw ---
//
// The high-speed train leaves the forest and quickly carries you south. You
// can even see a desert in the distance! Since you have some spare time, you
// might as well see if there was anything interesting in the image the
// Mythical Information Bureau satellite captured.
//
// After decoding the satellite messages, you discover that the data actually
// contains many small images created by the satellite's camera array. The
// camera array consists of many cameras; rather than produce a single square
// image, they produce many smaller square image tiles that need to be
// reassembled back into a single image.
//
// Each camera in the camera array returns a single monochrome image tile with
// a random unique ID number. The tiles (your puzzle input) arrived in a random
// order.
//
// Worse yet, the camera array appears to be malfunctioning: each image tile
// has been rotated and flipped to a random orientation. Your first task is to
// reassemble the original image by orienting the tiles so they fit together.
//
// To show how the tiles should be reassembled, each tile's image data includes
// a border that should line up exactly with its adjacent tiles. All tiles have
// this border, and the border lines up exactly when the tiles are both
// oriented correctly. Tiles at the edge of the image also have this border,
// but the outermost edges won't line up with any other tiles.
//
// For example, suppose you have the following nine tiles:
//
// Tile 2311:
// ..##.#..#.
// ##..#.....
// #...##..#.
// ####.#...#
// ##.##.###.
// ##...#.###
// .#.#.#..##
// ..#....#..
// ###...#.#.
// ..###..###
//
// Tile 1951:
// #.##...##.
// #.####...#
// .....#..##
// #...######
// .##.#....#
// .###.#####
// ###.##.##.
// .###....#.
// ..#.#..#.#
// #...##.#..
//
// Tile 1171:
// ####...##.
// #..##.#..#
// ##.#..#.#.
// .###.####.
// ..###.####
// .##....##.
// .#...####.
// #.##.####.
// ####..#...
// .....##...
//
// Tile 1427:
// ###.##.#..
// .#..#.##..
// .#.##.#..#
// #.#.#.##.#
// ....#...##
// ...##..##.
// ...#.#####
// .#.####.#.
// ..#..###.#
// ..##.#..#.
//
// Tile 1489:
// ##.#.#....
// ..##...#..
// .##..##...
// ..#...#...
// #####...#.
// #..#.#.#.#
// ...#.#.#..
// ##.#...##.
// ..##.##.##
// ###.##.#..
//
// Tile 2473:
// #....####.
// #..#.##...
// #.##..#...
// ######.#.#
// .#...#.#.#
// .#########
// .###.#..#.
// ########.#
// ##...##.#.
// ..###.#.#.
//
// Tile 2971:
// ..#.#....#
// #...###...
// #.#.###...
// ##.##..#..
// .#####..##
// .#..####.#
// #..#.#..#.
// ..####.###
// ..#.#.###.
// ...#.#.#.#
//
// Tile 2729:
// ...#.#.#.#
// ####.#....
// ..#.#.....
// ....#..#.#
// .##..##.#.
// .#.####...
// ####.#.#..
// ##.####...
// ##..#.##..
// #.##...##.
//
// Tile 3079:
// #.#.#####.
// .#..######
// ..#.......
// ######....
// ####.#..#.
// .#...#.##.
// #.#####.##
// ..#.###...
// ..#.......
// ..#.###...
//
// By rotating, flipping, and rearranging them, you can find a square
// arrangement that causes all adjacent borders to line up:
//
// #...##.#.. ..###..### #.#.#####.
// ..#.#..#.# ###...#.#. .#..######
// .###....#. ..#....#.. ..#.......
// ###.##.##. .#.#.#..## ######....
// .###.##### ##...#.### ####.#..#.
// .##.#....# ##.##.###. .#...#.##.
// #...###### ####.#...# #.#####.##
// .....#..## #...##..#. ..#.###...
// #.####...# ##..#..... ..#.......
// #.##...##. ..##.#..#. ..#.###...
//
// #.##...##. ..##.#..#. ..#.###...
// ##..#.##.. ..#..###.# ##.##....#
// ##.####... .#.####.#. ..#.###..#
// ####.#.#.. ...#.##### ###.#..###
// .#.####... ...##..##. .######.##
// .##..##.#. ....#...## #.#.#.#...
// ....#..#.# #.#.#.##.# #.###.###.
// ..#.#..... .#.##.#..# #.###.##..
// ####.#.... .#..#.##.. .######...
// ...#.#.#.# ###.##.#.. .##...####
//
// ...#.#.#.# ###.##.#.. .##...####
// ..#.#.###. ..##.##.## #..#.##..#
// ..####.### ##.#...##. .#.#..#.##
// #..#.#..#. ...#.#.#.. .####.###.
// .#..####.# #..#.#.#.# ####.###..
// .#####..## #####...#. .##....##.
// ##.##..#.. ..#...#... .####...#.
// #.#.###... .##..##... .####.##.#
// #...###... ..##...#.. ...#..####
// ..#.#....# ##.#.#.... ...##.....
//
// For reference, the IDs of the above tiles are:
//
// 1951    2311    3079
// 2729    1427    2473
// 2971    1489    1171
//
// To check that you've assembled the image correctly, multiply the IDs of the
// four corner tiles together. If you do this with the assembled tiles from the
// example above, you get 1951 * 3079 * 2971 * 1171 = 20899048083289.
//
// Assemble the tiles into an image. What do you get if you multiply together
// the IDs of the four corner tiles?

use std::fs;
use std::collections::HashSet;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "tiles.pest"]
pub struct TilesParser;

#[derive(Debug)]
struct Tile{
    name: usize,
    lines: Vec<String>,
    // sides: [usize; 8],
}

enum Side {
    North,
    East,
    South,
    West,
}

impl Tile {
    fn from_rule(rule: Pair<Rule>) -> Tile {
        // println!("New Tile: {}", rule.as_str());
        let mut pair = rule.into_inner();
        let name: usize = pair.next().unwrap().as_str().parse().unwrap();
        let lines: Vec<String> = pair.next().unwrap().into_inner().map(|x| x.as_str().to_string()).collect();
        Tile {
            name,
            lines,
            // sides: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    fn get_side(&self, side: Side, flipped: bool) -> String {
        let s1: String = match side {
            Side::West => {self.lines.iter().map(|l| l.chars().next().unwrap()).collect()}
            Side::North => {self.lines[0].clone()}
            Side::East => {self.lines.iter().map(|l| l.chars().last().unwrap()).collect()}
            Side::South => {self.lines.iter().last().unwrap().clone()}
        };
        let s2: String = match flipped {
            true => {s1.chars().rev().collect()}
            false => {s1}
        };
        s2
    }
}

fn main() {
    println!("Advent of Code 2020 Day 20!");

    let filename = "inputexample.txt";
    // let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let tilefile = TilesParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let rtiles = tilefile.into_inner().next().unwrap();
    // println!("Tiles: {}", rtiles.as_str());
    let mut tiles: Vec<Tile> = Vec::new();
    for rtile in rtiles.into_inner() {
        let tile = Tile::from_rule(rtile);
        println!("Tile {:?}", tile);
        tiles.push(tile);
    }

    let tile = &tiles.iter().last().unwrap();
    println!("N: {}", tile.get_side(Side::North, false));
    println!("N: {}", tile.get_side(Side::North, true));
    println!("S: {}", tile.get_side(Side::South, false));
    println!("S: {}", tile.get_side(Side::South, true));
    println!("E: {}", tile.get_side(Side::East, false));
    println!("W: {}", tile.get_side(Side::West, false));

    let mut allsides: HashSet<&String> = HashSet::new();

}
