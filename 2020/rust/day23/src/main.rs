//! --- Day 23: Crab Cups ---
//
// The small crab challenges you to a game! The crab is going to mix up some
// cups, and you have to predict where they'll end up.
//
// The cups will be arranged in a circle and labeled clockwise (your puzzle
// input). For example, if your labeling were 32415, there would be five cups
// in the circle; going clockwise around the circle from the first cup, the
// cups would be labeled 3, 2, 4, 1, 5, and then back to 3 again.
//
// Before the crab starts, it will designate the first cup in your list as the
// current cup. The crab is then going to do 100 moves.
//
// Each move, the crab does the following actions:
//
//   - The crab picks up the three cups that are immediately clockwise of the
//     current cup. They are removed from the circle; cup spacing is adjusted
//     as necessary to maintain the circle.
//   - The crab selects a destination cup: the cup with a label equal to the
//     current cup's label minus one. If this would select one of the cups that
//     was just picked up, the crab will keep subtracting one until it finds a
//     cup that wasn't just picked up. If at any point in this process the
//     value goes below the lowest value on any cup's label, it wraps around to
//     the highest value on any cup's label instead.
//   - The crab places the cups it just picked up so that they are immediately
//     clockwise of the destination cup. They keep the same order as when they
//     were picked up.
//   - The crab selects a new current cup: the cup which is immediately
//     clockwise of the current cup.
//
// For example, suppose your cup labeling were 389125467. If the crab were to
// do merely 10 moves, the following changes would occur:
//
// -- move 1 --
// cups: (3) 8  9  1  2  5  4  6  7
// pick up: 8, 9, 1
// destination: 2
//
// -- move 2 --
// cups:  3 (2) 8  9  1  5  4  6  7
// pick up: 8, 9, 1
// destination: 7
//
// -- move 3 --
// cups:  3  2 (5) 4  6  7  8  9  1
// pick up: 4, 6, 7
// destination: 3
//
// -- move 4 --
// cups:  7  2  5 (8) 9  1  3  4  6
// pick up: 9, 1, 3
// destination: 7
//
// -- move 5 --
// cups:  3  2  5  8 (4) 6  7  9  1
// pick up: 6, 7, 9
// destination: 3
//
// -- move 6 --
// cups:  9  2  5  8  4 (1) 3  6  7
// pick up: 3, 6, 7
// destination: 9
//
// -- move 7 --
// cups:  7  2  5  8  4  1 (9) 3  6
// pick up: 3, 6, 7
// destination: 8
//
// -- move 8 --
// cups:  8  3  6  7  4  1  9 (2) 5
// pick up: 5, 8, 3
// destination: 1
//
// -- move 9 --
// cups:  7  4  1  5  8  3  9  2 (6)
// pick up: 7, 4, 1
// destination: 5
//
// -- move 10 --
// cups: (5) 7  4  1  8  3  9  2  6
// pick up: 7, 4, 1
// destination: 3
//
// -- final --
// cups:  5 (8) 3  7  4  1  9  2  6
//
// In the above example, the cups' values are the labels as they appear moving
// clockwise around the circle; the current cup is marked with ( ).
//
// After the crab is done, what order will the cups be in? Starting after the
// cup labeled 1, collect the other cups' labels clockwise into a single string
// with no extra characters; each number except 1 should appear exactly once.
// In the above example, after 10 moves, the cups clockwise from 1 are labeled
// 9, 2, 6, 5, and so on, producing 92658374. If the crab were to complete all
// 100 moves, the order after cup 1 would be 67384529.
//
// Using your labeling, simulate 100 moves. What are the labels on the cups
// after cup 1?
//
// Your puzzle input is 589174263.

// --- Part Two ---
//
// Due to what you can only assume is a mistranslation (you're not exactly
// fluent in Crab), you are quite surprised when the crab starts arranging many
// cups in a circle on your raft - one million (1000000) in total.
//
// Your labeling is still correct for the first few cups; after that, the
// remaining cups are just numbered in an increasing fashion starting from the
// number after the highest number in your list and proceeding one by one until
// one million is reached. (For example, if your labeling were 54321, the cups
// would be numbered 5, 4, 3, 2, 1, and then start counting up from 6 until one
// million is reached.) In this way, every number from one through one million
// is used exactly once.
//
// After discovering where you made the mistake in translating Crab Numbers,
// you realize the small crab isn't going to do merely 100 moves; the crab is
// going to do ten million (10000000) moves!
//
// The crab is going to hide your stars - one each - under the two cups that
// will end up immediately clockwise of cup 1. You can have them if you predict
// what the labels on those cups will be when the crab is finished.
//
// In the above example (389125467), this would be 934001 and then 159792;
// multiplying these together produces 149245887792.
//
// Determine which two cups will end up immediately clockwise of cup 1. What do
// you get if you multiply their labels together?

use std::collections::LinkedList;

/// Subtract 1 modulo 9
fn subtract1(x: usize) -> usize {
    ((x + 7) % 9) + 1
}

