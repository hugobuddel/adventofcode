//! Advent of Code 2020 Day 3
// --- Day 3: Toboggan Trajectory ---
// With the toboggan login problems resolved, you set off toward the airport.
// While travel by toboggan might be easy, it's certainly not safe: there's
// very minimal steering and the area is covered in trees. You'll need to see
// which angles will take you near the fewest trees.
//
// Due to the local geology, trees in this area only grow on exact integer
// coordinates in a grid. You make a map (your puzzle input) of the open
// squares (.) and trees (#) you can see. For example:
//
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#
//
// These aren't the only trees, though; due to something you read about once
// involving arboreal genetics and biome stability, the same pattern repeats
// to the right many times:
//
// ..##.........##.........##.........##.........##.........##.......  --->
// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........#.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...##....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//
// You start on the open square (.) in the top-left corner and need to reach
// the bottom (below the bottom-most row on your map).
//
// The toboggan can only follow a few specific slopes (you opted for a cheaper
// model that prefers rational numbers); start by counting all the trees you
// would encounter for the slope right 3, down 1:
//
// From your starting position at the top-left, check the position that is
// right 3 and down 1. Then, check the position that is right 3 and down 1
// from there, and so on until you go past the bottom of the map.
//
// The locations you'd check in the above example are marked here with O where
// there was an open square and X where there was a tree:
//
// ..##.........##.........##.........##.........##.........##.......  --->
// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........X.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...#X....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//
// In this example, traversing the map using this slope would cause you to
// encounter 7 trees.
//
// Starting at the top-left corner of your map and following a slope of right 3
// and down 1, how many trees would you encounter?

// --- Part Two ---
// Time to check the rest of the slopes - you need to minimize the probability
// of a sudden arboreal stop, after all.
//
// Determine the number of trees you would encounter if, for each of the
// following slopes, you start at the top-left corner and traverse the map all
// the way to the bottom:
//
// Right 1, down 1.
// Right 3, down 1. (This is the slope you already checked.)
// Right 5, down 1.
// Right 7, down 1.
// Right 1, down 2.
// In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s)
// respectively; multiplied together, these produce the answer 336.
//
// What do you get if you multiply together the number of trees encountered on
// each of the listed slopes?

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn slide(
    map: &Vec<String>,
    slopex: usize,
    slopey: usize,
) -> usize {
    let mut x: usize = 0;
    let mut collisions: usize = 0;
    let mut ycounter: usize = 0;
    for row in map {
        let mut chars: Vec<char> = row.chars().collect();
        // let mut result = String::with_capacity(row.len());
        // Only count rows that fit with slopey.
        if ycounter == 0 {
            if chars[x] == '#' {
                collisions += 1;
                chars[x] = 'X';
            } else {
                chars[x] = 'O';
            }
            x += slopex;
            x = x % row.len();
        }
        let row2: String = chars.iter().collect();
        println!("{:?}", row2);
        ycounter += 1;
        ycounter = ycounter % slopey;
    }
    collisions
}

fn main() {
    println!("Advent of Code 2020 Day 3!");
    let map :Vec<String> = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ].iter().map(|x| x.to_string()).collect();
    let slopex: usize = 3;
    let slopey: usize = 1;

    let collisions = slide(&map, slopex, slopey);
    println!("Colissions: {}", collisions);
    assert_eq!(collisions, 7, "Collisions should be 7.");

    let slopex: usize = 1;
    let slopey: usize = 2;
    let collisions = slide(&map, slopex, slopey);
    println!("Colissions: {}", collisions);
    assert_eq!(collisions, 2, "Collisions should be 2.");

    let map2 = map;
    let slopex: usize = 1;
    let slopey: usize = 1;
    let collisions1 = slide(&map2, slopex, slopey);
    println!("Colissions: {} {} {}", slopex, slopey, collisions1);

    let slopex: usize = 3;
    let slopey: usize = 1;
    let collisions2 = slide(&map2, slopex, slopey);
    println!("Colissions: {} {} {}", slopex, slopey, collisions2);

    let slopex: usize = 5;
    let slopey: usize = 1;
    let collisions3 = slide(&map2, slopex, slopey);
    println!("Colissions: {} {} {}", slopex, slopey, collisions3);

    let slopex: usize = 7;
    let slopey: usize = 1;
    let collisions4 = slide(&map2, slopex, slopey);
    println!("Colissions: {} {} {}", slopex, slopey, collisions4);

    let slopex: usize = 1;
    let slopey: usize = 2;
    let collisions5 = slide(&map2, slopex, slopey);
    println!("Colissions: {} {} {}", slopex, slopey, collisions5);

    let collisions_prod = collisions1 * collisions2 * collisions3 * collisions4 * collisions5;
    println!("Colissions Product: {}", collisions_prod);
    assert_eq!(collisions_prod, 336, "Collisions Product should be 336.");


    if let Ok(map) = read_lines("./input.txt") {
        // let map2 = map.map(|x| x.unwrap().as_str()).collect::<Vec<_>>();
        // let map2 = map.map(|x| &x.unwrap().as_str()).collect::<Vec<_>>();
        let map2 = map.map(
            |x| x.unwrap().chars().collect::<String>()
        ).collect::<Vec<String>>();
        let slopex: usize = 3;
        let slopey: usize = 1;
        let collisions = slide(&map2, slopex, slopey);
        println!("Colissions: {}", collisions);

        let slopex: usize = 1;
        let slopey: usize = 1;
        let collisions1 = slide(&map2, slopex, slopey);
        println!("Colissions: {} {} {}", slopex, slopey, collisions1);

        let slopex: usize = 3;
        let slopey: usize = 1;
        let collisions2 = slide(&map2, slopex, slopey);
        println!("Colissions: {} {} {}", slopex, slopey, collisions2);

        let slopex: usize = 5;
        let slopey: usize = 1;
        let collisions3 = slide(&map2, slopex, slopey);
        println!("Colissions: {} {} {}", slopex, slopey, collisions3);

        let slopex: usize = 7;
        let slopey: usize = 1;
        let collisions4 = slide(&map2, slopex, slopey);
        println!("Colissions: {} {} {}", slopex, slopey, collisions4);

        let slopex: usize = 1;
        let slopey: usize = 2;
        let collisions5 = slide(&map2, slopex, slopey);
        println!("Colissions: {} {} {}", slopex, slopey, collisions5);

        let collisions_prod = collisions1 * collisions2 * collisions3 * collisions4 * collisions5;
        println!("Colissions Product: {}", collisions_prod);
    }
}

