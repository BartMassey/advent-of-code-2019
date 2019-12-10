// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 10.  
//! Bart Massey 2019

use std::collections::{HashMap, HashSet};

/// Type of coordinates and coordinate differences.
// XXX We could probably be more careful because the
// coordinates themselves are unsigned, but then Rust just
// forces a giant pile of casty garbage everywhere, so no.
type Coord = (i64, i64);

/// Represent the map as a set of asteroids.
type Map = HashSet<Coord>;

/// Read a map from a text description.
fn read_map<S>(lines: &[S]) -> Map
where
    S: AsRef<str>,
{
    let mut map = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.as_ref().chars().enumerate() {
            match c {
                '#' => {
                    map.insert((x as i64, y as i64));
                }
                '.' => (),
                c => panic!("unexpected char {} in map", c as u32),
            }
        }
    }
    map
}

/// The GCD is not part of standard Rust. We don't need
/// super-efficiency, so we just use the faster form of the
/// [Euclidean
/// Algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm#Procedure).
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// The GCD is worth testing on its own.
#[test]
fn test_gcd() {
    assert_eq!(0, gcd(0, 0));
    assert_eq!(1, gcd(1, 0));
    assert_eq!(1, gcd(0, 1));
    assert_eq!(5, gcd(5, 0));
    assert_eq!(5, gcd(0, 5));
    assert_eq!(1, gcd(3, 5));
    assert_eq!(2, gcd(2, 4));
    assert_eq!(2, gcd(4, 2));
    assert_eq!(3, gcd(9, 12));
}

/// This is the heart of the computation for this week.
/// Compute a "reduced slope" as a *run, rise* pair but
/// divided by their GCD. This is what the problem
/// description seems to mean by occlusion: same reduced
/// slope.
fn int_slope((dx, dy): Coord) -> Coord {
    assert!(dx != 0 || dy != 0);
    let q = gcd(dx.abs(), dy.abs());
    (dx / q, dy / q)
}

// Do some computations taken from the first example to
// check the occlusion hypothesis.
#[test]
fn test_int_slope() {
    assert_eq!(int_slope((-2, 4)), int_slope((-1, 2)));
    assert!(int_slope((1, 2)) != int_slope((1, 4)));
    assert!(int_slope((0, 2)) != int_slope((1, 4)));
}

/// Subtract second tuple from first.
// XXX Why isn't there tuple arithmetic by default in Rust?
// There just isn't, that's why.
fn coord_sub((x1, y1): Coord, (x2, y2): Coord) -> Coord {
    (x1 - x2, y1 - y2)
}

/// Return a map from a slope from the given origin to the
/// collection of asteroid coordinates on the map that share
/// that slope. Each collection is a `Vec` in arbitrary
/// order.
fn find_slopes(map: &Map, origin: Coord) -> HashMap<Coord, Vec<Coord>> {
    // Make an iterator over slope, coordinate pairs from
    // the map.
    let slopes = map
        .iter()
        .filter(|&&a| a != origin)
        .map(|&a| (int_slope(coord_sub(a, origin)), a));

    // Build up the result map using the iterator.
    let mut sc = HashMap::new();
    for (s, c) in slopes {
        let coords = sc.entry(s).or_insert_with(Vec::new);
        coords.push(c);
    }

    sc
}

/// Count the asteroids visible from a given origin
/// asteroid.
fn count_visible(map: &Map, origin: Coord) -> usize {
    find_slopes(map, origin).len()
}

/// Find the optimal observatory origin and its count.
fn max_visibility(map: &Map) -> (usize, Coord) {
    map.iter()
        .map(|&a| (count_visible(map, a), a))
        .max_by_key(|&a| a.0)
        .expect("only one asteroid")
}

// Some of the tests from the problem.
#[test]
fn test_visible() {
    #[rustfmt::skip]
    let map = &[
        ".#..#",
        ".....",
        "#####",
        "....#",
        "...##",
    ];
    let map = read_map(map);
    #[rustfmt::skip]
    let tests = &[
        (8, (3, 4)),
        (6, (0, 2)),
        (7, (2, 2)),
        (5, (4, 2)),
    ];
    for &(c, a) in tests {
        assert_eq!(c, count_visible(&map, a));
    }
    assert_eq!((8, (3, 4)), max_visibility(&map));
}

