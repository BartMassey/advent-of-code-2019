# Advent of Code 2019: Day 7
Bart Massey

Geez this was tedious.

Sometimes there its just *one sentence* I miss in the
problem description and it costs me hours of fail. In this
case, it was that none of the amplifier parameters could be
equal: needed a permutation of 0..4.

The other thing is that I have guessed three times what the
"final form" of I/O will be for this thing, and I have been
wrong three times. Each time I've had to restructure my I/O
code. May this be the last time.

I used the `permutohedron` crate for permutations
today. Didn't feel like writing this one myself.

There's some functions-as-values in my code today. That
said, I would love to generalize some of the types more…

---

Solution to
[this problem](https://adventofcode.com/2019/day/).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
