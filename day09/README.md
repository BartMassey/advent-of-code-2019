# Advent of Code 2019: Day 9
Bart Massey

Ouch. Recorded this one as a video, but got the video stuff
backward and spent the whole time looking at the wrong
display. Ah well.

I'm really tired of building an Intcode machine. I hope
we're done with it as promised. I will probably skip the
reverse-engineering problem this year unless somebody on the
Internets provides tools: it's almost Christmas and I have a
life.

The joy of Rust is that the actual problem code ran on the
first try. I had some bugs in my Intcode machine hacks, but
they were all caught reasonably either by the compiler or
the runtime.

---

Solution to
[this problem](https://adventofcode.com/2019/day/9).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
