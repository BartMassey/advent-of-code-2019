// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Bart Massey 2019

use aoc::Intcode;

pub fn main() {
    let part = aoc::get_part();
    let mut prog = Intcode::read();
    match part {
        aoc::Part1 => {
            prog.poke(1, 12);
            prog.poke(2, 2);
            prog.run();
            println!("{}", prog.peek(0));
        }
        aoc::Part2 => {
            for noun in 0..100 {
                for verb in 0..100 {
                    let mut prog = prog.clone();
                    prog.poke(1, noun);
                    prog.poke(2, verb);
                    prog.run();
                    let result = prog.peek(0);
                    // It is clearer to use the text pasted
                    // directly from the web than to try to
                    // "format" it. Sorry Clippy.
                    #[allow(clippy::unreadable_literal)]
                    const EXPECTED_RESULT: i64 = 19690720;
                    if result == EXPECTED_RESULT {
                        println!("{}", 100 * noun + verb);
                        return;
                    }
                }
            }
        }
    }
}
