#!/bin/bash

# Run all tests suites and report any errors

errors=0

run() {
	echo "running $*"
	$*
	if [[ $? -ne 0 ]]; then
	   errors=1
	fi
	echo
}

if [[ "$1" == "--release" ]]; then
    export SM_TARGET="release"
    cargo_build_arg="--release"
else
    export SM_TARGET="debug"
    cargo_build_arg=""
fi

if [[ -z "$SPECS_ONLY" ]]; then
	run cargo build "$cargo_build_arg"

	if [[ errors -ne 0 ]]; then
		echo -e "\nbuild failed\n"
		exit 1
	fi
fi

run python ./fuzz/script/setup.py
run cargo +nightly fuzz run snoofuzz
