#![allow(unused)]
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 12.  
//! Bart Massey 2019

use lazy_static::lazy_static;
use regex::Regex;

use aoc::sgn;

lazy_static! {
    /// Regular expression used for parsing the input.
    static ref POSN_RE: Regex =
        Regex::new("<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>")
            .unwrap();
}

/// A 3D point.
type Point3 = [i64; 3];

/// Physical state of a planet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    /// Planet position.
    posn: Point3,
    /// Planet velocity.
    vel: Point3,
}

/// Turn a collection of points into initial states by
/// giving them zero velocity.
fn make_states(posns: &[Point3]) -> Vec<State> {
    posns
        .iter()
        .map(|&p| State {
            posn: p,
            vel: [0, 0, 0],
        })
        .collect()
}

/// Simulation step.
fn step(states: &mut [State]) {
    // Adjust velocities.
    let nstates = states.len();
    for s in 0..nstates {
        for t in 0..nstates {
            for i in 0..3 {
                let ps = states[s].posn[i];
                let pt = states[t].posn[i];
                states[s].vel[i] += sgn(pt - ps);
            }
        }
    }

    // Adjust positions.
    for s in states {
        for i in 0..3 {
            s.posn[i] += s.vel[i];
        }
    }
}

/// Run the simulation forward a given number of steps.
fn sim(steps: usize, posns: &[Point3]) -> Vec<State> {
    let mut states = make_states(posns);
    for _ in 0..steps {
        step(&mut states);
    }
    states
}

/// This is the LCM of *n* numbers.
// XXX This should be in `libaoc`, not here.
fn reduce(ps: &[i64]) -> i64 {
    ps[1..].iter().fold(ps[0], |a, &x| aoc::lcm(a, x))
}

/// Length of a planetary cycle.
fn cycle_len(posns: &[Point3]) -> i64 {
    assert!(!posns.is_empty());
    let states0 = make_states(posns);

    // Compute the cycle on each coordinate separately, then
    // combine them at the end. This is much faster.
    let mut components = Vec::with_capacity(3);
    for j in 0..3 {
        let mut states = states0.clone();
        let mut k = 0;
        let ncycle = 'searching: loop {
            step(&mut states);
            k += 1;
            for i in 0..states.len() {
                if states[i].posn[j] != states0[i].posn[j]
                    || states[i].vel[j] != states0[i].vel[j]
                {
                    continue 'searching;
                }
            }
            break k;
        };
        components.push(ncycle);
    }

    // The overall cycle is the LCM of the components.
    reduce(&components)
}

/// The "energy" of a planet.
fn energy(state: &State) -> i64 {
    fn e(p: &Point3) -> i64 {
        p.iter().map(|&c| c.abs()).sum()
    }

    let pot = e(&state.posn);
    let kin = e(&state.vel);
    pot * kin
}

/// The total "energy" of a simulation state.
fn total_energy(states: &[State]) -> i64 {
    states.iter().map(|s| energy(s)).sum()
}

/// Parse a position string in the problem input format.
fn parse_posn(posn: &str) -> Point3 {
    let fields = POSN_RE.captures(posn).expect("invalid posn");
    let mut result = [0; 3];
    for (i, c) in result.iter_mut().enumerate() {
        *c = fields
            .get(i + 1)
            .expect("invalid match")
            .as_str()
            .parse()
            .expect("invalid coordinate");
    }
    result
}

/// Collect the full problem input.
fn read_input() -> Vec<Point3> {
    aoc::input_lines().map(|p| parse_posn(&p)).collect()
}

// Test cases from problem description.
#[test]
pub fn test_problem_cases() {
    // Simplest example.
    #[rustfmt::skip]
    let posns = &[
        [-1, 0, 2],
        [2, -10, -7],
        [4, -8, 8],
        [3, 5, -1],
    ];

    // Check a single simulation step.
    #[rustfmt::skip]
    let expected: &[State] = &[
        State {
            posn: [2, -1, 1],
            vel: [3, -1, -1],
        },
        State {
            posn: [3, -7, -4],
            vel: [1, 3, 3],
        },
        State {
            posn: [1, -7, 5],
            vel: [-3, 1, -3],
        },
        State {
            posn: [2, 2, 0],
            vel: [-1, -3, 1],
        },
    ];
    let states = sim(1, posns);
    assert_eq!(expected.len(), states.len());
    for i in 0..expected.len() {
        assert_eq!(expected[i], states[i]);
    }

    // Check energy after 10 simulation steps.
    let states = sim(10, posns);
    assert_eq!(179, total_energy(&states));

    // Check cycle length directly and via
    // the function for it.
    let states0 = sim(0, posns);
    let states_final = sim(2772, posns);
    assert_eq!(states0, states_final);
    assert_eq!(2772, cycle_len(posns));

    // Check the long-cycle example.
    #[rustfmt::skip]
    let posns = &[
        "<x=-8, y=-10, z=0>",
        "<x=5, y=5, z=10>",
        "<x=2, y=-7, z=3>",
        "<x=9, y=-8, z=-3>",
    ];
    let posns: Vec<Point3> =
        posns.iter().map(|p| parse_posn(p)).collect();
    assert_eq!(4686774924, cycle_len(&posns));
}

pub fn main() {
    let posns = read_input();
    let part = aoc::get_part();
    match part {
        aoc::Part1 => {
            let states = sim(1000, &posns);
            let energy = total_energy(&states);
            println!("{}", energy);
        }
        aoc::Part2 => {
            let n = cycle_len(&posns);
            println!("{}", n);
        }
    }
}
