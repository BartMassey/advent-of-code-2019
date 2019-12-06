// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 6.  
//! Bart Massey 2019

use multimap::MultiMap;

type Map = MultiMap<String, String>;

/// Read the lines of a map and parse them into a map object
/// for processing.
fn build_map<S>(lines: impl Iterator<Item = S>) -> Map
where
    S: AsRef<str>,
{
    lines
        .map(|l| {
            // XXX This fiddly mess avoids doing unnecessary
            // allocations or collecting. It's not too
            // readable, though.
            let mut fields =
                l.as_ref().trim_end().split(')').map(|f| f.to_string());
            let key = fields.next().expect("missing key");
            let val = fields.next().expect("missing val");
            assert_eq!(None, fields.next());
            (key, val)
        })
        .collect()
}

/// Count the orbits from the given starting point, which is
/// assumed to be at the given depth from the original
/// starting point.
// XXX There is probably a way to get rid of the depth
// parameter, but this works.
fn count_orbits(m: &Map, here: &str, depth: u64) -> u64 {
    match m.get_vec(here) {
        None => depth,
        Some(children) => {
            children
                .iter()
                .map(|child| count_orbits(m, &child, depth + 1))
                .sum::<u64>()
                + depth
        }
    }
}

// Test example from problem description. I threw in a
// mini-example to aid in debugging.
#[test]
fn test_count_orbits() {
    #[rustfmt::skip]
    let tests: &[(u64, &[&str])] = &[
        (
            5,
            &[
                "COM)B",
                "B)C",
                "B)G",
            ],
        ),
        (
            42,
            &[
                "COM)B",
                "B)C",
                "C)D",
                "D)E",
                "E)F",
                "B)G",
                "G)H",
                "D)I",
                "E)J",
                "J)K",
                "K)L",
            ],
        ),
    ];
    for (output, input) in tests {
        let map = build_map(input.iter());
        assert_eq!(*output, count_orbits(&map, "COM", 0));
    }
}

/// Find a path from a given starting point to a given target
/// in the tree. Returns None if no path was found.
fn find_path<'a>(
    m: &'a Map,
    here: &'a str,
    target: &'a str,
) -> Option<Vec<&'a str>> {
    if here == target {
        return Some(Vec::new());
    }
    match m.get_vec(here) {
        None => None,
        Some(children) => {
            for child in children {
                let path = find_path(m, child, target);
                if let Some(mut path) = path {
                    path.push(here);
                    return Some(path);
                }
            }
            None
        }
    }
}

/// Count the transfers from you to Santa. Finds the path
/// from the common center to each, and then discards the
/// duplicate portion.
fn count_transfers(m: &Map) -> u64 {
    let mut you_path =
        find_path(m, "COM", "YOU").expect("no path to you");
    let mut santa_path =
        find_path(m, "COM", "SAN").expect("no path to santa");
    loop {
        let you_posn = you_path.pop().expect("ran out of you");
        let santa_posn = santa_path.pop().expect("ran out of santa");
        if you_posn != santa_posn {
            return you_path.len() as u64 + santa_path.len() as u64 + 2;
        }
    }
}

// Example test from the problem description.
#[test]
fn test_count_transfers() {
    #[rustfmt::skip]
    let lines = &[
        "COM)B",
        "B)C",
        "C)D",
        "D)E",
        "E)F",
        "B)G",
        "G)H",
        "D)I",
        "E)J",
        "J)K",
        "K)L",
        "K)YOU",
        "I)SAN",
    ];
    let map = build_map(lines.iter());
    // find_path tests.
    let you_path = vec!["K", "J", "E", "D", "C", "B", "COM"];
    let santa_path = vec!["I", "D", "C", "B", "COM"];
    assert_eq!(Some(you_path), find_path(&map, "COM", "YOU"));
    assert_eq!(Some(santa_path), find_path(&map, "COM", "SAN"));
    // count_transfers test.
    assert_eq!(4, count_transfers(&map));
}

pub fn main() {
    let map = build_map(aoc::input_lines());
    let part = aoc::get_part();
    let count = match part {
        aoc::Part1 => count_orbits(&map, "COM", 0),
        aoc::Part2 => count_transfers(&map),
    };
    println!("{:?}", count);
}
