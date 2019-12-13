// Copyright © 2019 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Map rendering for Advent of Code solutions.

use std::collections::{HashMap, HashSet};

/// Compute the bounding box of a set of coordinates. The
/// max coordinate values of the box are the max coordinate
/// values that appear in the coordinate set (not 1
/// greater).
pub fn bounding_box<H>(
    map: &HashSet<(i64, i64), H>,
) -> ((i64, i64), (i64, i64))
where
    H: std::hash::BuildHasher,
{
    macro_rules! c {
        ($f:ident, $s:tt) => {
            map.iter().map(|&c| c.$s).$f().expect("empty map")
        };
    }
    ((c!(min, 0), c!(min, 1)), (c!(max, 0), c!(max, 1)))
}

/// Render a set of coordinates as an ASCII map.  The
/// resulting string will have '*' for coordinate locations
/// and ' ' elsewhere. Each line including the last will be
/// terminated by a newline.
pub fn render<H>(map: &HashSet<(i64, i64), H>) -> String
where
    H: std::hash::BuildHasher,
{
    let ((min_x, min_y), (max_x, max_y)) = bounding_box(&map);
    let width = (max_x + 1 - min_x) as usize;
    let height = (max_y + 1 - min_y) as usize;
    let mut result = String::with_capacity((width + 1) * height);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }
    result
}

/// Render a `HashMap` as an ASCII map.  The resulting
/// string will have characters at coordinate locations
/// represented in the map as defined by the rendering
/// function, and the default character elsewhere. Each line
/// including the last will be terminated by a newline.
pub fn render_map<H, F>(
    map: &HashMap<(i64, i64), i64, H>,
    mut render: F,
    default: char,
) -> String
where
    H: std::hash::BuildHasher,
    F: FnMut(i64) -> char,
{
    let posns: HashSet<(i64, i64)> = map.keys().cloned().collect();
    let ((min_x, min_y), (max_x, max_y)) = bounding_box(&posns);
    let width = (max_x + 1 - min_x) as usize;
    let height = (max_y + 1 - min_y) as usize;
    let mut result = String::with_capacity((width + 1) * height);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let t = map.get(&(x, y));
            let ch = match t {
                Some(t) => render(*t),
                None => default,
            };
            result.push(ch);
        }
        result.push('\n');
    }
    result
}
