//! Intcode interpreter for Advent of Code 2019 solutions.
//!
//! This is a fairly standard pcode setup with the exception
//! of being highly self-modifying code. Use
//! `Intcode::read()` to load a program from `stdin`;
//! `input()` to load input values; `run()` to run until
//! halted; `output()` to get the output value.

// Possible opcodes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Add = 0,
    Mul = 1,
    Input = 2,
    Output = 3,
    JumpIfTrue = 4,
    JumpIfFalse = 5,
    LessThan = 6,
    Equals = 7,
    Halt = 8,
    RBO = 9,
}

impl Opcode {
    // Make an opcode from a numeric code, checking for
    // validity.
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
            RBO,
        ];
        if code == 0 || code > codes.len() {
            panic!("illegal opcode {}", code);
        }
        codes[code - 1]
    }
}

// Mode for operand fetch / store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpndMode {
    Imm,
    Pos,
    Rel,
}

impl OpndMode {
    // Make a new operand mode from a number, checking for
    // validity.
    fn new(mode: usize) -> Self {
        let modes = [Self::Pos, Self::Imm, Self::Rel];
        if mode > modes.len() {
            panic!("illegal opnd mode {}", mode);
        }
        modes[mode]
    }
}

// Iterator-like object for fetching / storing successive
// arguments of an instruction.
struct Decode<'a> {
    prog: &'a mut Vec<i64>,
    index: usize,
    modebits: usize,
    rel_base: i64,
}

impl<'a> Decode<'a> {
    // Decode an instruction. Returns the `Opcode` and
    // itself as an iterator that will be used on successive
    // operands.
    fn new(
        prog: &'a mut Vec<i64>,
        index: usize,
        rel_base: i64,
    ) -> (Opcode, Self) {
        let opcode = prog[index] as usize;
        let op = Opcode::new(opcode % 100);
        let modebits = opcode / 100;
        (
            op,
            Self {
                prog,
                modebits,
                index: index + 1,
                rel_base,
            },
        )
    }

    // Treat the current instruction operand as a fetch and
    // get the value.
    //
    // XXX Ugly copy-paste of this code below that is
    // awkward to get rid of.
    fn fetch(&mut self) -> i64 {
        let nprog = self.prog.len();
        let mode = self.modebits % 10;
        if self.index >= nprog {
            self.prog.resize(self.index + 1, 0);
        }
        let mut opnd = self.prog[self.index];
        let mode = OpndMode::new(mode);
        let val = match mode {
            OpndMode::Imm => opnd,
            OpndMode::Pos | OpndMode::Rel => {
                if mode == OpndMode::Rel {
                    opnd += self.rel_base;
                }
                if opnd < 0 {
                    panic!("fetch position out of range");
                }
                if opnd as usize >= nprog {
                    self.prog.resize(opnd as usize + 1, 0);
                }
                self.prog[opnd as usize]
            }
        };
        self.modebits /= 10;
        self.index += 1;
        val
    }

    // Treat the current instruction operand as a store and
    // store the value.
    fn store(&mut self, val: i64) {
        let nprog = self.prog.len();
        let mode = self.modebits % 10;
        if self.index >= nprog {
            self.prog.resize(self.index + 1, 0);
        }
        let mut opnd = self.prog[self.index];
        let mode = OpndMode::new(mode);
        match mode {
            OpndMode::Imm => {
                panic!("store to immediate");
            }
            OpndMode::Pos | OpndMode::Rel => {
                if mode == OpndMode::Rel {
                    opnd += self.rel_base;
                }
                if opnd < 0 {
                    panic!("store position out of range");
                }
                if opnd as usize >= nprog {
                    self.prog.resize(opnd as usize + 1, 0);
                }
                self.prog[opnd as usize] = val;
            }
        }
        self.modebits /= 10;
        self.index += 1;
    }

    // Skip the current operand. This is used, for example,
    // for jumps not taken.
    fn skip(&mut self) {
        self.modebits /= 10;
        self.index += 1;
    }

    // Check that there are not remaining unconsumed modes
    // (would probably indicate a number-of-arguments error)
    // and then return the index one past the end of this
    // instruction.
    fn finish(self) -> usize {
        if self.modebits != 0 {
            panic!("unused mode bits");
        }
        self.index
    }
}

