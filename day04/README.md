# Advent of Code 2019: Day 4
Bart Massey

This felt easier than some previous puzzles. My current
solution is pretty slow, so that's probably why.

A much faster solution would be to take care of all the
matching while incrementing the characters of the target
integer directly. This would allow skipping large sections
of the search space, and would avoid using expensive
int-to-string conversion at each iteration. It's harder and
more work, I think, so I'll leave it as an exercise for the
interested reader.

For some reason I am using a lot more functional-programming
stuff in my Rust this year.

This is the first puzzle with no text file for input. For
these, I will simple paste the given input into a text file
manually; it works better with my machinery and is more
consistent.

Run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
