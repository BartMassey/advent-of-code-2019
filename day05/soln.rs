// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 5.  
//! Bart Massey 2019

use aoc::Part::*;

pub fn main() {
    let part = aoc::get_part();
    let input = match part {
        Part1 => 1,
        Part2 => 5,
    };
    let mut prog = aoc::Intcode::read().with_inputs(vec![input]);
    let outputs = prog.collect_outputs();
    let last = outputs.len() - 1;
    match part {
        Part1 => {
            for q in &outputs[..last] {
                assert_eq!(*q, 0);
            }
            println!("{}", outputs[last]);
        }
        Part2 => {
            println!("{:?}", outputs[last]);
        }
    }
}
