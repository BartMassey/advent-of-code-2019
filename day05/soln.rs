// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 5.  
//! Bart Massey 2019

use aoc::Terminus::*;

pub fn main() {
    let mut prog = aoc::Intcode::read();
    let part = aoc::get_part();
    match part {
        aoc::Part1 => {
            prog.add_input(1);
            while let HaveOutput(q) = prog.run() {
                if q != 0 {
                    println!("{}", q);
                    return;
                }
            }
            panic!("program did not produce nonzero code");
        }
        aoc::Part2 => {
            prog.add_input(5);
            match prog.run() {
                HaveOutput(q) => println!("{}", q),
                result => {
                    panic!("unexpected program result {:?}", result)
                }
            }
        }
    }
}
