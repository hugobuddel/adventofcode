//! --- Day 19: Monster Messages ---
//
// You land in an airport surrounded by dense forest. As you walk to your
// high-speed train, the Elves at the Mythical Information Bureau contact you
// again. They think their satellite has collected an image of a sea monster!
// Unfortunately, the connection to the satellite is having problems, and many
// of the messages sent back from the satellite have been corrupted.
//
// They sent you a list of the rules valid messages should obey and a list of
// received messages they've collected so far (your puzzle input).
//
// The rules for valid messages (the top part of your puzzle input) are
// numbered and build upon each other. For example:
//
// 0: 1 2
// 1: "a"
// 2: 1 3 | 3 1
// 3: "b"
//
// Some rules, like 3: "b", simply match a single character (in this case, b).
//
// The remaining rules list the sub-rules that must be followed; for example,
// the rule 0: 1 2 means that to match rule 0, the text being checked must
// match rule 1, and the text after the part that matched rule 1 must then
// match rule 2.
//
// Some of the rules have multiple lists of sub-rules separated by a pipe (|).
// This means that at least one list of sub-rules must match. (The ones that
// match might be different each time the rule is encountered.) For example,
// the rule 2: 1 3 | 3 1 means that to match rule 2, the text being checked
// must match rule 1 followed by rule 3 or it must match rule 3 followed by
// rule 1.
//
// Fortunately, there are no loops in the rules, so the list of possible
// matches will be finite. Since rule 1 matches a and rule 3 matches b, rule 2
// matches either ab or ba. Therefore, rule 0 matches aab or aba.
//
// Here's a more interesting example:
//
// 0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: "a"
// 5: "b"
//
// Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two
// letters that are the same (aa or bb), and rule 3 matches two letters that
// are different (ab or ba).
//
// Since rule 1 matches rules 2 and 3 once each in either order, it must match
// two pairs of letters, one pair with matching letters and one pair with
// different letters. This leaves eight possibilities: aaab, aaba, bbab, bbba,
// abaa, abbb, baaa, or babb.
//
// Rule 0, therefore, matches a (rule 4), then any of the eight options from
// rule 1, then b (rule 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb,
// abaaab, or ababbb.
//
// The received messages (the bottom part of your puzzle input) need to be
// checked against the rules so you can determine which are valid and which are
// corrupted. Including the rules and the messages together, this might look
// like:
//
// 0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: "a"
// 5: "b"
//
// ababbb
// bababa
// abbbab
// aaabbb
// aaaabbb
//
// Your goal is to determine the number of messages that completely match rule
// 0. In the above example, ababbb and abbbab match, but bababa, aaabbb, and
// aaaabbb do not, producing the answer 2. The whole message must match all of
// rule 0; there can't be extra unmatched characters in the message. (For
// example, aaaabbb might appear to match rule 0 above, but it has an extra
// unmatched b on the end.)
//
// How many messages completely match rule 0?

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "monstersimple.pest"]
pub struct MonsterParser;

fn string_from_rule(rule: &Pair<Rule>) -> String {
    // println!("D {:?}", rule);
    match rule.as_rule() {
        Rule::rule => {
            let mut pairrule = rule.clone().into_inner();
            let name = string_from_rule(&pairrule.next().unwrap());
            let right = string_from_rule(&pairrule.next().unwrap());
            match name.as_str() {
                "p0" => {name + &" = { SOI ~ ".to_string() + &right  + &" ~ EOI}".to_string()}
                _ => {name + &" = {".to_string() + &right  + &"}".to_string()}
            }
        }
        Rule::name => {
            "p".to_string() + &rule.as_str().to_string()
        }
        Rule::andterms => {
            let pairrule = rule.clone().into_inner();
            let stringlist: Vec<String> = pairrule.map(|r| string_from_rule(&r)).collect();
            stringlist.join(" ~ ")
        }
        Rule::orterms => {
            let pairrule = rule.clone().into_inner();
            // let stringlist: Vec<String> = pairrule.map(|r| string_from_rule(&r)).collect();
            let stringlist: Vec<String> = pairrule.map(|r| "(".to_string() + &string_from_rule(&r) + &")".to_string()).collect();
            stringlist.join(" | ")
        }
        Rule::letter => {
            rule.as_str().to_string()
        }
        _ => unreachable!(rule)
    }
    // "2".to_string()
}

fn main() {
    println!("Advent of Code 2020 Day 19!");

    let filename = "inputexample.txt";
    // let filename = "input.txt";
    // let filename = "inputupdated.txt";
    // let filename = "inputexample2updated.txt";
    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let monsterfile = MonsterParser::parse(Rule::file, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let rules = monsterfile.into_inner().next().unwrap();
    // println!("Monster: {}", monsterfile.as_str());

    let mut monster: String = fs::read_to_string("src/monstersimple.pest").expect("Error reading monster.");
    monster += &"\n";

    for rule in rules.into_inner() {
        println!("Rule1: {}", rule.as_str());
        let rule2 = string_from_rule(&rule);
        println!("Rule2: {}", rule2);
        monster += &rule2;
        monster += &"\n";
    }

    fs::write("src/monsterfull.pest", monster).expect("Error writing monster.");
}
