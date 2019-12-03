// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 1.  
//! Bart Massey 2019

/// Read the masses from stdin and return an iterator over
/// them. We use `i64` because it is sufficiently large and
/// the calculation in `calc_fuel_recur()` may go negative.
fn get_masses() -> impl Iterator<Item = i64> {
    aoc::input_lines().map(|m| m.parse::<i64>().unwrap())
}

/// Calculate the total fuel consumed by the given masses.
fn calc_total_fuel(masses: impl Iterator<Item = i64>) -> i64 {
    masses.map(|m| m / 3 - 2).sum()
}

/// Do the recurrent fuel calculation for a single mass.
fn calc_fuel_recur(mut cur_fuel: i64) -> i64 {
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
fn test_calc_fuel_recur() {
    assert_eq!(2, calc_fuel_recur(14));
    assert_eq!(966, calc_fuel_recur(1969));
    assert_eq!(50346, calc_fuel_recur(100756));
}

/// Calculate the total recurrent fuel cost for the given
/// masses.
fn calc_total_fuel_recur(masses: impl Iterator<Item = i64>) -> i64 {
    masses.map(calc_fuel_recur).sum()
}

/// Run the problem.
pub fn main() {
    let part = aoc::get_part();
    let masses = get_masses();
    let fuel = match part {
        aoc::Part1 => calc_total_fuel(masses),
        aoc::Part2 => calc_total_fuel_recur(masses),
    };
    println!("{}", fuel);
}
