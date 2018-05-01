#!/bin/bash

# Grab the release version of snoomark and run it

FILE=./target/release/libsnoomark.dylib


cp $FILE ./snoomark.so
python md-test.py $1

