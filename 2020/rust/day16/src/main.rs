//! --- Day 16: Ticket Translation ---
//
// As you're walking to yet another connecting flight, you realize that one of
// the legs of your re-routed trip coming up is on a high-speed train. However,
// the train ticket you were given is in a language you don't understand. You
// should probably figure out what it says before you get to the train station
// after the next flight.
//
// Unfortunately, you can't actually read the words on the ticket. You can,
// however, read the numbers, and so you figure out the fields these tickets
// must have and the valid ranges for values in those fields.
//
// You collect the rules for ticket fields, the numbers on your ticket, and the
// numbers on other nearby tickets for the same train service (via the airport
// security cameras) together into a single document you can reference (your
// puzzle input).
//
// The rules for ticket fields specify a list of fields that exist somewhere on
// the ticket and the valid ranges of values for each field. For example, a
// rule like class: 1-3 or 5-7 means that one of the fields in every ticket is
// named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such
// that 3 and 5 are both valid in this field, but 4 is not).
//
// Each ticket is represented by a single line of comma-separated values. The
// values are the numbers on the ticket in the order they appear; every ticket
// has the same format. For example, consider this ticket:
//
// .--------------------------------------------------------.
// | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
// |                                                        |
// | ??: 301  ??: 302             ???????: 303      ??????? |
// | ??: 401  ??: 402           ???? ????: 403    ????????? |
// '--------------------------------------------------------'
//
// Here, ? represents text in a language you don't understand. This ticket
// might be represented as 101,102,103,104,301,302,303,401,402,403; of course,
// the actual train tickets you're looking at are much more complicated. In any
// case, you've extracted just the numbers in such a way that the first number
// is always the same specific field, the second number is always a different
// specific field, and so on - you just don't know what each position actually
// means!
//
// Start by determining which tickets are completely invalid; these are tickets
// that contain values which aren't valid for any field. Ignore your ticket for
// now.
//
// For example, suppose you have the following notes:
//
// class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50
//
// your ticket:
// 7,1,14
//
// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12
//
// It doesn't matter which position corresponds to which field; you can
// identify invalid nearby tickets by considering only whether tickets contain
// values that are not valid for any field. In this example, the values on the
// first nearby ticket are all valid for at least one field. This is not true
// of the other three nearby tickets: the values 4, 55, and 12 are are not
// valid for any field. Adding together all of the invalid values produces
// your ticket scanning error rate: 4 + 55 + 12 = 71.
//
// Consider the validity of the nearby tickets you scanned. What is your ticket
// scanning error rate?

// --- Part Two ---
//
// Now that you've identified which tickets contain invalid values, discard
// those tickets entirely. Use the remaining valid tickets to determine which
// field is which.
//
// Using the valid ranges for each field, determine what order the fields
// appear on the tickets. The order is consistent between all tickets: if seat
// is the third field, it is the third field on every ticket, including your
// ticket.
//
// For example, suppose you have the following notes:
//
// class: 0-1 or 4-19
// row: 0-5 or 8-19
// seat: 0-13 or 16-19
//
// your ticket:
// 11,12,13
//
// nearby tickets:
// 3,9,18
// 15,1,5
// 5,14,9
//
// Based on the nearby tickets in the above example, the first position must be
// row, the second position must be class, and the third position must be seat;
// you can conclude that in your ticket, class is 12, row is 11, and seat is 13.
//
// Once you work out which field is which, look for the six fields on your
// ticket that start with the word departure. What do you get if you multiply
// those six values together?

use std::fs;
use std::collections::HashSet;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "tickets.pest"]
pub struct TicketParser;

#[derive(Debug)]
struct TicketRule {
    name: String,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
}

impl TicketRule {
    fn from_pair(pair: &Pair<Rule>) -> TicketRule {
        let mut pairs = pair.clone().into_inner();
        TicketRule {
            name: pairs.next().unwrap().as_str().to_string(),
            start1: pairs.next().unwrap().as_str().parse().unwrap(),
            end1: pairs.next().unwrap().as_str().parse().unwrap(),
            start2: pairs.next().unwrap().as_str().parse().unwrap(),
            end2: pairs.next().unwrap().as_str().parse().unwrap(),
        }
    }
}

// Validate one number with one rule.
fn validateonenumberrule(number: &usize, rule: &TicketRule) -> bool {
    (*number >= rule.start1 && rule.end1 >= *number) ||
           (*number >= rule.start2 && rule.end2 >= *number)
}

