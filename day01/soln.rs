// This program is licensed under the "MIT License".
// Please see the file COPYING in this distribution
// for license terms.

// Advent of Code Day 1.
// Bart Massey 2019

fn get_fuel() -> i64 {
    aoc::input_lines()
        .map(|m| m.parse::<i64>().unwrap() / 3 - 2)
        .sum()
}

fn recur_fuel(mut cur_fuel: i64) -> i64 {
    let mut tot_fuel = 0;
    loop {
        cur_fuel = cur_fuel / 3 - 2;
        if cur_fuel <= 0 {
            break;
        }
        tot_fuel += cur_fuel;
    }
    tot_fuel
}

#[test]
fn test_recur_fuel_examples() {
    assert_eq!(2, recur_fuel(14));
    assert_eq!(966, recur_fuel(1969));
    assert_eq!(50346, recur_fuel(100756));
}

fn get_recur_fuel() -> i64 {
    aoc::input_lines()
        .map(|m| recur_fuel(m.parse::<i64>().unwrap()))
        .sum()
}

pub fn main() {
    let part = aoc::get_part();
    let fuel = match part {
        aoc::Part1 => get_fuel(),
        aoc::Part2 => get_recur_fuel(),
    };
    println!("{}", fuel);
}
