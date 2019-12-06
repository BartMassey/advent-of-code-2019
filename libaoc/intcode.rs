//! Intcode interpreter for Advent of Code 2019 solutions.
//!
//! This is a fairly standard pcode setup with the exception
//! of being highly self-modifying code. Use
//! `Intcode::read()` to load a program from `stdin`;
//! `input()` to load input values; `run()` to run until
//! halted; `output()` to get the output value.

#[derive(Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Opcode {
    fn new(code: usize) -> Self {
        use Opcode::*;
        if code == 99 {
            return Halt;
        }
        let codes = [
            Add,
            Mul,
            Input,
            Output,
            JumpIfTrue,
            JumpIfFalse,
            LessThan,
            Equals,
        ];
        if code == 0 || code > codes.len() {
            panic!("illegal opcode {}", code);
        }
        codes[code - 1]
    }
}

#[derive(Clone, Copy)]
enum OpndMode {
    Imm,
    Pos,
}

impl OpndMode {
    fn new(mode: usize) -> Self {
        let modes = [Self::Pos, Self::Imm];
        if mode > modes.len() {
            panic!("illegal opnd mode {}", mode);
        }
        modes[mode]
    }
}

struct Decode<'a> {
    prog: &'a mut Vec<i64>,
    index: usize,
    modebits: usize,
}

impl<'a> Decode<'a> {
    fn new(prog: &'a mut Vec<i64>, index: usize) -> (Opcode, Self) {
        let opcode = prog[index] as usize;
        let op = Opcode::new(opcode % 100);
        let modebits = opcode / 100;
        (
            op,
            Self {
                prog,
                modebits,
                index: index + 1,
            },
        )
    }

    // XXX Ugly copy-paste here that is awkward to get rid of.

    fn fetch(&mut self) -> i64 {
        let nprog = self.prog.len();
        let mode = self.modebits % 10;
        if self.index > nprog {
            panic!("fetch off end");
        }
        let opnd = self.prog[self.index];
        let val = match OpndMode::new(mode) {
            OpndMode::Imm => opnd,
            OpndMode::Pos => {
                if opnd < 0 || opnd as usize > nprog {
                    panic!("fetch position out of range");
                }
                self.prog[opnd as usize]
            }
        };
        self.modebits /= 10;
        self.index += 1;
        val
    }

    fn store(&mut self, val: i64) {
        let nprog = self.prog.len();
        let mode = self.modebits % 10;
        if self.index > nprog {
            panic!("store off end");
        }
        let opnd = self.prog[self.index];
        match OpndMode::new(mode) {
            OpndMode::Imm => {
                panic!("store to immediate");
            }
            OpndMode::Pos => {
                if opnd < 0 || opnd as usize > nprog {
                    panic!("store position out of range");
                }
                self.prog[opnd as usize] = val;
            }
        }
        self.modebits /= 10;
        self.index += 1;
    }

    fn skip(&mut self) {
        self.modebits /= 10;
        self.index += 1;
    }

    fn finish(self) -> usize {
        if self.modebits != 0 {
            panic!("unused mode bits");
        }
        self.index
    }
}

#[derive(Debug, Clone)]
pub struct Intcode {
    prog: Vec<i64>,
    inputs: Option<Vec<i64>>,
    outputs: Vec<i64>,
}

impl Intcode {
    fn new(prog: Vec<i64>) -> Self {
        Self {
            prog,
            inputs: None,
            outputs: Vec::new(),
        }
    }

    /// Builder for adding user inputs to the Intcode
    /// program before running. Can only be done once.
    pub fn with_inputs(mut self, inputs: Vec<i64>) -> Self {
        assert!(self.inputs.is_none());
        self.inputs = Some(inputs);
        self
    }

    /// View outputs from the Intcode program after running.
    pub fn view_outputs(&self) -> &[i64] {
        &self.outputs
    }

    /// Read and parse the program from stdin in the
    /// specified format.
    pub fn read() -> Self {
        use std::io::{stdin, BufReader, Read};

        let mut text = String::new();
        BufReader::new(stdin()).read_to_string(&mut text).unwrap();
        let prog = text
            .trim_end()
            .split(',')
            .map(|w| w.parse().unwrap())
            .collect();
        Self::new(prog)
    }