fn main() {
    println!("Advent of Code 2020 Day 23!");

    // Put the current cup at the end instead of at the beginning.
    // let mut cups = vec![8, 9, 1, 2, 5, 4, 6, 7, 3];  // example
    let mut cups = vec![3_usize, 8, 9, 1, 2, 5, 4, 6, 7];  // example without current cup at the end
    cups.rotate_left(1);

    let mut cupsll: LinkedList<usize> = LinkedList::new();
    for c in cups.iter() {
        cupsll.push_back(*c);
    }

    assert_eq!(cups.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(""), "891254673"); // example "389125467"
    assert_eq!(cupsll.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(""), "891254673"); // example "389125467"
    // for i in 10_usize..=1000000 {
    //     cups.push(i);
    // }
    // assert_eq!(*cups.last().unwrap(), 1000000_usize);
    // assert_eq!(cups.len(), 1000000_usize);

    // let mut cups = vec![5, 8, 9, 1, 7, 4, 2, 6, 3];  // puzzle, without the current cup at the end, so wrong
    // cups.rotate_left(1);
    // let mut cups = vec![8, 9, 1, 7, 4, 2, 6, 3, 5];  // puzzle, moving the current cup to the end, so right
    // let nr_of_moves = 10;
    // let nr_of_moves = 100;

    // let nr_of_moves = 10000;
    // 10000 moves take 5 minutes, so 10 million moves take 5000 minutes
    // so about 100 hours, so 4 days. Doable, but not really.
    // But suddenly it takes only 16 seconds, so 10 million will take 16000
    // seconds, so 300 minutes.

    let nr_of_moves = 10000;
    // let nr_of_moves = 10000000;

    for move_counter in 1..=nr_of_moves {
        println!("-- move {} --", move_counter);
        // println!("{:?}", cups);
        // Method 1
        // let picked_up = cups.drain(0..3).collect::<Vec<_>>();
        // Method 2
        // let p0 = cups.remove(0);
        // let p1 = cups.remove(0);
        // let p2 = cups.remove(0);
        // Method 3
        // let p0 = cups[0];
        // let p1 = cups[1];
        // let p2 = cups[2];
        // for _ in cups.drain(0..3) {}
        // let cups2 = cups.split_off(3);
        // cups = cups2;

        let templl = cupsll.split_off(3);
        let mut picked_up_ll = cupsll;
        cupsll = templl;

        // println!("{:?}", cups);
        // println!("{:?}", picked_up);
        // println!("picked_up: {} {} {}", p0, p1, p2);
        println!("Picked up ll: {:?}", picked_up_ll);

        // let mut destination = subtract1(*cups.last().unwrap());
        // while picked_up.contains(&destination) {
        // while p0 == destination || p1 == destination || p2 == destination {
        //     destination = subtract1(destination);
        // }

        let mut destination_ll = subtract1(*cupsll.back().unwrap());
        while picked_up_ll.contains(&destination_ll) {
            destination_ll = subtract1(destination_ll);
        }
        // println!("Destination {} {}", destination, destination_ll);

        // let position = cups.iter().position(|x| x == &destination).unwrap();
        let position_ll = cupsll.iter().position(|x| x == &destination_ll).unwrap();
        // println!("Destination    {} at position {}", destination, position);
        println!("Destination LL {} at position {}", destination_ll, position_ll);

        // for cup in picked_up.iter().rev() {
        //     cups.insert(position + 1, *cup);
        // }
        let mut part2 = cupsll.split_off(position_ll + 1);
        cupsll.append(&mut picked_up_ll);
        cupsll.append(&mut part2);

        // cups.insert(position + 1, p2);
        // cups.insert(position + 1, p1);
        // cups.insert(position + 1, p0);

        // remove + push is apparently a bit faster
        // cups.rotate_left(1);
        let cup_current = cups.remove(0);
        cups.push(cup_current);
        let cup_current_ll = cupsll.pop_front().unwrap();
        cupsll.push_back(cup_current_ll);

        // println!();
    }

    if false {
        // part 1 code
        // println!("Cups1 {:?}", cups);
        // while cups[0] != 1 {
        //     cups.rotate_left(1);
        // }
        while cupsll.front().unwrap() != &1 {
            let cup_current_ll = cupsll.pop_front().unwrap();
            cupsll.push_back(cup_current_ll);
        }
        // println!("Cups2 {:?}", cups);
        // let ll = cups.iter().skip(1).map(|x| format!("{}", x)).collect::<Vec<_>>().join("");
        // println!("ll: {:?}", ll);
        // assert_eq!(ll, "92658374"); // demo 10
        // assert_eq!(ll, "67384529"); // demo 100
        // assert_eq!(ll, "43896725"); // puzzle 100

        let llll = cupsll.iter().skip(1).map(|x| format!("{}", x)).collect::<Vec<_>>().join("");
        println!("llll: {:?}", llll);
        // assert_eq!(llll, "92658374"); // demo 10
        assert_eq!(llll, "67384529"); // demo 100


        // let p2 = cups[1] * cups[2];
        // println!("p2: {}", p2);
        // assert_eq!(149245887792, p2);
    }

    // let position = cups.iter().position(|x| x == &1).unwrap();
    // let p2 = cups[position + 1] * cups[position + 2];

    // println!("cups: {:?}", cupsll);
    let mut ii = cupsll.iter();
    while ii.next().unwrap() != &1 {}
    let pa = ii.next().unwrap();
    let pb = ii.next().unwrap();
    let p2 = pa * pb;
    println!("pos, pa, pb, p2: {} {} {}", pa, pb, p2);
    // assert_eq!(149245887792, p2);

}
