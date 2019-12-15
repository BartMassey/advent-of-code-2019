# Advent of Code 2019: Day 15
Bart Massey

Nice little Dijkstra search problem. Interesting that once
the robot finds the generator it can just start searching
again from there.

Added some tracing macros to `libaoc`, which was fun.

Part 2 is quite slow. I guess the obvious plan is to not
keep moving the robot back to the origin, and instead try to
move it directly to the nearest open location. I could even
do some fancy TSP-style motion planning. Nah. I'm sleepy.

---

Solution to
[this problem](https://adventofcode.com/2019/day/15).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

You can use `--features=tracing` to get a trace of
the robot's operation.

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
