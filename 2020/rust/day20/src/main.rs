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
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "tiles.pest"]
pub struct TilesParser;

#[derive(Debug)]
#[derive(Clone)]
struct Tile{
    name: usize,
    lines: Vec<String>,
    // sides: [usize; 8],
}

#[derive(Debug)]
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

    fn get_side(&self, side: &Side, flipped: bool) -> String {
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

    // Must this tile be a corner tile?
    fn must_be_corner_tile(&self, allsides: &HashMap<String, usize>) -> bool {
        let mut count = 0;
        for side in &[Side::North, Side::South, Side::East, Side::West] {
            if allsides[&self.get_side(side, false)] == 1 {
                count += 1
            }
        }
        let mut count_flipped = 0;
        for side in &[Side::North, Side::South, Side::East, Side::West] {
            if allsides[&self.get_side(side, false)] == 1 {
                count_flipped += 1
            }
        }
        // This must be a corner tile if it has uniq edges in both orientations
        (count >= 2) && (count_flipped >= 2)
    }

    fn flip_vertically(&mut self) {
        self.lines = self.lines.iter().rev().map(|x| x.clone()).collect();
    }

    fn flip_horizontally(&mut self) {
        self.lines = self.lines.iter().map(|x| x.chars().rev().collect()).collect();
    }

    // See whether any of the sides matches.
    fn has_matching_edge(&self, edge: String) -> bool {
        for side in &[Side::North, Side::South, Side::East, Side::West] {
            for flip in &[false, true] {
                if edge == self.get_side(side, *flip) {
                    return true
                }
            }
        }
        false
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
    println!("N: {}", tile.get_side(&Side::North, false));
    println!("N: {}", tile.get_side(&Side::North, true));
    println!("S: {}", tile.get_side(&Side::South, false));
    println!("S: {}", tile.get_side(&Side::South, true));
    println!("E: {}", tile.get_side(&Side::East, false));
    println!("W: {}", tile.get_side(&Side::West, false));

    let mut allsides: HashMap<String, usize> = HashMap::new();
    for tile in &tiles {
        for side in &[Side::North, Side::South, Side::East, Side::West] {
            for flip in &[false, true] {
                let edge = tile.get_side(side, *flip);
                if let Some(x) = allsides.get_mut(&edge) {
                    *x += 1;
                } else {
                    allsides.insert(edge, 1);
                }
            }
        }
    }
    println!("Total number of uniq edges: {}", &allsides.len());
    println!("Edges: {:?}", allsides);
    // Apparently each edge matches only one other edge.

    let tiles_corner: Vec<&Tile> = tiles.iter().filter(|t| t.must_be_corner_tile(&allsides)).collect::<_>();
    let mut prod: usize = 1;
    for tile in &tiles_corner {
        prod *= tile.name;
    }
    println!("Must be corner tiles: {} {}", tiles_corner.len(), prod);

    // Get a corner tile to start with.
    let mut tile_current: Tile = (*tiles_corner[0]).clone();
    // tile_current.flip_horizontally();
    // Flip horizontally if the North side has a match.
    let mnorth = allsides[&tile_current.get_side(&Side::North, false)];
    let msouth = allsides[&tile_current.get_side(&Side::South, false)];
    let meast = allsides[&tile_current.get_side(&Side::East, false)];
    let mwest = allsides[&tile_current.get_side(&Side::West, false)];
    println!("Side counts: {} {} {} {}", mnorth, msouth, meast, mwest);
    if allsides[&tile_current.get_side(&Side::North, false)] == 2 {
        println!("Flip vertically.");
        tile_current.flip_vertically();
    }
    if allsides[&tile_current.get_side(&Side::North, false)] == 2 {
        unreachable!("Flip vertically.");
    }
    if allsides[&tile_current.get_side(&Side::West, false)] == 2 {
        println!("Flip horizontally.");
        tile_current.flip_horizontally();
    }
    if allsides[&tile_current.get_side(&Side::West, false)] == 2 {
        unreachable!("Flip horizontally.");
    }

    let mut tiles_square: Vec<Vec<Tile>> = Vec::new();
    let mut at_bottom: bool = false;
    let mut row_previous: Vec<Tile> = Vec::new();
    let mut row_current: Vec<Tile> = Vec::new();
    let mut nr_row: usize = 0;
    let mut nr_col: usize = 1; // start at one because we have the corner already

    // Hardcode the top left tile.
    let mut row_current = vec![tile_current.clone()];
    println!("Found1 tile {} {}", tile_current.name, row_current.len());

    // Fill the next row.
    while ! at_bottom {
        // OK, there are now two ways to track where we are..
        let mut at_right_end = false;
        if nr_row > 0 {
            nr_col = 0;
        }
        while !at_right_end {
            let tile_up = if nr_row > 0 {row_previous[nr_col].clone()} else {tile_current.clone()};

            println!("Searching {} {} {} {}", nr_row, nr_col, 0, 0);

            // Find the matching tile.
            let tiles_next: Vec<&Tile> = tiles.iter()
                // Ensure tiles match.
                .filter(
                    |t| if nr_col > 0 {
                        t.has_matching_edge(tile_current.get_side(&Side::East, false))
                        // Ensure we did not get the same tile again.
                        && t.name != tile_current.name
                    } else {
                        true
                    } && if nr_row > 0 {
                        t.has_matching_edge(tile_current.get_side(&Side::North, false))
                        // Ensure we did not get the same tile again.
                        && t.name != tile_up.name
                    } else { true }
                )
                .collect::<_>();
            assert_eq!(1, tiles_next.len());
            tile_current = tiles_next[0].clone();
            // TODO: flip upside down if necessary
            at_right_end = allsides[&tile_current.get_side(&Side::East, false)] == 1;
            row_current.push(tile_current.clone());
            nr_col += 1;
            println!("Found tile {} {}", tile_current.name, row_current.len());
        }
        // Pus row onto the square.
        tiles_square.push(row_current.clone());
        // Set previous row.
        row_previous = row_current.clone();
        // Go back to start of the row.
        tile_current = row_current[0].clone();
        nr_row += 1;
        // at_bottom_end =
    }
    // let mut image: Vec<String> = Vec::new();


}
