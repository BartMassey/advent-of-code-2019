#!/bin/sh
for D in day??
do
    ( cd $D && cargo clean )
done
