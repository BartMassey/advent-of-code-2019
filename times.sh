#!/bin/sh
for D in day??
do
    echo $D
    ( cd $D
      if [ $D = day25 ]
      then
          echo -n "sole part: "
          /usr/bin/time -f '%e' cargo run --release <input.txt
          continue
      fi
      for PART in 1 2
      do
          CMD="`egrep \"^     *cargo run --release $PART\" README.md`"
          if [ $? -ne 0 ]
          then
              continue
          fi
          if [ $PART = 1 ]
          then
              cargo build --release >/dev/null 2>&1
          fi
          echo -n "part $PART: "
          /usr/bin/time -f '%e' sh -c "$CMD >/dev/null 2>&1" 
      done )
done
