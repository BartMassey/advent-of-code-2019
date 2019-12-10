# Advent of Code 2019: Day 10
Bart Massey

Uggh. I tried to vidrecord this one, but it just got too
long and tedious. About 3 hours before I gave up on the
video. About 5 hours before I was really officially "done".

That's too much. I may not be able to keep going with this
game.

There were two key insights. One was to compute the "reduced
slope" (see the code) for occlusion. The other was to use
the venerable `atan2()` function to rotate the laser in Part
2.

The rest was just a hell of tedious coordinate stuff.  Why
must we work in a flipped x-y coordinate space? Why??  A
row-column coordinate space would have been perfectly
acceptable, as would a normal right-handed x-y space with y
up and the direction vector starting to the right. I spent
way too long realizing that I had a bunch of cancelling
screwups with this in my tests.

Oh well. I'm sure tomorrow will be better.

---

Solution to
[this problem](https://adventofcode.com/2019/day/10).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
