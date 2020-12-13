extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "cvs.pest"]
pub struct CSVParser;

fn main() {
    println!("Hello, world!");
}
