//! --- Day 11: Seating System ---
//
// --- Part Two ---
//
// As soon as people start to arrive, you realize your mistake. People don't
// just care about adjacent seats - they care about the first seat they can
// see in each of those eight directions!
//
// Now, instead of considering just the eight immediately adjacent seats,
// consider the first seat in each of those eight directions. For example,
// the empty seat below would see eight occupied seats:
//
// .......#.
// ...#.....
// .#.......
// .........
// ..#L....#
// ....#....
// .........
// #........
// ...#.....
//
// The leftmost empty seat below would only see one empty seat, but cannot see
// any of the occupied ones:
//
// .............
// .L.L.#.#.#.#.
// .............
//
// The empty seat below would see no occupied seats:
//
// .##.##.
// #.#.#.#
// ##...##
// ...L...
// ##...##
// #.#.#.#
// .##.##.
//
// Also, people seem to be more tolerant than you expected: it now takes five
// or more visible occupied seats for an occupied seat to become empty (rather
// than four or more from the previous rules). The other rules still apply:
// empty seats that see no occupied seats become occupied, seats matching no
// rule don't change, and floor never changes.
//
// Given the same starting layout as above, these new rules cause the seating
// area to shift around as follows:
//
// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL
//
// #.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##
//
// #.LL.LL.L#
// #LLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLLL.L
// #.LLLLL.L#
//
// #.L#.##.L#
// #L#####.LL
// L.#.#..#..
// ##L#.##.##
// #.##.#L.##
// #.#####.#L
// ..#.#.....
// LLL####LL#
// #.L#####.L
// #.L####.L#
//
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##LL.LL.L#
// L.LL.LL.L#
// #.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLL#.L
// #.L#LL#.L#
//
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.#L.L#
// #.L####.LL
// ..#.#.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
//
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.LL.L#
// #.LLLL#.LL
// ..#.L.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
//
// Again, at this point, people stop shifting around and the seating area
// reaches equilibrium. Once this occurs, you count 26 occupied seats.
//
// Given the new visibility method and the rule change for occupied seats
// becoming empty, once equilibrium is reached, how many seats end up occupied?

use std::fs;

#[derive(Debug)]
struct Seats {
    rows: Vec<Vec<char>>,
    rows_prev:  Vec<Vec<char>>,
}

impl Seats {
    fn from_filename(filename: &str) -> Seats {
        let contents = fs::read_to_string(filename).unwrap();
        let myvec: Vec<Vec<char>> = contents.trim().split("\n").map(|x| x.chars().collect()).collect();

        Seats {
            rows: myvec,
            rows_prev: Vec::new(),
        }
    }

    fn step(&mut self) {
        // Copy the board because we need to check it while modifying.
        self.rows_prev = self.rows.clone();
        for x in 0..self.rows.len() {
            for y in 0..self.rows[0].len() {
                let mut count_occupied = 0;
                // let mut count_space = 0;
                for xi in (x as i32) - 1..(x as i32) + 2 {
                    for yi in (y as i32) -1..(y as i32) + 2 {
                        if xi >= 0 &&
                            xi < self.rows.len() as i32 &&
                            yi >= 0 &&
                            yi < self.rows[0].len() as i32 &&
                            !(xi == x as i32 && yi == y as i32)
                        {
                            // count_space += 1;
                            if self.rows_prev[xi as usize][yi as usize] == '#'
                            {
                                count_occupied += 1;
                            }
                        }
                    }
                }
                // println!("{} {} {} {} {}", x, y, self.rows[x][y], count_space, count_occupied);
                if self.rows[x][y] == 'L' && count_occupied == 0 {
                    self.rows[x][y] = '#';
                } else if self.rows[x][y] == '#' && count_occupied >= 5 {
                    self.rows[x][y] = 'L';
                }
            }
        }
    }

    fn occupied(&self) -> usize {
        let mut count_occupied = 0;
        for row in &self.rows {
            for seat in row {
                if *seat == '#' {
                    count_occupied += 1;
                }
            }
        }
        count_occupied
    }

    fn pprint(&self) {
        println!();
        for line in &self.rows {
            println!("{:?}", line.iter().collect::<String>());
        }
        println!("Occupied: {}", &self.occupied());
    }
}

fn main() {
    println!("Advent of Code 2020 Day 11!");
    let filename = "inputexample.txt";
    // let filename = "input.txt";
    let mut seats = Seats::from_filename(filename);
    // println!("{:?}", seats);
    while seats.rows_prev != seats.rows {
        // seats.pprint();
        seats.step();
    }
    seats.pprint();
}
