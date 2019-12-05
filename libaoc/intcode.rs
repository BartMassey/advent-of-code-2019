/// Intcode is this year's virtual machine.
#[derive(Debug, Clone)]
pub struct Intcode(Vec<usize>);

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

        // Give a more comprehensible panic rather than an
        // array bounds panic.
        let check_range = |loc| {
            if loc >= nprog {
                panic!("operand out of range");
            }
            loc
        };

        // Our opcodes are implemented by functions that
        // perform the operations.
        type Op = &'static dyn Fn(usize, usize) -> usize;
        let add: Op = &|a, b| a + b;
        let mul: Op = &|a, b| a * b;
        let ops = [add, mul];

        // The actual emulator loop tries to be careful in
        // its checking.
        let mut ip = 0;
        while ip < nprog && prog[ip] != 99 {
            if ip + 3 > nprog {
                panic!("program ran off end");
            }
            let opcode = prog[ip];
            if opcode < 1 || opcode > ops.len() + 1 {
                panic!("illegal opcode {} at {}", opcode, ip);
            }
            let op = ops[opcode - 1];
            let src1 = check_range(prog[ip + 1]);
            let src2 = check_range(prog[ip + 2]);
            let dst = check_range(prog[ip + 3]);
            prog[dst] = op(prog[src1], prog[src2]);
            ip += 4;
        }
    }

    // For 2019 Day 02, this is how you input values.
    pub fn input(&mut self, v1: usize, v2: usize) {
        self.0[1] = v1;
        self.0[2] = v2;
    }

    // For 2019 Day 02, this is the output value.
    pub fn output(&self) -> usize {
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
    let testcases: &[(&[usize], &[usize])] = &[
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
