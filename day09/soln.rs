// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 9.  
//! Bart Massey 2019

pub fn main() {
    let mut prog = aoc::Intcode::read();
    let part = aoc::get_part();
    match part {
        aoc::Part1 => {
            prog.add_input(1);
            while let aoc::Terminus::HaveOutput(q) = prog.run() {
                if q != 0 {
                    println!("{}", q);
                    return;
                }
            }
            panic!("did not produce BOOST code");
        }
        aoc::Part2 => {
            prog.add_input(2);
            if let aoc::Terminus::HaveOutput(q) = prog.run() {
                println!("{}", q);
            } else {
                panic!("did not produce BOOST code");
            }
        }
    }
}
