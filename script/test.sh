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
    cargo_build_arg = "--release"
else
    export SM_TARGET="debug"
    cargo_build_arg = "--debug"
fi

if [[ -z "$SPECS_ONLY" ]]; then
	run cargo build "$cargo_build_arg"

	if [[ errors -ne 0 ]]; then
		echo -e "\nbuild failed\n"
		exit 1
	fi

	run cargo test "$cargo_build_arg"
fi

# Then the python test harness
run python3 script/spec_tests.py --rtjson --spec specs/rtjson/rtjson.spec
run python3 script/spec_tests.py --rtjson --spec specs/rtjson/bugs.spec

if [[ errors -ne 0 ]]; then
	echo -e "\nsome tests failed\n"
	exit 1
fi
