//! --- Day 8: Handheld Halting ---
//
// Your flight to the major airline hub reaches cruising altitude without
// incident. While you consider checking the in-flight menu for one of those
// drinks that come with a little umbrella, you are interrupted by the kid
// sitting next to you.
//
// Their handheld game console won't turn on! They ask if you can take a look.
//
// You narrow the problem down to a strange infinite loop in the boot code
// (your puzzle input) of the device. You should be able to fix it, but first
// you need to be able to run the code in isolation.
//
// The boot code is represented as a text file with one instruction per line of
// text. Each instruction consists of an operation (acc, jmp, or nop) and an
// argument (a signed number like +4 or -20).
//
//     acc increases or decreases a single global value called the accumulator
//         by the value given in the argument. For example, acc +7 would
//         increase the accumulator by 7. The accumulator starts at 0. After an
//         acc instruction, the instruction immediately below it is executed
//         next.
//     jmp jumps to a new instruction relative to itself. The next instruction
//         to execute is found using the argument as an offset from the jmp
//         instruction; for example, jmp +2 would skip the next instruction,
//         jmp +1 would continue to the instruction immediately below it, and
//         jmp -20 would cause the instruction 20 lines above to be executed
//         next.
//     nop stands for No OPeration - it does nothing. The instruction
//         immediately below it is executed next.
//
// For example, consider the following program:
//
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
//
// These instructions are visited in this order:
//
// nop +0  | 1
// acc +1  | 2, 8(!)
// jmp +4  | 3
// acc +3  | 6
// jmp -3  | 7
// acc -99 |
// acc +1  | 4
// jmp -4  | 5
// acc +6  |
//
// First, the nop +0 does nothing. Then, the accumulator is increased from 0 to
// 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the
// bottom. After it increases the accumulator from 1 to 2, jmp -4 executes,
// setting the next instruction to the only acc +3. It sets the accumulator to
// 5, and jmp -3 causes the program to continue back at the first acc +1.
//
// This is an infinite loop: with this sequence of jumps, the program will run
// forever. The moment the program tries to run any instruction a second time,
// you know it will never terminate.
//
// Immediately before the program would run an instruction a second time, the
// value in the accumulator is 5.
//
// Run your copy of the boot code. Immediately before any instruction is
// executed a second time, what value is in the accumulator?

// --- Part Two ---
//
// After some careful analysis, you believe that exactly one instruction is
// corrupted.
//
// Somewhere in the program, either a jmp is supposed to be a nop, or a nop is
// supposed to be a jmp. (No acc instructions were harmed in the corruption of
// this boot code.)
//
// The program is supposed to terminate by attempting to execute an instruction
// immediately after the last instruction in the file. By changing exactly one
// jmp or nop, you can repair the boot code and make it terminate correctly.
//
// For example, consider the same program from above:
//
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
//
// If you change the first instruction from nop +0 to jmp +0, it would create a
// single-instruction infinite loop, never leaving that instruction. If you
// change almost any of the jmp instructions, the program will still eventually
// find another jmp instruction and loop forever.
//
// However, if you change the second-to-last instruction (from jmp -4 to nop
// -4), the program terminates! The instructions are visited in this order:
//
// nop +0  | 1
// acc +1  | 2
// jmp +4  | 3
// acc +3  |
// jmp -3  |
// acc -99 |
// acc +1  | 4
// nop -4  | 5
// acc +6  | 6
//
// After the last instruction (acc +6), the program terminates by attempting to
// run the instruction below the last instruction in the file. With this change,
// after the program terminates, the accumulator contains the value 8 (acc +1,
// acc +1, acc +6).
//
// Fix the program so that it terminates normally by changing exactly one jmp
// (to nop) or nop (to jmp). What is the value of the accumulator after the
// program terminates?

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashMap;

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Instruction {
    name: String,
    toadd: i32,
    done: bool,
}

impl Instruction {
    fn from_str(x: &str) -> Instruction {
        let name_toadd: Vec<&str> = x.splitn(2, " ").collect();
        let name = name_toadd[0].to_string();
        let toadd = name_toadd[1].parse::<i32>().unwrap();
        Instruction {
            name: name,
            toadd: toadd,
            done: false,
        }
    }
}

#[derive(Debug)]
struct Machine {
    instructions: Vec<Instruction>,
    accumulator: i32,
    current: usize,
}

impl Machine {
    fn from_filename(filename: &str) -> Machine {
        let mut instructions: Vec<Instruction> = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            println!("{:?}", lines);
            for line in lines {
                println!("{:?}", line);
                let line2 = line.unwrap();
                let instruction = Instruction::from_str(line2.as_str());
                println!("{:?}", instruction);
                instructions.push(instruction);
            }
        }
        Machine {
            instructions: instructions,
            accumulator: 0,
            current: 0,
        }
    }

    fn step(&mut self) {
        // let  instruction = self.instructions[self.current];
        println!("Performing: {} {:?}", self.accumulator, self.instructions[self.current]);
        self.instructions[self.current].done = true;
        // instruction.done = true;
        // match instruction.name.as_str() {
        match self.instructions[self.current].name.as_str() {
            "nop" => {
                self.current += 1;
            }
            "acc" => {
                self.accumulator += self.instructions[self.current].toadd;
                self.current += 1;
            }
            "jmp" => {
                self.current = (self.current as i32 + self.instructions[self.current].toadd) as usize;
            }
            _ => {
                println!("Unknown instruction!");
            }
        }
    }
}

fn main() {
    println!("Advent of Code 2020 Day 8!");

    // let filename = "inputexample.txt";
    let filename = "input.txt";
    let mut machine = Machine::from_filename(filename);
    while ! machine.instructions[machine.current].done {
        machine.step();
    }
}
