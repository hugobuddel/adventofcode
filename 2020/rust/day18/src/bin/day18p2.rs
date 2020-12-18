//! --- Day 18: Operation Order ---
//
// --- Part Two ---
//
// You manage to answer the child's questions and they finish part 1 of their
// homework, but get stuck when they reach the next section: advanced math.
//
// Now, addition and multiplication have different precedence levels, but
// they're not the ones you're familiar with. Instead, addition is evaluated
// before multiplication.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are
// now as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
//   3   * 3 + 4 * 5 + 6
//   3   *   7   * 5 + 6
//   3   *   7   *  11
//      21       *  11
//          231
//
// Here are the other examples from above:
//
//     1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
//     2 * 3 + (4 * 5) becomes 46.
//     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
//     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
//     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
//
// What do you get if you add up the results of evaluating the homework
// problems using these new rules?

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "calculator2.pest"]
pub struct CalculatorParser;

fn evaluate_term(term: &Pair<Rule>) -> i128 {
    match term.as_rule() {
        Rule::number => {term.as_str().parse().unwrap()}
        Rule::expression => {evaluate(term)}
        _ => {
            println!("Huh {:?}", term);
            unreachable!()
        }
    }
}

fn evaluate(expression: &Pair<Rule>) -> i128 {
    let mut pair = expression.clone().into_inner();
    let mut value = evaluate_term(&pair.next().unwrap());
    let actions = &pair.next().unwrap();
    for action in actions.clone().into_inner() {
        let mut pairaction = action.into_inner();
        let operator = pairaction.next().unwrap();
        let value2 = evaluate_term(&pairaction.next().unwrap());
        match operator.as_str() {
            "+" => {value += value2}
            "-" => {value -= value2}
            "*" => {value *= value2}
            _ => {unreachable!()}
        }
    }
    value
}

fn main() {
    println!("Advent of Code 2020 Day 18 part 2!");

    let filename = "inputexample.txt";
    // let filename = "input.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let calculatorfile = CalculatorParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let mut thesum = 0_i128;
    let mut calculator = calculatorfile.into_inner();
    for expression in calculator.next().unwrap().into_inner() {
        let value = evaluate(&expression);
        println!("Expression {:?} = {}", expression.as_str(), value);
        thesum += value;
    }
    println!("Total: {}", thesum);
}
