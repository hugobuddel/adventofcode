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

use std::fs;

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

// A number is correct if it matches at least one of the rules.
fn validateone(number: &usize, rules: &Vec<TicketRule>) -> usize {
    for rule in rules {
        if (*number >= rule.start1 && rule.end1 >= *number) ||
           (*number >= rule.start2 && rule.end2 >= *number) {
            // println!("Correct: {}, {}-{}, {}-{}", number, rule.start1, rule.end1, rule.start2, rule.end2);
            return 0
        }
    }
    *number
}

fn validate(ticket: &Vec<usize>, rules: &Vec<TicketRule>) -> usize {
    ticket.iter().map(|number| validateone(number, rules)).sum()
}

fn main() {
    println!("Advent of Code 2020 Day 16!");

    // let filename = "inputexample.txt";
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
    for ticket in othertickets {
        let ok_ticket = validate(&ticket, &rules);
        // println!("Ticket {:?} is {}!", ticket, ok_ticket);
        number_bad += ok_ticket;
    }

    println!("Total badness is {}", number_bad);
    assert_eq!(27911, number_bad);
}
