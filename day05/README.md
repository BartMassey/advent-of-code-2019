# Advent of Code 2019: Day 5
Bart Massey

OK, build more of the virtual machine. Ugh, but expected.
Almost all of the work was done in `libaoc/intcode.rs`,
which I split out from [Day 2](../day02).

My Intcode interpreter implements a custom iterator and a
builder and does pretty thorough runtime checks. It's worth
looking at, maybe. Sadly, it was convenient to flip the
fancy array of op functions back into a big `match`.

I hope the next few days aren't quite so much tedious
typing.

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
