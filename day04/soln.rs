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
            let mut doubled = false;
            for i in 0..ndigits {
                if i >= ndigits - 1 {
                    continue;
                }
                if digits[i] > digits[i+1] {
                    return false;
                }
                let same = || digits[i] == digits[i+1];
                let this_doubled = match part {
                    aoc::Part1 => same(),
                    aoc::Part2 => {
                        let diff_start =
                            || i == 0 || digits[i-1] != digits[i];
                        let diff_end =
                            || i >= ndigits - 2 || digits[i+1] != digits[i+2];
                        same() && diff_start() && diff_end()
                    },
                };
                doubled = doubled || this_doubled;
            }
            doubled
        })
        .count();
    println!("{}", ncands);
}
