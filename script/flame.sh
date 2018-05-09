#!/bin/bash

set -e

# Grab the release version of snoomark and run it

FILE=./target/release/libsnoomark.so


cp -f $FILE ./target/release/snoomark.so
python script/flame-test.py $1

