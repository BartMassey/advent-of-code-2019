#!/bin/sh
for D in day??
do
    ( cd $D && cargo clean )
done
if [ "$1" = '-a' ]
then
    ( cd libaoc && cargo clean )
fi
