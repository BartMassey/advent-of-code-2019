// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 11.  
//! Bart Massey 2019

use std::collections::{HashMap, HashSet};

use aoc::{dirns::*, intcode::*, Terminus::*};

type Map = HashMap<Point, i64>;

fn paint(mut prog: Intcode, mut default_color: i64) -> Map {
    let mut map: Map = HashMap::new();
    let mut facing = Dirn::Up;
    let mut posn = (0, 0);
    loop {
        let color = *map.get(&posn).unwrap_or(&default_color);
        default_color = 0;
        prog.add_input(color);
        let new_color = match prog.run() {
            HaveOutput(q) => q,
            Halted => break,
            NeedInput => panic!("unexpected input request"),
        };
        map.insert(posn, new_color);
        let rot = match prog.run() {
            HaveOutput(q) => q,
            Halted => panic!("unexpected halt during output"),
            NeedInput => panic!("unexpected input request"),
        };
        let rot = match rot {
            0 => Rot::CCW,
            1 => Rot::CW,
            _ => panic!("unexpected rotation"),
        };
        facing = facing.turn(rot);
        let (dx, dy) = facing.disp();
        posn.0 += dx;
        posn.1 += dy;
    }
    map
}

pub fn main() {
    let prog = aoc::Intcode::read();
    let part = aoc::get_part();
    match part {
        aoc::Part1 => {
            let map = paint(prog, 0);
            println!("{}", map.len());
        }
        aoc::Part2 => {
            let map = paint(prog, 1);
            let map: HashSet<Point> = map
                .iter()
                .filter(|&(_, &v)| v == 1)
                .map(|(&k, _)| k)
                .collect();
            print!("{}", aoc::render(&map));
        }
    }
}
