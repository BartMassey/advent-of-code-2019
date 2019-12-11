# Advent of Code 2019: Day 11
Bart Massey

That was quick and reasonably fun.

Debugging was a little tricky; it would have been nice to
have a small sample paintbot program provided to test with.

Some use of a macro to shorten things a bit, which is
interesting. I have moved the `render()` and
`bounding_box()` functions (who even remembers why `box` is
a keyword? I do, but yeesh) I wrote today into `libaoc`, so
you can find them there if you're interested.

---

Solution to
[this problem](https://adventofcode.com/2019/day/11).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
