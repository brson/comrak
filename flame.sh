#!/bin/bash

# Grab the release version of snoomark and run it

FILE=./target/release/libsnoomark.so


cp $FILE ./snoomark.so
python md-test.py $1

