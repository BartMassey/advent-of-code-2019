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
    prog.run();
    let outputs = prog.view_outputs();
    match part {
        Part1 => {
            assert_eq!(&outputs[..9], &[0, 0, 0, 0, 0, 0, 0, 0, 0]);
            println!("{}", outputs[9]);
        }
        Part2 => {
            assert_eq!(outputs.len(), 1);
            println!("{}", outputs[0]);
        }
    }
}
