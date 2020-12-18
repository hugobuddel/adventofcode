//! --- Day 18: Operation Order ---
//
// As you look out the window and notice a heavily-forested continent slowly
// appear over the horizon, you are interrupted by the child sitting next to
// you. They're curious if you could help them with their math homework.
//
// Unfortunately, it seems like this "math" follows different rules than you
// remember.
//
// The homework (your puzzle input) consists of a series of expressions that
// consist of addition (+), multiplication (*), and parentheses ((...)). Just
// like normal math, parentheses indicate that the expression inside must be
// evaluated before it can be used by the surrounding expression. Addition
// still finds the sum of the numbers on both sides of the operator, and
// multiplication still finds the product.
//
// However, the rules of operator precedence have changed. Rather than
// evaluating multiplication before addition, the operators have the same
// precedence, and are evaluated left-to-right regardless of the order in which
// they appear.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are
// as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
//   3   * 3 + 4 * 5 + 6
//       9   + 4 * 5 + 6
//          13   * 5 + 6
//              65   + 6
//                  71
//
// Parentheses can override this order; for example, here is what happens if
// parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
//
// 1 + (2 * 3) + (4 * (5 + 6))
// 1 +    6    + (4 * (5 + 6))
//      7      + (4 * (5 + 6))
//      7      + (4 *   11   )
//      7      +     44
//             51
//
// Here are a few more examples:
//
//     2 * 3 + (4 * 5) becomes 26.
//     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//
// Before you can help with the homework, you need to understand it yourself.
// Evaluate the expression on each line of the homework; what is the sum of the
// resulting values?

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "calculator.pest"]
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
    println!("Advent of Code 2020 Day 18!");

    // let filename = "inputexample.txt";
    let filename = "input.txt";
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
