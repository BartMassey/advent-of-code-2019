# Advent of Code 2019: Day 12
Bart Massey

Well that hurt.

I got the trick for Part 2 right away, but I made a wrong
"physics" assumption (that I could calculate the orbit of a
given planet without respect to the others) that cost me
several hours.

There was a lot of tricky debugging in there: I was pretty
sure I knew how to compute the LCM of *n* integers, and
wrote the right thing eventually, but my attempts to debug
it weren't helping since it wasn't the real problem.

I really didn't enjoy that. I feel like the "trick" was
pretty telegraphed if you saw previous years, and yet
somehow I managed to make it into a 4-hour marathon.

I really can't keep doing these with these time demands.  I
have to either get faster or give up, I think.

---

Solution to
[this problem](https://adventofcode.com/2019/day/12).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
