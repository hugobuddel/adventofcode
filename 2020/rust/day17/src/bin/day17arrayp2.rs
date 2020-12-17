//! --- Day 17: Conway Cubes ---
//
// As your flight slowly drifts through the sky, the Elves at the Mythical
// Information Bureau at the North Pole contact you. They'd like some help
// debugging a malfunctioning experimental energy source aboard one of their
// super-secret imaging satellites.
//
// The experimental energy source is based on cutting-edge technology: a set of
// Conway Cubes contained in a pocket dimension! When you hear it's having
// problems, you can't help but agree to take a look.
//
// The pocket dimension contains an infinite 3-dimensional grid. At every
// integer 3-dimensional coordinate (x,y,z), there exists a single cube which
// is either active or inactive.
//
// In the initial state of the pocket dimension, almost all cubes start
// inactive. The only exception to this is a small flat region of cubes (your
// puzzle input); the cubes in this region start in the specified active (#) or
// inactive (.) state.
//
// The energy source then proceeds to boot up by executing six cycles.
//
// Each cube only ever considers its neighbors: any of the 26 other cubes where
// any of their coordinates differ by at most 1. For example, given the cube at
// x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at
// x=0,y=2,z=3, and so on.
//
// During a cycle, all cubes simultaneously change their state according to the
// following rules:
//
//     If a cube is active and exactly 2 or 3 of its neighbors are also active,
//     the cube remains active. Otherwise, the cube becomes inactive.
//
//     If a cube is inactive but exactly 3 of its neighbors are active, the
//     cube becomes active. Otherwise, the cube remains inactive.
//
// The engineers responsible for this experimental energy source would like you
// to simulate the pocket dimension and determine what the configuration of
// cubes should be at the end of the six-cycle boot process.
//
// For example, consider the following initial state:
//
// .#.
// ..#
// ###
//
// Even though the pocket dimension is 3-dimensional, this initial state
// represents a small 2-dimensional slice of it. (In particular, this initial
// state defines a 3x3x1 region of the 3-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following
// configurations, where the result of each cycle is shown layer-by-layer at
// each given z coordinate (and the frame of view follows the active cells in
// each cycle):
//
// Before any cycles:
//
// z=0
// .#.
// ..#
// ###
//
//
// After 1 cycle:
//
// z=-1
// #..
// ..#
// .#.
//
// z=0
// #.#
// .##
// .#.
//
// z=1
// #..
// ..#
// .#.
//
//
// After 2 cycles:
//
// z=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=0
// ##...
// ##...
// #....
// ....#
// .###.
//
// z=1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=2
// .....
// .....
// ..#..
// .....
// .....
//
//
// After 3 cycles:
//
// z=-2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
//
// z=-1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=0
// ...#...
// .......
// #......
// .......
// .....##
// .##.#..
// ...#...
//
// z=1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
//
// After the full six-cycle boot process completes, 112 cubes are left in the
// active state.
//
// Starting with your given initial configuration, simulate six cycles. How
// many cubes are left in the active state after the sixth cycle?

// --- Part Two ---
//
// For some reason, your simulated results don't match what the experimental
// energy source engineers expected. Apparently, the pocket dimension actually
// has four spatial dimensions, not three.
//
// The pocket dimension contains an infinite 4-dimensional grid. At every
// integer 4-dimensional coordinate (x,y,z,w), there exists a single cube
// (really, a hypercube) which is still either active or inactive.
//
// Each cube only ever considers its neighbors: any of the 80 other cubes where
// any of their coordinates differ by at most 1. For example, given the cube at
// x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube
// at x=0,y=2,z=3,w=4, and so on.
//
// The initial state of the pocket dimension still consists of a small flat
// region of cubes. Furthermore, the same rules for cycle updating still apply:
// during each cycle, consider the number of active neighbors of each cube.
//
// For example, consider the same initial state as in the example above. Even
// though the pocket dimension is 4-dimensional, this initial state represents
// a small 2-dimensional slice of it. (In particular, this initial state
// defines a 3x3x1x1 region of the 4-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following
// configurations, where the result of each cycle is shown layer-by-layer at
// each given z and w coordinate:
//
// Before any cycles:
//
// z=0, w=0
// .#.
// ..#
// ###
//
//
// After 1 cycle:
//
// z=-1, w=-1
// #..
// ..#
// .#.
//
// z=0, w=-1
// #..
// ..#
// .#.
//
// z=1, w=-1
// #..
// ..#
// .#.
//
// z=-1, w=0
// #..
// ..#
// .#.
//
// z=0, w=0
// #.#
// .##
// .#.
//
// z=1, w=0
// #..
// ..#
// .#.
//
// z=-1, w=1
// #..
// ..#
// .#.
//
// z=0, w=1
// #..
// ..#
// .#.
//
// z=1, w=1
// #..
// ..#
// .#.
//
//
// After 2 cycles:
//
// z=-2, w=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1, w=-2
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=-2
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=1, w=-2
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-2, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=-1, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=-2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=-1, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=-2, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=-1, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=-2, w=2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1, w=2
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=2
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=1, w=2
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=2
// .....
// .....
// ..#..
// .....
// .....
//
// After the full six-cycle boot process completes, 848 cubes are left in the
// active state.
//
// Starting with your given initial configuration, simulate six cycles in a
// 4-dimensional space. How many cubes are left in the active state after the
// sixth cycle?

