// Copyright © 2016 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Directions management for Advent of Code solutions.
//!
//! To use this, make a new `GridBox` to set clipping bounds,
//! then call the `neighbors()` method of the `ClipBox` to get
//! an iterator over clipped neighbors in cardinal directions.
//!
//! # Examples
//!
//! ```rust
//! use aoc::dirns::*;
//!
//! let clip_box = GridBox::new(3, 4);
//! let neighbors = clip_box.neighbors((2, 0))
//!                 .collect::<Vec<_>>();
//! assert_eq!(neighbors, vec![(1, 0), (2, 1)]);
//! ```

/// Symbolic direction constants. It is unfortunate that
/// these need to be matched to DIRNS below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dirn {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

/// Rotation directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rot {
    /// Counter-clockwise.
    CCW,
    /// Clockwise.
    CW,
}

/// Displacements induced by the cardinal directions: up,
/// down, left, right in an x-y coordinate system where
/// increasing y is down.
pub const DIRNS: [(i64, i64); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

/// The possible facings.
pub const FACINGS: [Dirn; 4] =
    [Dirn::Up, Dirn::Left, Dirn::Down, Dirn::Right];

impl Dirn {
    /// Displacement resulting from a step in the given
    /// direction.
    pub fn disp(self) -> (i64, i64) {
        DIRNS[self as usize]
    }

    /// Apply the appropriate displacement for
    /// this direction to the given point.
    pub fn displace(self, mut p: Point) -> Point {
        let disp = self.disp();
        p.0 += disp.0;
        p.1 += disp.1;
        p
    }

    /// Direction resulting from turning in the given
    /// rotation direction.
    pub fn turn(self, rot: Rot) -> Dirn {
        let offset = match rot {
            Rot::CCW => 1,
            Rot::CW => FACINGS.len() - 1,
        };
        FACINGS[(self as usize + offset) % FACINGS.len()]
    }

    /// Direction resulting from turning around.
    pub fn reverse(self) -> Dirn {
        FACINGS[(self as usize + 2) % FACINGS.len()]
    }
}

#[test]
fn test_rot() {
    use Dirn::*;
    use Rot::*;
    assert_eq!(Left, Up.turn(CCW));
    assert_eq!(Right, Up.turn(CW));
    assert_eq!(Down, Left.turn(CCW));
    assert_eq!(Down, Right.turn(CW));
}

/// Type of coordinates.
pub type Point = (i64, i64);

/// Description of the grid, for possible clipping.
#[derive(Copy, Clone)]
pub enum GridBox {
    /// Grid is clipped on bottom and right.
    ClipBox(Point),
    /// Grid is unclipped.
    Unclipped,
}

use self::GridBox::*;

impl GridBox {
    /// Create a clip box for neighbor calculations.
    #[allow(dead_code)]
    pub fn new(x_size: i64, y_size: i64) -> GridBox {
        ClipBox((x_size, y_size))
    }

    /// Create an "unbounded clip box" for neighbor
    /// calculations.  Negative locations will still be
    /// clipped.
    pub fn new_grid() -> GridBox {
        Unclipped
    }

    /// Return an iterator that will produce the neighbors
    /// of the given location, clipped as needed.
    pub fn neighbors(&self, location: Point) -> Neighbors {
        if let ClipBox((x_size, y_size)) = *self {
            let (x, y) = location;
            assert!(x < x_size && y < y_size);
        };
        Neighbors::new(*self, location)
    }

    /// Return the source location adjusted by the given offset
    /// iff the dest location is in-bounds. This is useful when
    /// "manual" clipping is needed.
    pub fn clip(&self, loc: Point, off: (i64, i64)) -> Option<Point> {
        let (x, y) = loc;
        let (dx, dy) = off;
        let nx = x + dx;
        let ny = y + dy;
        if nx < 0 || ny < 0 {
            return None;
        }
        if let ClipBox((x_size, y_size)) = *self {
            if nx >= x_size as i64 || ny >= y_size as i64 {
                return None;
            }
        };
        Some((nx, ny))
    }
}

/// Iterator over the neighbors of a point in the four cardinal
/// directions, clipped as appropriate.
pub struct Neighbors {
    /// Possible upper bounds on neighbor location.
    bounds: GridBox,
    /// Source location.
    loc: Point,
    /// Iterator for cardinal directions.
    dirns: Box<dyn Iterator<Item = &'static (i64, i64)>>,
}

impl Neighbors {
    /// Return an iterator over the neighbors of
    /// the given grid box starting at the given location.
    pub fn new(grid_box: GridBox, location: Point) -> Self {
        Neighbors {
            bounds: grid_box,
            loc: location,
            dirns: Box::new(DIRNS.iter()),
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    /// Return the next cardinal neighbor of the source point,
    /// clipped as needed.
    fn next(&mut self) -> Option<Point> {
        loop {
            match self.dirns.next() {
                Some(&d) => {
                    if let Some(n) = self.bounds.clip(self.loc, d) {
                        return Some(n);
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}

/// The ["Manhattan Distance"][1] between two points.
///
/// [1]: http://en.wikipedia.org/wiki/Taxicab_geometry
pub fn manhattan_distance((x1, y1): Point, (x2, y2): Point) -> u64 {
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    (dx + dy) as u64
}
