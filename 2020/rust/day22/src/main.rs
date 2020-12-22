//! --- Day 22: Crab Combat ---
//
// It only takes a few hours of sailing the ocean on a raft for boredom to sink
// in. Fortunately, you brought a small deck of space cards! You'd like to play
// a game of Combat, and there's even an opponent available: a small crab that
// climbed aboard your raft before you left.
//
// Fortunately, it doesn't take long to teach the crab the rules.
//
// Before the game starts, split the cards so each player has their own deck
// (your puzzle input). Then, the game consists of a series of rounds: both
// players draw their top card, and the player with the higher-valued card wins
// the round. The winner keeps both cards, placing them on the bottom of their
// own deck so that the winner's card is above the other card. If this causes a
// player to have all of the cards, they win, and the game ends.
//
// For example, consider the following starting decks:
//
// Player 1:
// 9
// 2
// 6
// 3
// 1
//
// Player 2:
// 5
// 8
// 4
// 7
// 10
//
// This arrangement means that player 1's deck contains 5 cards, with 9 on top
// and 1 on the bottom; player 2's deck also contains 5 cards, with 5 on top
// and 10 on the bottom.
//
// The first round begins with both players drawing the top card of their
// decks: 9 and 5. Player 1 has the higher card, so both cards move to the
// bottom of player 1's deck such that 9 is above 5. In total, it takes 29
// rounds before a player has all of the cards:
//
// -- Round 1 --
// Player 1's deck: 9, 2, 6, 3, 1
// Player 2's deck: 5, 8, 4, 7, 10
// Player 1 plays: 9
// Player 2 plays: 5
// Player 1 wins the round!
//
// -- Round 2 --
// Player 1's deck: 2, 6, 3, 1, 9, 5
// Player 2's deck: 8, 4, 7, 10
// Player 1 plays: 2
// Player 2 plays: 8
// Player 2 wins the round!
//
// -- Round 3 --
// Player 1's deck: 6, 3, 1, 9, 5
// Player 2's deck: 4, 7, 10, 8, 2
// Player 1 plays: 6
// Player 2 plays: 4
// Player 1 wins the round!
//
// -- Round 4 --
// Player 1's deck: 3, 1, 9, 5, 6, 4
// Player 2's deck: 7, 10, 8, 2
// Player 1 plays: 3
// Player 2 plays: 7
// Player 2 wins the round!
//
// -- Round 5 --
// Player 1's deck: 1, 9, 5, 6, 4
// Player 2's deck: 10, 8, 2, 7, 3
// Player 1 plays: 1
// Player 2 plays: 10
// Player 2 wins the round!
//
// ...several more rounds pass...
//
// -- Round 27 --
// Player 1's deck: 5, 4, 1
// Player 2's deck: 8, 9, 7, 3, 2, 10, 6
// Player 1 plays: 5
// Player 2 plays: 8
// Player 2 wins the round!
//
// -- Round 28 --
// Player 1's deck: 4, 1
// Player 2's deck: 9, 7, 3, 2, 10, 6, 8, 5
// Player 1 plays: 4
// Player 2 plays: 9
// Player 2 wins the round!
//
// -- Round 29 --
// Player 1's deck: 1
// Player 2's deck: 7, 3, 2, 10, 6, 8, 5, 9, 4
// Player 1 plays: 1
// Player 2 plays: 7
// Player 2 wins the round!
//
//
// == Post-game results ==
// Player 1's deck:
// Player 2's deck: 3, 2, 10, 6, 8, 5, 9, 4, 7, 1
//
// Once the game ends, you can calculate the winning player's score. The bottom
// card in their deck is worth the value of the card multiplied by 1, the
// second-from-the-bottom card is worth the value of the card multiplied by 2,
// and so on. With 10 cards, the top card is worth the value on the card
// multiplied by 10. In this example, the winning player's score is:
//
//    3 * 10
// +  2 *  9
// + 10 *  8
// +  6 *  7
// +  8 *  6
// +  5 *  5
// +  9 *  4
// +  4 *  3
// +  7 *  2
// +  1 *  1
// = 306
//
// So, once the game ends, the winning player's score is 306.
//
// Play the small crab in a game of Combat using the two decks you just dealt.
// What is the winning player's score?

use std::fs;
use std::collections::VecDeque;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
// use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "spacecards.pest"]
pub struct SpacecardsParser;

fn main() {
    println!("Advent of Code 2020 Day 22!");

    let filename = "inputexample.txt";
    // let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let tilefile = SpacecardsParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    // All cards have 0 < value < 100, so there are at most 100 cards.
    let mut deck1: VecDeque<usize> = VecDeque::with_capacity(100);
    let mut deck2: VecDeque<usize> = VecDeque::with_capacity(100);
    let mut pair = tilefile.into_inner();
    let mut pairdecks = pair.next().unwrap().into_inner();
    for card in pairdecks.next().unwrap().into_inner() {
        // println!("Card {:?}", card);
        deck1.push_back(card.as_str().parse().unwrap());
    }
    for card in pairdecks.next().unwrap().into_inner() {
        // println!("Card {:?}", card);
        deck2.push_back(card.as_str().parse().unwrap());
    }

    let mut round: usize = 1;
    while deck1.len() > 0 && deck2.len() > 0
    {
        println!("-- Round {} --", round);
        println!("Player 1's deck: {:?}", deck1);
        println!("Player 2's deck: {:?}", deck2);
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        println!("Player 1 plays: {}", card1);
        println!("Player 2 plays: {}", card2);
        if card1 > card2 {
            println!("Player 1 wins the round!");
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else if card2 > card1 {
            println!("Player 2 wins the round!");
            deck2.push_back(card2);
            deck2.push_back(card1);
        } else {
            unreachable!("Two cards can never have the same value!");
        }
        round += 1;
        println!();
    }

    println!();
    println!("== Post-game results ==");
    println!("Player 1's deck: {:?}", deck1);
    println!("Player 2's deck: {:?}", deck2);

    let mut score: usize = 0;
    for (i, value) in deck1.iter().chain(deck2.iter()).rev().enumerate() {
        score += (i + 1) * value;
        // println!("{}", value);
    }
    println!("Final score: {}", score);
}
