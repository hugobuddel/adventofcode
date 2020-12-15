//! --- Day 14: Docking Data ---
// --- Part Two ---
//
// For some reason, the sea port's computer system still can't communicate with
// your ferry's docking program. It must be using version 2 of the decoder
// chip!
//
// A version 2 decoder chip doesn't modify the values being written at all.
// Instead, it acts as a memory address decoder. Immediately before a value is
// written to memory, each bit in the bitmask modifies the corresponding bit of
// the destination memory address in the following way:
//
//     If the bitmask bit is 0, the corresponding memory address bit is unchanged.
//     If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
//     If the bitmask bit is X, the corresponding memory address bit is floating.
//
// A floating bit is not connected to anything and instead fluctuates
// unpredictably. In practice, this means the floating bits will take on all
// possible values, potentially causing many memory addresses to be written
// all at once!
//
// For example, consider the following program:
//
// mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1
//
// When this program goes to write to memory address 42, it first applies the
// bitmask:
//
// address: 000000000000000000000000000000101010  (decimal 42)
// mask:    000000000000000000000000000000X1001X
// result:  000000000000000000000000000000X1101X
//
// After applying the mask, four bits are overwritten, three of which are
// different, and two of which are floating. Floating bits take on every
// possible combination of values; with two floating bits, four actual memory
// addresses are written:
//
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// 000000000000000000000000000000111010  (decimal 58)
// 000000000000000000000000000000111011  (decimal 59)
//
// Next, the program is about to write to memory address 26 with a different
// bitmask:
//
// address: 000000000000000000000000000000011010  (decimal 26)
// mask:    00000000000000000000000000000000X0XX
// result:  00000000000000000000000000000001X0XX
//
// This results in an address with three floating bits, causing writes to eight
// memory addresses:
//
// 000000000000000000000000000000010000  (decimal 16)
// 000000000000000000000000000000010001  (decimal 17)
// 000000000000000000000000000000010010  (decimal 18)
// 000000000000000000000000000000010011  (decimal 19)
// 000000000000000000000000000000011000  (decimal 24)
// 000000000000000000000000000000011001  (decimal 25)
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
//
// The entire 36-bit address space still begins initialized to the value 0 at
// every address, and you still need the sum of all values left in memory at
// the end of the program. In this example, the sum is 208.
//
// Execute the initialization program using an emulator for a version 2 decoder
// chip. What is the sum of all values left in memory after it completes?

use std::fs;
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "program.pest"]
pub struct ProgramParser;

// https://stackoverflow.com/questions/40718975/how-to-get-every-subset-of-a-vector-in-rust
fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                             .map(|(_, element)| element.clone())
                             .collect()
     }).collect()
}

fn main() {
    println!("Advent of Code 2020 Day 14!");
    // let filename = "inputexample2.txt";
    let filename = "input.txt";
    let mut maskor: usize = 0;
    let mut maskand: usize = 0;
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let file_unparsed = fs::read_to_string(filename).expect("Error reading file.");
    let pprogram = ProgramParser::parse(Rule::program, &file_unparsed)
        .expect("Error parsing program.").next().unwrap();
    let mut count_floats: usize = 0; // 2 ** floaning bits in mask
    let mut count_mem: usize = 0;
    let mut count_mem_mask: usize = 0;
    let mut floating_values: Vec<usize> = Vec::new();
    for command in pprogram.into_inner() {
        match command.as_rule() {
            Rule::commandmask => {
                // add the previous counted values to the total
                count_mem += count_floats * count_mem_mask;
                // reset the counter
                count_mem_mask = 0;
                let maskstring = command.into_inner().next().unwrap().as_str();
                count_floats = 2_usize.pow(maskstring.chars().filter(|x| *x == 'X').count() as u32);
                println!("");
                println!("Mask: {} {}", maskstring, count_floats);
                maskor = maskstring.chars().rev()
                    .enumerate().filter(|x| x.1 == '1')
                    .map(|x| 2_usize.pow(x.0 as u32))
                    .sum();
                maskand = maskstring.chars().rev()
                    .enumerate().filter(|x| x.1 != 'X')
                    .map(|x| 2_usize.pow(x.0 as u32))
                    .sum();
                floating_values = maskstring.chars().rev()
                    .enumerate().filter(|x| x.1 == 'X')
                    .map(|x| 2_usize.pow(x.0 as u32))
                    .collect();
                println!("maskor:{} maskand:{} fv:{:?}", maskor, maskand, floating_values);
            }
            Rule::commandmem => {
                count_mem_mask += 1;
                let mut pair = command.into_inner();
                let location: usize = pair.next().unwrap().as_str().parse().unwrap();
                let value: usize = pair.next().unwrap().as_str().parse().unwrap();
                let locationmasked = (location | maskor) & maskand;
                println!("location:{} value:{} locationmasked:{}", location, value, locationmasked);
                for pset in powerset(&floating_values) {
                    let location_floated = locationmasked + pset.iter().sum::<usize>();
                    println!("Setting {} to {}", location_floated, value);
                    memory.insert(location_floated, value);
                }
            }
            _ => unreachable!()
        }
    }
    println!("");
    // println!("Memory: {:?}", memory);
    let total: usize = memory.iter().map(|x| x.1).sum();
    println!("Total: {}", total);
    // We're missing the last mask, doesn't matter.
    println!("Max memory requirements: {}", count_mem);
    // assert_eq!(165, total);
}
