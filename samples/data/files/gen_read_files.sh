#!/bin/bash

if [ -z "$1" ]; then
    echo "No directory provided"
    exit 1
else
    echo "Directory provided: $1"
fi

d=$1

mkdir -p $d

for x in {0..5}; do
  k=$(( 2 ** $x ))
  t=$(( $k * 1024 ))
  dd if=/dev/urandom of=$d/file_$t.bin bs=$k count=1024
  echo ""
done
