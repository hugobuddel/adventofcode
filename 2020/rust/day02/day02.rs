// All warnings enabled, to learn what they do.
#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    clashing_extern_declarations,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unreachable_pub,
    unsafe_code,
    // unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
)]

//! Advent of Code 2020 Day 2
// --- Day 2: Password Philosophy ---
//
// Your flight departs in a few days from the coastal airport; the easiest way
// down to the coast from here is via toboggan.
//
// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
// "Something's wrong with our computers; we can't log in!" You ask if you can
// take a look.
//
// Their password database seems to be a little corrupted: some of the
// passwords wouldn't have been allowed by the Official Toboggan Corporate
// Policy that was in effect when they were chosen.
//
// To try to debug the problem, they have created a list (your puzzle input) of
// passwords (according to the corrupted database) and the corporate policy
// when that password was set.
//
// For example, suppose you have the following list:
//
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
//
// Each line gives the password policy and then the password. The password
// policy indicates the lowest and highest number of times a given letter must
// appear for the password to be valid. For example, 1-3 a means that the
// password must contain a at least 1 time and at most 3 times.
//
// In the above example, 2 passwords are valid. The middle password, cdefg, is
// not; it contains no instances of b, but needs at least 1. The first and
// third passwords are valid: they contain one a or nine c, both within the
// limits of their respective policies.
//
// How many passwords are valid according to their policies?

// --- Part Two ---
//
// While it appears you validated the passwords correctly, they don't seem to
// be what the Official Toboggan Corporate Authentication System is expecting.
//
// The shopkeeper suddenly realizes that he just accidentally explained the
// password policy rules from his old job at the sled rental place down the
// street! The Official Toboggan Corporate Policy actually works a little
// differently.
//
// Each policy actually describes two positions in the password, where 1 means
// the first character, 2 means the second character, and so on. (Be careful;
// Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of
// these positions must contain the given letter. Other occurrences of the
// letter are irrelevant for the purposes of policy enforcement.
//
// Given the same example list from above:
//
//     1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//     1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//     2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//
// How many passwords are valid according to the new interpretation of the
// policies?

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Password {
    minimum: usize,
    maximum: usize,
    letter: char,
    password: String,
}

impl Password {
    fn validate(&self) -> bool {
        let the_count = self.password.chars().filter(|x| *x == self.letter).count();
        // println!("The count {}", the_count);
        (self.minimum <= the_count) & (the_count <= self.maximum)
    }

    // Either the character at position 'minimum' or at position 'maximum'
    // must be 'letter'. 1-based indexing.
    fn validate_toboggan(&self) -> bool {
        // println!("Testing {:?}", self);
        (self.password.chars().nth(self.minimum - 1).unwrap() == self.letter) ^
            (self.password.chars().nth(self.maximum - 1).unwrap() == self.letter)
    }

    fn from_str(x: &str) -> Password {
        let abc1 = x.split(" ").collect::<Vec<&str>>();
        let abc2 = (
            abc1[0].split("-").map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
            abc1[1].trim_end_matches(":"),
            abc1[2],
        );
        Password {
            minimum: abc2.0[0],
            maximum: abc2.0[1],
            letter: abc2.1.chars().nth(0).unwrap(),
            password: abc2.2.to_string(),
        }
    }
}

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    println!("Advent of Code 2020 Day 2");

    let entries1 = vec![
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc",
    ];
    // println!("{:?}", entries1);
    let entries4 = entries1.iter().map(
        |x| Password::from_str(x)
    ).collect::<Vec<_>>();
    // println!("{:?}", entries4);

    for entry in entries4 {
        if ! entry.validate() {
            println!("Bad entry: {:?}", entry);
        }
    }

    // let file = File::open("./input").expect("Cannot read file.");
    // let entries5 = io::BufReader::new(file);
    if let Ok(entries5) = read_lines("./input") {
        for entry in entries5 {
            let entry2 = entry.unwrap();
            let pw = Password::from_str(&entry2);
            if ! pw.validate() {
                // println!("Bad: {:?}", entry2);
            }
        }
    }

    if let Ok(entries6) = read_lines("./input") {
        let entries7 = entries6.map(|x| Password::from_str(&x.unwrap()));
        let good_entries = entries7.filter(|x| x.validate());
        println!("Number of good entries: {}", good_entries.collect::<Vec<_>>().len());
    }

    if let Ok(entries6) = read_lines("./input") {
        let entries7 = entries6.map(|x| Password::from_str(&x.unwrap()));
        let bad_entries = entries7.filter(|x| ! x.validate());
        println!("Number of bad entries: {}", bad_entries.collect::<Vec<_>>().len());
    }

    if let Ok(entries6) = read_lines("./input") {
        let entries7 = entries6.map(|x| Password::from_str(&x.unwrap()));
        let good_entries = entries7.filter(|x| x.validate_toboggan());
        println!("Number of good entries tobbogan: {}", good_entries.collect::<Vec<_>>().len());
    }

}
