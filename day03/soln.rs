// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 3.  
//! Bart Massey 2019

use std::collections::{HashMap, HashSet};

/// We use some machinery from `libaoc` to deal with
/// geometry.
use aoc::{
    self,
    Dirn::{self, *},
    Point,
};

/// A wire segment, with distance and direction.
#[derive(Debug, Clone)]
struct Segment {
    dirn: Dirn,
    dist: i64,
}

/// A wire. This is returned by a lot of the functions
/// below, so seemed worth its own type.
type Wire = Vec<Segment>;

/// Given a strings of some kind representing a wire, return
/// the parsed wire.
fn parse_wire<T: AsRef<str>>(desc: T) -> Wire {
    desc.as_ref()
        .trim_end()
        .split(',')
        .map(|s| {
            let (dirn, dist) = s.split_at(1);
            let dirn = match dirn {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => panic!("unknown direction {}", dirn),
            };
            let dist = dist.parse().expect("unknown dist");
            Segment { dirn, dist }
        })
        .collect()
}

/// Read wire descriptions from standard input and arrange
/// for them to be parsed.
fn read_wires() -> Vec<Wire> {
    let wires: Vec<Wire> = aoc::input_lines().map(parse_wire).collect();
    if wires.len() != 2 {
        panic!("unexpected number {} of wires", wires.len());
    }
    wires
}

/// A tracepoint consists of a point and a distance along a
/// wire to reach that point.
type Tracepoint = (Point, u64);

/// Given a wire, return successive tracepoints representing
/// minimum wire distances to each point. This is not lazy,
/// although it could be: it's easier to read when returning
/// the iterator at the end, and it's never used in a
/// context where laziness could matter.
fn trace_wire(wire: &[Segment]) -> impl Iterator<Item = Tracepoint> {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    let mut posns = Vec::with_capacity(wire.len());
    let mut seen: HashSet<Point> = HashSet::new();
    for s in wire {
        let (dx, dy) = s.dirn.disp();
        for _ in 1..=s.dist {
            x += dx;
            y += dy;
            steps += 1;
            if seen.insert((x, y)) {
                posns.push(((x, y), steps));
            }
        }
    }
    posns.into_iter()
}

// Try a really simple wire to make sure things are good.
#[test]
fn test_trace_wire() {
    let wire = &[
        Segment {
            dirn: Down,
            dist: 2,
        },
        Segment {
            dirn: Right,
            dist: 1,
        },
    ];
    let expected = vec![((0, 1), 1), ((0, 2), 2), ((1, 2), 3)];

    let actual: Vec<Tracepoint> = trace_wire(wire).collect();
    assert_eq!(expected, actual);
}

/// The set of points forming the intersection of two wires.
/// Note that this code really only works for two wires; the
/// extension to multiple wire pairs is easy, but the
/// complexity grows as O(n^2) and that generality is not
/// needed for this problem.
fn intersect_wires(wires: &[Wire]) -> HashSet<Point> {
    assert_eq!(wires.len(), 2);
    let maps: Vec<HashSet<Point>> = wires
        .iter()
        .map(|w| trace_wire(w).map(|p| p.0).collect())
        .collect();
    maps[0].intersection(&maps[1]).cloned().collect()
}

/// Find the distance of a wire intersection closest to the
/// origin.
fn min_intersect_dist(wires: &[Wire]) -> u64 {
    intersect_wires(wires)
        .iter()
        .map(|&p| aoc::manhattan_distance((0, 0), p))
        .min()
        .expect("no min dist")
}

/// Find the length of a wire intersection with minimal wire
/// length.
fn min_intersect_len(wires: &[Wire]) -> u64 {
    assert_eq!(wires.len(), 2);
    let traces: Vec<HashMap<Point, u64>> =
        wires.iter().map(|w| trace_wire(w).collect()).collect();
    intersect_wires(wires)
        .iter()
        .map(|p| traces[0][p] + traces[1][p])
        .min()
        .expect("no min len")
}

// Test the examples given with the puzzle against both part
// 1 and part 2 answers.
#[test]
fn test_examples() {
    #[rustfmt::skip]
    let examples = &[
        (
            vec![
                "R8,U5,L5,D3",
                "U7,R6,D4,L4",
            ],
            6,
            30,
        ),
        (
            vec![
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
            ],
            159,
            610,
        ),
        (
            vec![
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            ],
            135,
            410,
        ),
    ];
    for (e, d, l) in examples {
        let wires: Vec<Wire> = e.iter().map(parse_wire).collect();
        // Part 1.
        assert_eq!(min_intersect_dist(&wires), *d);
        // Part 2.
        assert_eq!(min_intersect_len(&wires), *l);
    }
}

pub fn main() {
    let wires = read_wires();
    let result = match aoc::get_part() {
        aoc::args::Part1 => min_intersect_dist(&wires),
        aoc::args::Part2 => min_intersect_len(&wires),
    };
    println!("{}", result);
}
