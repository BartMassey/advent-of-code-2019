// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 15.  
//! Bart Massey 2019

use std::collections::{HashMap, VecDeque};

use aoc::*;
use Dirn::*;
use Terminus::*;

/// The code corresponding to a given turn direction, as
/// specified in the problem.
fn dirn_code(d: Dirn) -> i64 {
    match d {
        Up => 1,
        Down => 2,
        Left => 3,
        Right => 4,
    }
}

/// This map gives a shortest path back to the origin from a
/// given explored point.
type Map = HashMap<Point, Dirn>;

/// Robot statuses as reported by the program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Blocked = 0,
    Moved = 1,
    FoundGoal = 2,
}
use Status::*;

/// Try to move the robot in a given direction and report
/// what happened. The `goal` parameter is true iff we are
/// treating a move to the goal differently from a move to
/// another empty square.
fn move_robot(prog: &mut Intcode, dirn: Dirn, goal: bool) -> Status {
    trace!("move robot {:?}", dirn);
    prog.add_input(dirn_code(dirn));
    match prog.run() {
        Halted => panic!("unexpected robot halt"),
        NeedInput => panic!("unexpected extra input"),
        HaveOutput(q) => match q {
            0 => Blocked,
            1 => Moved,
            2 => {
                if goal {
                    FoundGoal
                } else {
                    Moved
                }
            }
            _ => panic!("unexpected output"),
        },
    }
}

/// Do a Dijkstra search from an assumed "origin" using
/// the given robot. If `stop_early` is true, stop when
/// hitting the goal and return its distance. Otherwise
/// return the max distance reached while exploring the
/// entire space, ignoring the goal.
fn dijkstra(prog: &mut Intcode, stop_early: bool) -> usize {
    let origin = (0, 0);

    // Record the steps of paths back to the origin.  The
    // step direction for each point is oriented toward the
    // origin.
    let mut map = HashMap::new();

    // Calculate the full path from the given position back
    // to the origin.
    let to_origin = |map: &Map, mut p| {
        let mut path = Vec::new();
        while let Some(&dirn) = map.get(&p) {
            path.push(dirn);
            p = dirn.displace(p);
        }
        path
    };

    // Run the Dijkstra search.
    let mut que = VecDeque::new();
    let mut max_dist = 0;
    que.push_back((0, origin));
    while let Some((dist, p)) = que.pop_front() {
        trace!("exploring {:?} ({})", p, dist);
        max_dist = dist;
        let dist = dist + 1;

        // Move robot from origin to position.
        trace!("positioning");
        let path = to_origin(&map, p);
        for &dirn in path.iter().rev() {
            let dirn = dirn.reverse();
            let status = move_robot(prog, dirn, stop_early);
            assert_eq!(status, Moved);
        }

        // Explore neighborhood of position.
        trace!("visiting neighbors");
        for &dirn in FACINGS.iter() {
            let next = dirn.displace(p);
            if next == origin || map.get(&next).is_some() {
                trace!("{:?} already visited", dirn);
                continue;
            }

            let status = move_robot(prog, dirn, stop_early);
            match status {
                FoundGoal => {
                    if stop_early {
                        trace!("Found goal at {:?}!", next);
                        return dist;
                    } else {
                        panic!("found generator at {:?}", p);
                    }
                }
                Blocked => {
                    trace!("{:?} blocked", dirn);
                    continue;
                }
                Moved => (),
            }
            let rev = dirn.reverse();
            map.insert(next, rev);
            que.push_back((dist, next));
            let status = move_robot(prog, rev, stop_early);
            assert_eq!(status, Moved);
            trace!("{:?} queued", dirn);
        }

        // Return robot to origin.
        trace!("returning");
        for &dirn in path.iter() {
            let status = move_robot(prog, dirn, stop_early);
            assert_eq!(status, Moved);
        }
    }

    if stop_early {
        panic!("goal not found")
    } else {
        max_dist
    }
}

pub fn main() {
    let mut prog = Intcode::read();
    let part = get_part();
    // Always need to move the robot to the goal.
    let max_reached = dijkstra(&mut prog, true);
    match part {
        aoc::Part1 => {
            // Need do nothing else.
            println!("{}", max_reached);
        }
        aoc::Part2 => {
            // Now explore the space around the goal.
            let max_reached = dijkstra(&mut prog, false);
            println!("{}", max_reached);
        }
    }
}
