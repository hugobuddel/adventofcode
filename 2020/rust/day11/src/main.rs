//! --- Day 11: Seating System ---
//
// Your plane lands with plenty of time to spare. The final leg of your journey
// is a ferry that goes directly to the tropical island where you can finally
// start your vacation. As you reach the waiting area to board the ferry, you
// realize you're so early, nobody else has even arrived yet!
//
// By modeling the process people use to choose (or abandon) their seat in the
// waiting area, you're pretty sure you can predict the best place to sit. You
// make a quick map of the seat layout (your puzzle input).
//
// The seat layout fits neatly on a grid. Each position is either floor (.), an
// empty seat (L), or an occupied seat (#). For example, the initial seat
// layout might look like this:
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
// Now, you just need to model the people who will be arriving shortly.
// Fortunately, people are entirely predictable and always follow a simple set
// of rules. All decisions are based on the number of occupied seats adjacent
// to a given seat (one of the eight positions immediately up, down, left,
// right, or diagonal from the seat). The following rules are applied to every
// seat simultaneously:
//
//     If a seat is empty (L) and there are no occupied seats adjacent to it,
//     the seat becomes occupied.
//
//     If a seat is occupied (#) and four or more seats adjacent to it are also
//     occupied, the seat becomes empty.
//
//     Otherwise, the seat's state does not change.
//
// Floor (.) never changes; seats don't move, and nobody sits on the floor.
//
// After one round of these rules, every seat in the example layout becomes
// occupied:
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
// After a second round, the seats with four or more occupied adjacent seats
// become empty again:
//
// #.LL.L#.##
// #LLLLLL.L#
// L.L.L..L..
// #LLL.LL.L#
// #.LL.LL.LL
// #.LLLL#.##
// ..L.L.....
// #LLLLLLLL#
// #.LLLLLL.L
// #.#LLLL.##
//
// This process continues for three more rounds:
//
// #.##.L#.##
// #L###LL.L#
// L.#.#..#..
// #L##.##.L#
// #.##.LL.LL
// #.###L#.##
// ..#.#.....
// #L######L#
// #.LL###L.L
// #.#L###.##
//
// #.#L.L#.##
// #LLL#LL.L#
// L.L.L..#..
// #LLL.##.L#
// #.LL.LL.LL
// #.LL#L#.##
// ..L.L.....
// #L#LLLL#L#
// #.LLLLLL.L
// #.#L#L#.##
//
// #.#L.L#.##
// #LLL#LL.L#
// L.#.L..#..
// #L##.##.L#
// #.#L.LL.LL
// #.#L#L#.##
// ..L.L.....
// #L#L##L#L#
// #.LLLLLL.L
// #.#L#L#.##
//
// At this point, something interesting happens: the chaos stabilizes and
// further applications of these rules cause no seats to change state! Once
// people stop moving around, you count 37 occupied seats.
//
// Simulate your seating area by applying the seating rules repeatedly until
// no seats change state. How many seats end up occupied?

use std::fs;

#[derive(Debug)]
struct Seats {
    rows: Vec<Vec<char>>,
    rows_prev:  Vec<Vec<char>>,
    size: usize,
}

impl Seats {
    fn from_filename(filename: &str) -> Seats {
        let contents = fs::read_to_string(filename).unwrap();
        let myvec: Vec<Vec<char>> = contents.trim().split("\n").map(|x| x.chars().collect()).collect();
        let size = myvec.len();

        Seats {
            rows: myvec,
            rows_prev: Vec::new(),
            size: size,
        }
    }

    fn step(&mut self) {
        // Copy the board because we need to check it while modifying.
        self.rows_prev = self.rows.clone();
        for x in 0..self.size {
            for y in 0..self.size {
                let mut count_occupied = 0;
                let mut count_space = 0;
                for xi in (x as i32) - 1..(x as i32) + 2 {
                    for yi in (y as i32) -1..(y as i32) + 2 {
                        if xi >= 0 &&
                            xi < self.size as i32 &&
                            yi >= 0 &&
                            yi < self.size as i32 &&
                            !(xi == x as i32 && yi == y as i32)
                        {
                            count_space += 1;
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
                } else if self.rows[x][y] == '#' && count_occupied >= 4 {
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
    // let q = -1..1;
    // println!("q {}", q.collect::<Vec<_>>().len());
    let filename = "inputexample.txt";
    let mut seats = Seats::from_filename(filename);
    println!("{:?}", seats);
    while seats.rows_prev != seats.rows {
        seats.pprint();
        seats.step();
    }
    seats.pprint();
}
