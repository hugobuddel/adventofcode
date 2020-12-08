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

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use petgraph::Graph;
use petgraph::algo;
use std::collections::HashMap;

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main () {
    println!("Advent of Code 2020 Day 7.");

    let mut bagrules: Vec<(String, Vec<(i32, String)>)> = Vec::new();

    if let Ok(lines) = read_lines("./inputexample.txt") {
        for line in lines {
            // println!("{:?}", line.unwrap());
            let lineu = line.unwrap();
            let rr = lineu.split(" bags contain ").collect::<Vec<_>>();
            let name = rr[0];
            let rest = rr[1];
            let mut contains: Vec<(i32, String)> = Vec::new();
            if rest != "no other bags." {
                let rest_split = rest.replace(".", " ");
                let rest_split2 = rest_split.trim();
                let rest_split3 = rest_split2.split(", ");
                for count_type in rest_split3 {
                    // println!("{:?}", count_type);
                    let count_type_split: Vec<&str> = count_type.splitn(2, " ").collect();
                    // println!("{:?}", count_type_split);
                    let count = count_type_split[0].parse::<i32>().unwrap();
                    let bagtype = count_type_split[1].rsplitn(2, " ").collect::<Vec<_>>()[1];
                    let c_bt = (count, bagtype.to_string());
                    // println!("{:?}", c_bt);
                    contains.push(c_bt);
                }
            }
            bagrules.push((name.to_string(), contains.clone()));
            println!("{} {:?}", name, contains);
        }
    }

    let mut deps = Graph::<&str, &str>::new();
    let bagrules2 = bagrules.clone();
    let nodes = bagrules2.iter().map(|n_cc| (n_cc.0.clone(), deps.add_node(n_cc.0.as_str()))).collect::<HashMap<_, _>>();
    deps.add_edge(nodes["bright white"], nodes["shiny gold"], "1");
    for (name, rule) in bagrules {
        for (_count, name2) in rule {
            deps.add_edge(nodes[&name], nodes[&name2], "1");
        }
    }

    println!("{:?}", deps);

    let p1 = algo::has_path_connecting(&deps, nodes["shiny gold"], nodes["dark orange"], None);
    let p2 = algo::has_path_connecting(&deps, nodes["dark orange"], nodes["shiny gold"], None);
    let p3 = algo::has_path_connecting(&deps, nodes["shiny gold"], nodes["shiny gold"], None);
    println!("Testpaths {} {} {}", p1, p2, p3);

    let nodes_to_gold = nodes.iter().filter(
        |bag| algo::has_path_connecting(&deps, *bag.1, nodes["shiny gold"], None)
    ).collect::<Vec<_>>();
    // -1 because there is a path from a node to itself.
    let nr_bags_with_gold = nodes_to_gold.len() - 1;
    println!("Number of bags that can contain a shiny gold bag: {}", nr_bags_with_gold);
    assert_eq!(nr_bags_with_gold, 4);
}
