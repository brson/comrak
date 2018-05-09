#!/usr/bin/python

# This is a multi-purpose script to bind and test the Rust cm_to_rtjson function.
#
# It locates and loads the snoomark dylib, exposes the cm_to_rtjson function,
# and, when run as the main function prints the rtjson to stdout, or alternately
# generates flamegraphs, or captures timings for benchmarking.
#
# Basic use:
#
#     script/cm_to_rtjson.py [FILE]
#
#     script/cm_to_rtjson.py [FILE] --pretty
#
# The first will parse and print the document, the second will print with
# readable indentation.
#
# The script uses two environment variables to locate the dylib in the cargo
# target directory, SM_DIR, and SM_TARGET. By default SM_DIR is "." and
# SM_TARGET is "debug". To use the release binary you can run
#
#     SM_TARGET=release script/cm_to_rtjson.py [FILE]
#
# When run as main, the --flame argument will generate a flamegraph in
# flamegraph.html, or the --bench argument will parse the document multiple
# times and print timing info.

import sys
import json
import os
from timeit import default_timer as timer

# Use the environment to configure how to find the dylib
root_dir = os.environ.get("SM_DIR")
target = os.environ.get("SM_TARGET")
if not root_dir:
    root_dir = "."
if not target:
    target = "debug"

target_dir = "{}/target/{}".format(root_dir, target)
mac_dylib = "{}/libsnoomark.dylib".format(target_dir)
linux_dylib = "{}/libsnoomark.so".format(target_dir)

if os.path.exists(mac_dylib):
    dylib = mac_dylib
elif os.path.exists(linux_dylib):
    dylib = linux_dylib
else:
    raise Exception("Unable to find snoomark dylib")

# Symlink the snoomark dylib to snoomark.so and load the module
snoomark_so = "{}/snoomark.so".format(target_dir)
if os.path.lexists(snoomark_so):
    os.unlink(snoomark_so)

os.symlink(os.path.abspath(dylib), snoomark_so)
sys.path.append(target_dir)
import snoomark

# Export the cm_to_rtjson function for other modules
def cm_to_rtjson(s):
    return snoomark.cm_to_rtjson(s)

if __name__ == "__main__":
    file_ = sys.argv[1]

    if len(sys.argv) > 2:
        pretty = True if sys.argv[2] == "--pretty" else False;
        flame = True if sys.argv[2] == "--flame" else False;
        bench = True if sys.argv[2] == "--bench" else False;
    else:
        pretty = False
        flame = False
        bench = False

    md = []
    with open(file_, "r") as f:
        md = f.read()

    # If profiling then run it through once to trigger all the
    # lazy initializers
    if flame or bench:
        r = cm_to_rtjson(md)
        json.dumps(r) # maybe dumps needs to be initialized? just being cautious
        snoomark.flame_clear()
        del r

    # Run some number of times so that we're processing around N bytes
    repeat = 1 if not bench else 1000000 / len(md)
    assert repeat >= 1

    if bench:
        print "running {} times".format(repeat)
        print "processing {} bytes".format(repeat * len(md))

    bench_runs = []

    for i in range(0, repeat):

        start = timer()

        if flame:
            snoomark.flame_exec_start()
            snoomark.flame_convert_start()

        doc = cm_to_rtjson(md)

        if flame:
            snoomark.flame_convert_end()
            snoomark.flame_dumps_start()

        if not pretty:
            rendered = json.dumps(doc)
        else:
            rendered = json.dumps(doc, indent=4)

        if not bench and not flame:
            print(rendered)

        if flame:
            snoomark.flame_dumps_end()

        if flame:
            snoomark.flame_del_start()

        # Delete and release doc. Refcount is two: the local binding plus the
        # argument binding to getrefcount
        assert sys.getrefcount(doc) == 2
        del doc

        if flame:
            snoomark.flame_del_end()
            snoomark.flame_exec_end()
            snoomark.flame_write()

        end = timer()
        bench_runs.append(end - start)

    if bench:
        # Remove top and bottom outliers
        to_remove = repeat / 4
        assert repeat >= 1
        assert len(bench_runs) > to_remove * 3
        bench_runs.sort()
        bench_runs = bench_runs[:len(bench_runs) - to_remove]
        bench_runs = bench_runs[to_remove:]
        print "using {} runs".format(len(bench_runs))
        bench_sum = sum(bench_runs)
        avg_time = bench_sum / len(bench_runs)
        print "avg time: {} s".format(avg_time)
