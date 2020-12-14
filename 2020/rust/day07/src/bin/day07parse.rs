//! --- Day 7: Handy Haversacks ---
//
// You land at the regional airport in time for your next flight. In fact, it
// looks like you'll even have time to grab some food: all flights are
// currently delayed due to issues in luggage processing.
//
// Due to recent aviation regulations, many rules (your puzzle input) are being
// enforced about bags and their contents; bags must be color-coded and must
// contain specific quantities of other color-coded bags. Apparently, nobody
// responsible for these regulations considered how long they would take to
// enforce!
//
// For example, consider the following rules:
//
// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.
//
// These rules specify the required contents for 9 bag types. In this example,
// every faded blue bag is empty, every vibrant plum bag contains 11 bags (5
// faded blue and 6 dotted black), and so on.
//
// You have a shiny gold bag. If you wanted to carry it in at least one other
// bag, how many different bag colors would be valid for the outermost bag? (In
// other words: how many colors can, eventually, contain at least one shiny
// gold bag?)
//
// In the above rules, the following options would be available to you:
//
//     A bright white bag, which can hold your shiny gold bag directly.
//     A muted yellow bag, which can hold your shiny gold bag directly, plus
//         some other bags.
//     A dark orange bag, which can hold bright white and muted yellow bags,
//         either of which could then hold your shiny gold bag.
//     A light red bag, which can hold bright white and muted yellow bags,
//         either of which could then hold your shiny gold bag.
//
// So, in this example, the number of bag colors that can eventually contain at
// least one shiny gold bag is 4.
//
// How many bag colors can eventually contain at least one shiny gold bag? (The
// list of rules is quite long; make sure you get all of it.)

// --- Part Two ---
//
// It's getting pretty expensive to fly these days - not because of ticket
// prices, but because of the ridiculous number of bags you need to buy!
//
// Consider again your shiny gold bag and the rules from the above example:
//
//     faded blue bags contain 0 other bags.
//     dotted black bags contain 0 other bags.
//     vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted
//         black bags.
//     dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted
//         black bags.
//
// So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags
// within it) plus 2 vibrant plum bags (and the 11 bags within each of those):
// 1 + 1*7 + 2 + 2*11 = 32 bags!
//
// Of course, the actual rules have a small chance of going several levels
// deeper than this example; be sure to count all of the bags, even if the
// nesting becomes topologically impractical!
//
// Here's another example:
//
// shiny gold bags contain 2 dark red bags.
// dark red bags contain 2 dark orange bags.
// dark orange bags contain 2 dark yellow bags.
// dark yellow bags contain 2 dark green bags.
// dark green bags contain 2 dark blue bags.
// dark blue bags contain 2 dark violet bags.
// dark violet bags contain no other bags.
//
// In this example, a single shiny gold bag must contain 126 other bags.
//
// How many individual bags are required inside your single shiny gold bag?

use std::fs;
use petgraph::Graph;
use petgraph::algo;
use std::collections::HashMap;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "bags.pest"]
pub struct BagsParser;

fn count_contents(deps: &Graph::<&str, &str>, node:petgraph::prelude::NodeIndex) -> i32 {
    // println!("Getting contents of bag {:?}", node);
    let mut count_bags = 0;
    for neighbor in deps.neighbors_directed(node, petgraph::Direction::Outgoing) {
        // Add the contents of neighbor.
        count_bags += count_contents(deps, neighbor);
        // Add neighbor itself.
        count_bags += 1;
    }
    count_bags
}

fn main () {
    println!("Advent of Code 2020 Day 7.");

    // let filename = "inputexample.txt";
    let filename = "input.txt";

    let unparsed_file = fs::read_to_string(filename).expect("Error reading file.");

    let bagfile = BagsParser::parse(Rule::bags, &unparsed_file)
        .expect("Unsuccessful parse")
        .next().unwrap();

    let mut bagrules: Vec<(String, Vec<(i32, String)>)> = Vec::new();

    for line in bagfile.into_inner() {
        if line.as_rule() == Rule::EOI {
            break;
        }
        let mut pair = line.into_inner();
        let name = pair.next().unwrap().as_str();
        let mut contains: Vec<(i32, String)> = Vec::new();
        let rules = pair.next().unwrap();
        if rules.as_rule() == Rule::bagrules {
            for rule in rules.into_inner() {
                let mut countname = rule.into_inner();
                let count = countname.next().unwrap().as_str().parse::<i32>().unwrap();
                let bagtype = countname.next().unwrap().as_str();
                contains.push((count, bagtype.to_string()));
            }
        }
        bagrules.push((name.to_string(), contains.clone()));
    }

    let mut deps = Graph::<&str, &str>::new();
    let bagrules2 = bagrules.clone();
    let nodes = bagrules2.iter().map(|n_cc| (n_cc.0.clone(), deps.add_node(n_cc.0.as_str()))).collect::<HashMap<_, _>>();
    for (name, rule) in bagrules {
        for (count, name2) in rule {
            // Add the edge count number of times.
            // TODO: figure out how to use edge weights instead.
            for _counti in 0..count {
                deps.add_edge(nodes[&name], nodes[&name2], "1");
            }
        }
    }

    let nodes_to_gold = nodes.iter().filter(
        |bag| algo::has_path_connecting(&deps, *bag.1, nodes["shiny gold"], None)
    ).collect::<Vec<_>>();
    // -1 because there is a path from a node to itself.
    let nr_bags_with_gold = nodes_to_gold.len() - 1;
    println!("Number of bags that can contain a shiny gold bag: {}", nr_bags_with_gold);
    // assert_eq!(nr_bags_with_gold, 4);

    let bags_in_gold = count_contents(&deps, nodes["shiny gold"]);
    println!("Number of bags in gold bag: {}", bags_in_gold);
    // assert_eq!(bags_in_gold, 32);
}
