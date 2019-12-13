// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 13.  
//! Bart Massey 2019

use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;

use aoc::{Terminus::*, *};

/// Type of the map.
type Map = HashMap<Point, i64>;

/// Tile numbers.
#[allow(dead_code)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}
use Tile::*;

/// Map a tile number to its corresponding display
/// character.
fn render_tile(t: i64) -> char {
    const RENDER: [char; 5] = [' ', '+', '#', '_', 'o'];
    assert!(t >= 0 && (t as usize) < RENDER.len());
    RENDER[t as usize]
}

/// Expect an output from the program.
fn get_output(prog: &mut Intcode) -> i64 {
    match prog.run() {
        HaveOutput(q) => q,
        Halted => panic!("unexpected halt"),
        NeedInput => panic!("unexpected input request"),
    }
}

/// Render the playfield with no input.
fn paint(mut prog: Intcode) -> Map {
    let mut map: Map = HashMap::new();
    loop {
        let x = match prog.run() {
            HaveOutput(q) => q,
            Halted => return map,
            NeedInput => panic!("unexpected ask for input"),
        };
        let y = get_output(&mut prog);
        let t = get_output(&mut prog);
        map.insert((x, y), t);
    }
}

/// Play the game and return the score. If a delay is
/// provided, display game outputs with that delay after the
/// game starts.
fn play(mut prog: Intcode, delay: Option<Duration>) -> i64 {
    // XXX Only used for display.
    let mut stdout = std::io::stdout();
    let mut map: Map = HashMap::new();

    // Game state.
    let mut dirn = 0;
    let mut score = 0;
    // XXX Paddle is -2 until seen. Used by display code to
    // decide whether to delay.
    let mut paddle = -2;

    // Insert two coins.
    prog.poke(0, 2);

    // Play the game.
    loop {
        let x = match prog.run() {
            HaveOutput(q) => q,
            Halted => return score,
            NeedInput => {
                prog.add_input(dirn);
                continue;
            }
        };
        let y = get_output(&mut prog);
        let t = get_output(&mut prog);

        // Got a score instead of a render instruction.
        // XXX Will display score later when rendering.
        if x == -1 {
            score = t;
            continue;
        }

        // The controller. Try to keep the paddle under the
        // ball as much as possible.
        if t == Paddle as i64 {
            paddle = x;
        }
        if t == Ball as i64 {
            dirn = sgn(x - paddle);
        }

        // Render the map character.
        map.insert((x, y), t);

        if let Some(delay) = delay {
            // XXX ANSI escape sequence to clear the screen.
            print!("\u{1b}[H\u{1b}[2J");
            // Use libaoc to render the map.
            print!("{}", render_map(&map, render_tile, ' '));
            println!("score: {}", score);
            // The println!() above should take care of
            // flushing stdout, but who knows?
            stdout.flush().unwrap();
            // Only delay once the paddle is visible.
            if paddle != -2 {
                std::thread::sleep(delay);
            }
        }
    }
}

pub fn main() {
    let prog = aoc::Intcode::read();
    let (part, args) = aoc::get_part_args();
    match part {
        aoc::Part1 => {
            let map = paint(prog);
            // Count only block characters.
            let nblocks =
                map.values().filter(|&t| *t == Block as i64).count();
            println!("{}", nblocks);
        }
        aoc::Part2 => {
            let score = if args.is_empty() {
                play(prog, None)
            } else {
                let delay = args[0].parse().expect("illegal delay");
                let delay = Duration::from_millis(delay);
                play(prog, Some(delay))
            };
            println!("{}", score);
        }
    }
}