const SIZE_FIELD: usize = 2 + 6 + 8 + 6 + 2;
const X_START: usize = 2 + 6;

// const SIZE_FIELD: usize = 11;
// const X_START: usize = 4;

// fn printfield(field: &[[[[u32; SIZE_FIELD]; SIZE_FIELD]; SIZE_FIELD]; SIZE_FIELD]) {
//     for (z, plane) in field.iter().enumerate() {
//         println!("");
//         println!("z={}", z);
//         for line in plane {
//             let line2 = line.iter().map(|c| match c {
//                 1 => '#',
//                 _ => '.',
//             }).collect::<String>();
//             println!("{:?}", line2);
//         }
//     }
// }

// You know this feeling in programming where you are like, hmm, there must
// be a better way to do this!
fn stepfield(mut field: [[[[u32; SIZE_FIELD]; SIZE_FIELD]; SIZE_FIELD]; SIZE_FIELD]) -> [[[[u32; SIZE_FIELD]; SIZE_FIELD]; SIZE_FIELD]; SIZE_FIELD] {
    let field_old = field.clone();
    for z in 1..SIZE_FIELD-1 {
        for x in 1..SIZE_FIELD-1 {
            for y in 1..SIZE_FIELD-1 {
                for w in 1..SIZE_FIELD - 1 {
                    let c = field_old[z][x][y][w];
                    let mut neighbours: u32 = 0;
                    for zi in (z - 1)..(z + 2) {
                        for xi in (x - 1)..(x + 2) {
                            for yi in (y - 1)..(y + 2) {
                                for wi in (w - 1)..(w + 2) {
                                    neighbours += field_old[zi][xi][yi][wi];
                                }
                            }
                        }
                    }
                    neighbours -= field_old[z][x][y][w];
                    match (c, neighbours) {
                        (1, 2) => { field[z][x][y][w] = 1; }
                        (1, 3) => { field[z][x][y][w] = 1; }
                        (0, 3) => { field[z][x][y][w] = 1; }
                        _ => { field[z][x][y][w] = 0; }
                    }
                }
            }
        }
    }
    field
}


fn main() {
    println!("Advent of Code 2020 Day 17!");


    let mut field = [[[[0u32; SIZE_FIELD];  SIZE_FIELD]; SIZE_FIELD]; SIZE_FIELD];

    let filename = "inputexample.txt";
    // let filename = "input.txt";
    let fieldstart: String = std::fs::read_to_string(filename)
        .expect("Error reading file.");

    let mut x = X_START;
    let mut y = X_START;
    for c in fieldstart.chars() {
        match c {
            '.' => {
                y += 1;
            }
            '#' => {
                field[X_START][x][y][X_START] = 1;
                y += 1;
            }
            '\n' => {
                y = X_START;
                x += 1;
            }
            _ => unreachable!()
        }
    }

    for cycle in 0..7 {
        // printfield(&field);

        let mut totalactive: u32 = 0;
        for p in &field {
            for l in p {
                for q in l {
                    totalactive += q.iter().sum::<u32>();
                }
            }
        }

        println!("Cycle {}, Active {}", cycle, totalactive);
        field = stepfield(field);
    }
}