/// Return the coordinate of the nth asteroid vaporized by
/// the process described in Part 2 of the problem.
fn nth_vaporized(map: &Map, origin: Coord, mut n: usize) -> Coord {
    assert!(n <= map.len());
    // Get the slope sets.
    let mut aslopes = find_slopes(map, origin);

    // Sort each collection `Vec` in decreasing order by
    // distance.  This allows popping the nearest off the
    // end.  (A priority queue would be slightly more
    // efficent than a `Vec` for Part 2, but less so for
    // Part 2. So meh.)
    for v in aslopes.values_mut() {
        v.sort_by_key(|&c| {
            let (x, y) = coord_sub(c, origin);
            -(x * x + y * y)
        });
    }

    // Make a vector of the slope sets sorted by increasing
    // angle. To say that the `-atan2()` with backward
    // arguments is a bit odd would be an understatement.
    // Note the necessary use of `OrderedFloat` to avoid
    // Rust's lack of total order on floats due to
    // mishandling `NaN`.
    let mut slopes: Vec<Coord> = aslopes.keys().cloned().collect();
    slopes.sort_by_key(|&(x, y)| {
        ordered_float::OrderedFloat::from(-f64::atan2(
            x as f64, y as f64,
        ))
    });

    // Walk around the circle now that it is set up.  No
    // effort is made to remove empty sets from play.  Code
    // does try to detect infinite loop (for example if `n`
    // is larger than the number of asteroids, although
    // this is checked for above).
    let mut i = 0;
    let mut progress = false;
    loop {
        let slope = slopes[i];
        let roids = aslopes.get_mut(&slope).unwrap();
        if let Some(shot) = roids.pop() {
            progress = true;
            n -= 1;
            if n == 0 {
                return shot;
            }
        }
        i = (i + 1) % slopes.len();
        if i == 0 {
            if !progress {
                panic!("ran out of asteroids with n = {}", n);
            }
            progress = false;
        }
    }
}

// More tests from the problem.
#[test]
fn test_nth_vaporized() {
    #[rustfmt::skip]
    let map = &[
        ".#....#####...#..",
        "##...##.#####..##",
        "##...#...#.#####.",
        "..#.....#...###..",
        "..#.#.....#....##",
        ];
    let map = read_map(map);
    let origin = (8, 3);
    #[rustfmt::skip]
    let tests = &[
        ((8, 1), 1),
        ((9, 0), 2),
        ((15, 1), 9),
        ((2, 4), 19),
        ((5, 1), 27),
        ((14, 3), 36),
    ];
    for &(c, n) in tests {
        assert_eq!(c, nth_vaporized(&map, origin, n));
    }

    #[rustfmt::skip]
    let map = &[
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##",
    ];
    let map = read_map(map);
    let (nvis, origin) = max_visibility(&map);
    assert_eq!((nvis, origin), (210, (11, 13)));
    #[rustfmt::skip]
    let tests = &[
        (1, (11,12)),
        (2, (12,1)),
        (3, (12,2)),
        (10, (12,8)),
        (20, (16,0)),
        (50, (16,9)),
        (100, (10,16)),
        (199, (9,6)),
        (200, (8,2)),
        (201, (10,9)),
        (299, (11,1)),
    ];
    for &(n, c) in tests {
        assert_eq!(c, nth_vaporized(&map, origin, n));
    }
}

pub fn main() {
    let lines: Vec<String> = aoc::input_lines().collect();
    let map = read_map(&lines);
    let (nvis, origin) = max_visibility(&map);
    let part = aoc::get_part();
    match part {
        aoc::Part1 => println!("{}", nvis),
        aoc::Part2 => {
            let (x, y) = nth_vaporized(&map, origin, 200);
            println!("{:?}", x * 100 + y);
        }
    }
}
