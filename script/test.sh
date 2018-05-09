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

if [[ -z "$SPECS_ONLY" ]]; then
	run cargo build

	if [[ errors -ne 0 ]]; then
		echo -e "\nbuild failed\n"
		exit 1
	fi

	run cargo test
fi

# First run with the Rust test harness
run cargo run -- --rtjson --spec specs/rtjson/rtjson.spec
run cargo run -- --rtjson --spec specs/rtjson/bugs.spec

# Then the python test harness
run python3 script/spec_tests.py --rtjson --spec specs/rtjson/rtjson.spec
run python3 script/spec_tests.py --rtjson --spec specs/rtjson/bugs.spec

if [[ errors -ne 0 ]]; then
	echo -e "\nsome tests failed\n"
	exit 1
fi
