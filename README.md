# Advent Of Code 2019: Tutorial Solutions in Rust
Copyright (c) 2019 Bart Massey

Herein lie Rust solutions to at least some of the problems
of the 2019
[Advent of Code](http://adventofcode.com). Advent of Code is
a fantastic exercise, and I thank the author and others
involved profusely for their excellent work. Thanks also to
`relsqui` for pointing me at this back in 2015.

The solutions are in directories named `day01` through
`day25`. For each solution, I have included cleaned-up Rust
code. There is a `README.md` in every problem directory
containing algorithm descriptions, comments and usage
instructions. I have also included the problem descriptions
(`part1.md` and `part2.md`) and my specific `input.txt` for
posterity.

The solutions load library code from the included `libaoc`
crate. See its documentation for details.

There are no special system tests written for this code
other than the ones provided as part of the problem ---
there are occasional unit tests. I regard passing both parts
of a day's problem as strong validation, although I've been
wrong about this in the past. More tests should get written.

These programs are not production-quality: it is considered
acceptable to panic on erroneous input.

The goals of these solutions are to:

* Provide correct solutions with reasonable runtimes.

* Illustrate reasonable solution strategies.

* Illustrate the use of Rust in problem-solving.

As always I expect to learn some Rust and a little bit of
software engineering I should already have known writing
these.

There's also some engineering infrastructure in the form of
the `template` directory and the `mkday.sh` and
`process-aoc.sh` shell scripts.  These speed each day's
setup considerably. At the beginning of each day I `sh
mkday.sh`. (The day number is tracked automatically but can
be overwritten on the command line.) At the end of the
day I select and copy the page source of the day's AoC
webpage and then

    xclip -selection CLIPBOARD -out | sh ../process-aoc.sh

to get markdown into the problem files for posterity.

You can get times for all parts of all days with `sh
times.sh` (will build before timing). This also verifies
that everything runs.  You can use `sh clean.sh` to run
`cargo clean` in the day directories — Rust `target`
directories are huge.

These solutions deserve a much more thorough top-level
description than I usually have the energy to
write. Apologies.

---

This work is licensed under the "MIT License".  Please see
the file `LICENSE` in the source distribution of this
software for license terms.