    /// Run this Intcode program.
    pub fn run(&mut self) {
        let prog = &mut self.prog;
        let nprog = prog.len();

        // The actual emulator loop tries to be careful in
        // its checking.
        let mut ip: usize = 0;
        while ip < nprog {
            let (op, mut opnds) = Decode::new(prog, ip);
            use Opcode::*;
            ip = match op {
                Halt => return,
                Add | Mul | LessThan | Equals => {
                    let src1 = opnds.fetch();
                    let src2 = opnds.fetch();
                    let a = match op {
                        Add => src1 + src2,
                        Mul => src1 * src2,
                        LessThan => (src1 < src2) as i64,
                        Equals => (src1 == src2) as i64,
                        _ => unreachable!("wrong insn for ALU"),
                    };
                    opnds.store(a);
                    opnds.finish()
                }
                Input => {
                    let inputs = self
                        .inputs
                        .as_mut()
                        .expect("input was never provided");
                    let input =
                        inputs.pop().expect("input without value");
                    opnds.store(input);
                    opnds.finish()
                }
                Output => {
                    let output = opnds.fetch();
                    self.outputs.push(output);
                    opnds.finish()
                }
                JumpIfTrue | JumpIfFalse => {
                    let test = opnds.fetch();
                    let test = match op {
                        JumpIfTrue => test != 0,
                        JumpIfFalse => test == 0,
                        _ => unreachable!("wrong insn for jump"),
                    };
                    if test {
                        let target = opnds.fetch();
                        if target < 0 || target as usize > nprog {
                            panic!("jump target out of range");
                        }
                        target as usize
                    } else {
                        opnds.skip();
                        opnds.finish()
                    }
                }
            }
        }
        panic!("program ran off end");
    }

    /// Day 2: Input a "verb" and "noun" before starting the program.
    pub fn input(&mut self, verb: i64, noun: i64) {
        self.prog[1] = verb;
        self.prog[2] = noun;
    }

    /// Day 2: Fetch output after running the program.
    pub fn output(&self) -> i64 {
        self.prog[0]
    }
}

// The test examples given in the Day 2 problem.
#[test]
fn test_day02() {
    // We generally want whatever rustfmt does in our code.
    // In this case it gows things up trying to be clever.
    // So we tell it "no".
    #[rustfmt::skip]
    let testcases: &[(&[i64], &[i64])] = &[
        (
            &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        ),
        (
            &[1, 0, 0, 0, 99],
            &[2, 0, 0, 0, 99],
        ),
        (
            &[2, 3, 0, 3, 99],
            &[2, 3, 0, 6, 99],
        ),
        (
            &[2, 4, 4, 5, 99, 0],
            &[2, 4, 4, 5, 99, 9801],
        ),
        (
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
    ];
    for (init, fin) in testcases {
        let mut init = Intcode::new(init.to_vec());
        let fin = fin.to_vec();
        init.run();
        assert_eq!(init.prog, fin);
    }
}

// The test examples given in the Day 5 problem.
// Picked some inputs and outputs to try.
#[test]
fn test_day05() {
    #[rustfmt::skip]
    let testcases: &[(&[i64], &[(i64, i64)])] = &[
        (
            &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            &[(8, 1), (9, 0)],
        ),
        (
            &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[(7, 1), (8, 0), (9, 0)],
        ),
        (
            &[3, 3, 1108, -1, 8, 3, 4, 3, 99],
            &[(8, 1), (9, 0)],
        ),
        (
            &[3, 3, 1107, -1, 8, 3, 4, 3, 99],
            &[(7, 1), (8, 0), (9, 0)],
        ),
        (
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[(0, 0), (9, 1)],
        ),
        (
            &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &[(0, 0), (9, 1)],
        ),
        (
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107,
                8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
                0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46,
                104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
                20, 1105, 1, 46, 98, 99,
            ],
            &[(6, 999), (8, 1000), (9, 1001)],
        ),
    ];
    for (init, io) in testcases {
        for &(input, output) in io.iter() {
            let mut init =
                Intcode::new(init.to_vec()).with_inputs(vec![input]);
            init.run();
            assert_eq!(init.view_outputs(), &[output]);
        }
    }
}
