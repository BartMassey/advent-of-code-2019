# Advent of Code 2019: Day 6
Bart Massey

Welp. Easy problem, I think, but I found lots of ways to
fail.

I submitted my first wrong answer this season. The
`count_transfers()` test actually passed, so I figured I was
good to go, in spite of a ridiculously small-looking answer
on the input data. Turns out I had called `x.len()` on an
identification string `x` instead of the path `x` I thought
I was calling it on: the typechecker didn't save me. Sadly,
the length of the strings was just perfect for giving the
wrong answer. Horrible to debug. I have gotten used to the
compiler refusing if I've done something like this: can't
always rely on it, so need to read my code more carefully.

Then I was fooling around with logins on the Advent of Code
site, and didn't realize I'd created and logged into a new
account. So my Part 1 answer was not submitted anywhere that
mattered. Disappointing. I relogged and submitted it, but my
leaderboard status has been shaken. Not that I much care:
this is for the fun of it, not for proving anything.

This is the first problem I have used a nonstandard
third-party crate on: the problem really cried out for
multisets and I didn't see any reason to write my own.

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
