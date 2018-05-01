#!/bin/bash

# Grab the release version of snoomark and run it

FILE=./target/release/libsnoomark.dylib

if [ $FILE does not exist ]; then
  cargo build --release
fi

cp $FILE ./snoomark.so
python md-test.py $1

