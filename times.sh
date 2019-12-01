#!/bin/sh
for D in day??
do
    echo $D
    ( cd $D
      cargo build --release >/dev/null
      if [ $D = day25 ]
      then
          echo -n "sole part: "
          /usr/bin/time -f '%e' cargo run --release <input.txt
          continue
      fi
      for PART in 1 2
      do
          echo -n "part $PART: "
          egrep "^     *cargo run --release $PART" README.md |
          ( read CMD; /usr/bin/time -f '%e' sh -c "$CMD" )
      done )
done
