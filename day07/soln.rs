// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 7.  
//! Bart Massey 2019

use aoc::Intcode;

/// Run an amplifier chain in feedforward mode. Each
/// amplifier runs the given program and uses its given
/// setting.
fn chain_output(prog: &Intcode, settings: &[i64]) -> i64 {
    // This "input" will be set by previous outputs.
    let mut input = 0;
    for s in settings.as_ref().iter() {
        let mut prog = prog.clone().with_inputs(vec![*s, input]);
        let outputs = prog.collect_outputs();
        assert_eq!(1, outputs.len());
        input = outputs[0];
    }
    input
}

/// Run an amplifier chain in feedback mode. Each amplifier
/// runs the given program and uses its given setting.
fn chain_output_feedback(prog: &Intcode, settings: &[i64]) -> i64 {
    // Set up the programs with their parameters.
    let mut progs: Vec<Intcode> = settings
        .as_ref()
        .iter()
        .map(|s| {
            let mut prog = prog.clone();
            prog.add_input(*s);
            prog
        })
        .collect();

    use aoc::Terminus::*;
    // This "input" will be set by previous outputs.
    let mut input = 0;

    // Run the chain until completion. We assume that the
    // program is chainable: takes one input, produces one
    // output. We assume that the first amplifier will halt
    // first and the others follow: we will check this.
    'finished: loop {
        for prog in &mut progs {
            loop {
                match prog.run() {
                    Halted => break 'finished,
                    NeedInput => prog.add_input(input),
                    HaveOutput(out) => {
                        input = out;
                        break;
                    }
                }
            }
        }
    }

    // Run the other amplifiers to make sure they're about
    // to stop.
    for prog in &mut progs[1..] {
        match prog.run() {
            Halted => (),
            _ => panic!("subsequent amplifier failed to halt"),
        }
    }

    input
}

/// Try all possible parameter settings, running the given
/// amplifier chain on each to find its value.  Return the
/// max value. The parameter space is given as `initial`: it
/// is one legal set of settings to start the permutation.
fn max_output<C>(
    prog: &Intcode,
    chain: C,
    initial: impl Iterator<Item=i64>,
    ) -> i64
    where C: Fn(&Intcode, &[i64]) -> i64
{
    let mut initial: Vec<i64> = initial.collect();
    // "Heap" because "Heap's Algorithm": nothing to do with
    // other kinds of CS heap. This constructs an iterator
    // that produces Vecs containing permutations of the
    // input in some order.
    permutohedron::Heap::new(&mut initial)
        .map(|s| chain(prog, &s))
        .max()
        .expect("internal max error")
}

// Tests given in Part 1 of the problem.
#[test]
fn test_chain() {
    #[rustfmt::skip]
    let tests = vec![
        (
            vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
            &[4,3,2,1,0],
            43210,
        ),
        (
            vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                 101,5,23,23,1,24,23,23,4,23,99,0,0],
            &[0,1,2,3,4],
            54321,
        ),
        (
            vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],
            &[1,0,4,3,2],
            65210,
        ),
    ];
    for (prog, settings, output) in tests {
        let prog = Intcode::new(prog);
        assert_eq!(output, chain_output(&prog, settings.as_ref()));
        assert_eq!(output, max_output(&prog, chain_output, 0..=4));
    }
}


// Tests given in Part 2 of the problem.
#[test]
fn test_chain_feedback() {
    #[rustfmt::skip]
    let tests = vec![
        (
            vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
            &[9,8,7,6,5],
            139629729,
        ),
        (

            vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,
                 55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,
                 53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                 53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],

            &[9,7,8,5,6],
            18216,
        ),
    ];
    for (prog, settings, output) in tests {
        let prog = Intcode::new(prog);
        assert_eq!(output, chain_output_feedback(&prog, settings));
        assert_eq!(output, max_output(&prog, chain_output_feedback, 5..=9));
    }
}

fn main() {
    let prog = Intcode::read();
    let part = aoc::get_part();
    let power = match part {
        aoc::Part1 => max_output(&prog, chain_output, 0..=4),
        aoc::Part2 => max_output(&prog, chain_output_feedback, 5..=9),
    };
    println!("{}", power);
}
