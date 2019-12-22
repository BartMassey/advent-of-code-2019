# Advent of Code 2019: Day 14
Bart Massey

This problem had me stymied for the longest time. Turns out
the key is to topo-sort the products and then traverse them
from FUEL back to ORE keeping track of demand. I think this
only works if there is only one way to make each product and
the graph is a DAG: otherwise some sort of search is going
to be needed and maybe harder things.

There is a gratuitous amount of machinery in this solution:
it could be done smaller and cleaner.

Thanks to Keith Packard for help thinking through the
problem one more time, to folks on the Internet for
sugesting binary search for Part 2, and to the author of the
quite nice `topological-sort` crate.

---

Solution to
[this problem](https://adventofcode.com/2019/day/14).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
