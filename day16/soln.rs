// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 16.  
//! Bart Massey 2019

use std::char::from_digit;
use std::convert::TryInto;
use std::iter::repeat;
use std::string::ToString;

#[derive(Debug, Clone)]
struct Digits(Vec<i64>);

impl Digits {
    fn new(input: &str) -> Self {
        let digits = input
            .chars()
            .map(|c| i64::from(c.to_digit(10).expect("non-digit")))
            .collect();
        Self(digits)
    }

    fn make_patterns(&self) -> Vec<Self> {
        let n = self.0.len();
        let pat = [0, 1, 0, -1];
        let make_pat = |i| {
            let pi = pat
                .iter()
                .cloned()
                .flat_map(|p| repeat(p).take(i + 1))
                .cycle()
                .skip(1)
                .take(n)
                .collect();
            Self(pi)
        };
        (0..n).map(make_pat).collect()
    }

    fn sum(&self, phases: usize) -> Self {
        let ninput = self.0.len();
        let patterns = self.make_patterns();
        let mut input = self.0.clone();
        for _ in 0..phases {
            let mut next = Vec::with_capacity(ninput);
            for pat in &patterns {
                let pat = &pat.0;
                let digit: i64 = input
                    .iter()
                    .cloned()
                    .zip(pat.iter().cloned())
                    .map(|(q, r)| q * r)
                    .sum();
                next.push(digit.abs() % 10);
            }
            input = next;
        }
        Self(input)
    }
}

impl ToString for Digits {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .cloned()
            .map(|d| {
                from_digit(d.try_into().unwrap(), 10)
                    .expect("non-digit")
            })
            .collect()
    }
}

#[test]
fn test_sum() {
    let digits = Digits::new("12345678");
    let sum = digits.sum(1);
    assert_eq!(sum.to_string(), "48226158");
    let sum = digits.sum(4);
    assert_eq!(sum.to_string(), "01029498");

    let digits = Digits::new("80871224585914546619083218645595");
    let sum = digits.sum(100).to_string();
    assert_eq!(&sum[0..8], "24176176");
}

pub fn main() {
    let input = aoc::input_line();
    let digits = Digits::new(&input);
    let part = aoc::get_part();
    assert!(part == aoc::Part1);
    let sum = digits.sum(100).to_string();
    println!("{}", &sum[0..8]);
}
