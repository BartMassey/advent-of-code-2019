// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 4.  
//! Bart Massey 2019

pub fn main() {
    let part = aoc::get_part();
    let line = aoc::input_line();
    let range: Vec<&str> = line.trim_end().split('-').collect();
    assert_eq!(range.len(), 2);

    let start: usize = range[0].parse().expect("bad range start");
    let end: usize = range[1].parse().expect("bad range end");
    let ncands = (start..=end)
        .filter(|p| {
            let digits: Vec<char> = p.to_string().chars().collect();
            let ndigits = digits.len();
            let mut matched = false;
            for i in 0..ndigits - 1 {
                // No descending digits.
                if digits[i] > digits[i + 1] {
                    return false;
                }
                // Digits must match.
                if digits[i] != digits[i + 1] {
                    continue;
                }
                match part {
                    aoc::Part1 => {
                        matched = true;
                    }
                    aoc::Part2 => {
                        // Match must be delimited in front.
                        if i > 0 && digits[i - 1] == digits[i] {
                            continue;
                        }
                        // Match must be delimited in back.
                        if i == ndigits - 2 {
                            matched = true;
                        }
                        if i < ndigits - 2
                            && digits[i + 1] != digits[i + 2]
                        {
                            matched = true;
                        }
                    }
                }
            }
            matched
        })
        .count();
    println!("{}", ncands);
}