/// This is returned by `Intcode::run()` to indicate why it
/// stopped.
#[derive(Debug, Clone)]
pub enum Terminus {
    /// Program executed a `Halt` instruction.
    Halted,
    /// Program executed an `Input` instruction with
    /// no inputs buffered.
    NeedInput,
    /// Program executed an `Output` instruction with the
    /// given value.
    HaveOutput(i64),
}

/// An Intcode program. It has an input buffer for
/// preloading inputs, and saves its current instruction
/// pointer when suspending for input or output (and when
/// halted, although this is less useful.)
#[derive(Debug, Clone)]
pub struct Intcode {
    prog: Vec<i64>,
    inputs: Vec<i64>,
    ip: usize,
    rel_base: i64,
}

impl Intcode {
    /// Make a new intcode program from the given vector of
    /// instructions.
    pub fn new(prog: Vec<i64>) -> Self {
        Self {
            prog,
            inputs: Vec::new(),
            ip: 0,
            rel_base: 0,
        }
    }

    /// Builder for adding user inputs to the `Intcode`
    /// program before running.
    pub fn with_inputs(mut self, inputs: Vec<i64>) -> Self {
        self.inputs.reverse();
        self.inputs.extend_from_slice(&inputs);
        self.inputs.reverse();
        self
    }

    /// Add an input to the Intcode program while running.
    pub fn add_input(&mut self, input: i64) {
        self.inputs.reverse();
        self.inputs.push(input);
        self.inputs.reverse();
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

    /// Run this Intcode program until it suspends. Returns
    /// the cause of suspension.
    pub fn run(&mut self) -> Terminus {
        let prog = &mut self.prog;
        let nprog = prog.len();

        // The actual emulator loop tries to be careful in
        // its checking.
        let mut ip: usize = self.ip;
        while ip < nprog {
            let (op, mut opnds) = Decode::new(prog, ip, self.rel_base);
            use Opcode::*;
            ip = match op {
                Halt => {
                    self.ip = ip;
                    return Terminus::Halted;
                }
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
                    if self.inputs.is_empty() {
                        self.ip = ip;
                        return Terminus::NeedInput;
                    }
                    let input =
                        self.inputs.pop().expect("input without value");
                    opnds.store(input);
                    opnds.finish()
                }
                Output => {
                    let output = opnds.fetch();
                    ip = opnds.finish();
                    self.ip = ip;
                    return Terminus::HaveOutput(output);
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
                        if target < 0 {
                            panic!("jump target out of range");
                        }
                        target as usize
                    } else {
                        opnds.skip();
                        opnds.finish()
                    }
                }
                RBO => {
                    let offset = opnds.fetch();
                    self.rel_base += offset;
                    opnds.finish()
                }
            }
        }
        panic!("program ran off end");
    }

    /// Keep running the program until it halts, collecting
    /// any outputs produced along the way. Return them all.
    ///
    /// # Panics
    /// Will panic if program stops to request input.
    pub fn collect_outputs(&mut self) -> Vec<i64> {
        let mut outputs = Vec::new();
        loop {
            match self.run() {
                Terminus::Halted => return outputs,
                Terminus::HaveOutput(out) => outputs.push(out),
                Terminus::NeedInput => {
                    panic!("output collection stopped for input")
                }
            }
        }
    }

    /// Retrieve the value at the given address.
    ///
    /// # Panics
    /// Will panic if address is out of range.
    pub fn peek(&mut self, addr: usize) -> i64 {
        self.prog[addr]
    }

    /// Poke the given value into the given address.
    ///
    /// # Panics
    /// Will panic if address is out of range.
    pub fn poke(&mut self, addr: usize, val: i64) {
        self.prog[addr] = val;
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
            assert_eq!(init.collect_outputs(), vec![output]);
        }
    }
}

// Test Examples from the Day 9 problem.
#[test]
fn test_day09() {
    let code = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006,
        101, 0, 99,
    ];
    let mut prog = Intcode::new(code.clone());
    let outputs = prog.collect_outputs();
    assert_eq!(code, outputs);

    let code = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut prog = Intcode::new(code);
    match prog.run() {
        Terminus::HaveOutput(q) => assert_eq!(q.to_string().len(), 16),
        _ => panic!("test failed with no output"),
    }

    let code = vec![104, 1125899906842624, 99];
    let mut prog = Intcode::new(code.clone());
    match prog.run() {
        Terminus::HaveOutput(q) => assert_eq!(q, code[1]),
        _ => panic!("test failed with no output"),
    }
}
