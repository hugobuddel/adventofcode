extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "cvs.pest"]
pub struct CSVParser;

fn main() {
    println!("Hello, world!");

    let successful_parse = CSVParser::parse(Rule::field, "-273.15");
    println!("{:?}", successful_parse);

    let unsuccessful_parse = CSVParser::parse(Rule::field, "hello");
    println!("{:?}", unsuccessful_parse);


}