// A number is correct if it matches at least one of the rules.
fn validateone(number: &usize, rules: &Vec<TicketRule>) -> usize {
    for rule in rules {
        if validateonenumberrule(number, rule) {
            // println!("Correct: {}, {}-{}, {}-{}", number, rule.start1, rule.end1, rule.start2, rule.end2);
            return 0
        }
    }
    *number
}

// Does a ticket match all the rules?
fn validate(ticket: &Vec<usize>, rules: &Vec<TicketRule>) -> usize {
    ticket.iter().map(|number| validateone(number, rules)).sum()
}

// Are all values correct for the rule?
fn validaterule(values: &Vec<usize>, rule: &TicketRule) -> bool {
    values.iter().all(|v| validateonenumberrule(v, rule))
}

// https://stackoverflow.com/questions/29669287/how-can-i-zip-more-than-two-iterators
// fn transpose_records<T: Clone>(records: &Vec<Vec<T>>) -> Vec<Vec<T>> {
fn transpose_records<T: Clone>(records: &Vec<&Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed: Vec<Vec<T>> = vec![Vec::new(); records[0].len()];

    for record in records {
        for (index, element) in record.iter().enumerate() {
            transposed[index].push(element.clone());
        }
    }

    transposed
}

fn main() {
    println!("Advent of Code 2020 Day 16!");

    // let filename = "inputexample.txt";
    // let filename = "inputexample2.txt";
    let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let ticketsfile = TicketParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();
    // println!("{}", ticketsfile.as_str());

    let mut pairs = ticketsfile.into_inner();
    let rules:Vec<TicketRule> = pairs.next().unwrap().into_inner().map(|x| TicketRule::from_pair(&x)).collect();
    println!("Rules: {:?}", rules);

    // List of integers.
    let ticket:Vec<usize> = pairs.next().unwrap().into_inner().map(|x| x.as_str().parse().unwrap()).collect();
    println!("Ticket: {:?}", ticket);

    // List of lists of integers.
    let othertickets:Vec<Vec<usize>> = pairs.next().unwrap().into_inner()
        .map(|ts| ts.into_inner().map(|t| t.as_str().parse().unwrap()).collect())
        .collect();
    // println!("Other Tickets: {:?}", othertickets);

    let mut number_bad: usize = 0;
    for ticket in othertickets.clone() {
        let ok_ticket = validate(&ticket, &rules);
        // println!("Ticket {:?} is {}!", ticket, ok_ticket);
        number_bad += ok_ticket;
    }

    println!("Total badness is {}", number_bad);
    // assert_eq!(27911, number_bad);

    let tickets_good:Vec<&Vec<usize>> = othertickets.iter()
        .filter(|ticket| validate(&ticket, &rules) == 0)
        .collect();
    println!("Number of good tickets is {}.", tickets_good.len());

    // let rowvalues: Vec<Vec<usize>> = tickets_good.iter().zip


    // Interlude! Lets learn how to transpose a Vec<Vec<usize>>
    // https://stackoverflow.com/questions/29669287/how-can-i-zip-more-than-two-iterators
    use itertools::izip;

    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let c = [7, 8, 9];

    // izip!() accepts iterators and/or values with IntoIterator.
    for (x, y, z) in izip!(&a, &b, &c) {
        println!("XYZ1 {} {} {}", x, y, z);
    }
    // Not clear how to scale this to Vec<Vec<usize>>

    // println!("Good Tickets: {:?}", tickets_good);
    let rowvalues: Vec<Vec<usize>> = transpose_records(&tickets_good);
    // println!("Row Values:   {:?}", rowvalues);

    for (i, row) in rowvalues.iter().enumerate() {
        println!("Field {} matches:", i);
        for rule in &rules {
            if validaterule(row, &rule) {
                println!(" {}", rule.name);
            }
        }
    }

    // Store the names of all possible fields for each row.
    // let mut fieldposibilities: Vec<HashSet<&str>> = Vec::new();
    let mut fieldposibilities: Vec<HashSet<&str>> = rowvalues.iter()
        .map(|row| rules.iter().filter(|rule| validaterule(row, &rule))
            .map(|rule| rule.name.as_str()).collect()).collect();

    for (i, fields) in fieldposibilities.iter().enumerate() {
        println!("Field {} matches: {:?}", i, fields);
    }
}
