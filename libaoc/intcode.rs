//! Intcode interpreter for Advent of Code 2019 solutions.
//!
//! This is a fairly standard pcode setup with the exception
//! of being highly self-modifying code. Use
//! `Intcode::read()` to load a program from `stdin`;
//! `input()` to load input values; `run()` to run until
//! halted; `output()` to get the output value.

#[derive(Clone, Copy)]
enum Opcode {
    Add,
    Mul,
}

impl Opcode {
    fn new(code: usize) -> (Self, usize) {
        let codes = [(Self::Add, 3), (Self::Mul, 3)];
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

struct OpndModes {
    modebits: usize,
    count: usize,
}

impl OpndModes {
    fn new(opcode: usize, count: usize) -> Self {
        Self { modebits: opcode / 100, count }
    }

    fn end(&self) {
        if self.modebits != 0 {
            panic!("extra mode bits in operand mode");
        }
        if self.count != 0 {
            panic!("unused operand modes");
        }
    }
}

impl Iterator for OpndModes {
    type Item = OpndMode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            panic!("missing operand mode");
        }
        self.count -= 1;
        let mode = OpndMode::new(self.modebits % 10);
        self.modebits /= 10;
        Some(mode)
    }
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

#[derive(Debug, Clone)]
pub struct Intcode(Vec<isize>);

impl Intcode {
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
        Self(prog)
    }

    /// Run this Intcode program.
    pub fn run(&mut self) {
        let prog = &mut self.0;
        let nprog = prog.len();

        let fetch = |prog: &Vec<isize>, idx, modes: &mut OpndModes| {
            if idx > nprog {
                panic!("fetch off program end");
            }
            let opnd = prog[idx];
            match modes.next().unwrap() {
                OpndMode::Imm => opnd,
                OpndMode::Pos => {
                    if opnd < 0 || opnd as usize >= nprog {
                        panic!("fetch out of range");
                    }
                    prog[opnd as usize]
                }
            }
        };

        let store = |prog: &mut Vec<isize>, idx, modes: &mut OpndModes, val| {
            if idx > nprog {
                panic!("store off program end");
            }
            let opnd = prog[idx];
            match modes.next().unwrap() {
                OpndMode::Imm => panic!("immediate-mode store"),
                OpndMode::Pos => {
                    if opnd < 0 || opnd as usize >= nprog {
                        panic!("store out of range");
                    }
                    prog[opnd as usize] = val;
                }
            }
        };

        // The actual emulator loop tries to be careful in
        // its checking.
        let mut ip = 0;
        while ip < nprog && prog[ip] != 99 {
            if ip >= nprog {
                panic!("program ran off end");
            }
            let opcode = prog[ip] as usize;
            let (op, nargs) = Opcode::new(opcode % 100);
            match op {
                Opcode::Add | Opcode::Mul => {
                    assert_eq!(nargs, 3);
                    let mut modes = OpndModes::new(opcode, nargs);
                    let src1 = fetch(prog, ip + 1, &mut modes);
                    let src2 = fetch(prog, ip + 2, &mut modes);
                    let a = match op {
                        Opcode::Add => src1 + src2,
                        Opcode::Mul => src1 * src2,
                    };
                    store(prog, ip + 3, &mut modes, a);
                    modes.end();
                }
            }
            ip += nargs + 1;
        }
    }

    /// Input a "verb" and "noun" before starting the program.
    pub fn input(&mut self, verb: isize, noun: isize) {
        self.0[1] = verb;
        self.0[2] = noun;
    }

    /// Fetch output after running the program.
    pub fn output(&self) -> isize {
        self.0[0]
    }
}

// The test examples given in the problem.
#[test]
fn test_run() {
    // We generally want whatever rustfmt does in our code.
    // In this case it gows things up trying to be clever.
    // So we tell it "no".
    #[rustfmt::skip]
    let testcases: &[(&[isize], &[isize])] = &[
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
        let mut init = Intcode(init.to_vec());
        let fin = fin.to_vec();
        init.run();
        assert_eq!(init.0, fin);
    }
}
