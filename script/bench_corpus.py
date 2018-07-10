#!/usr/bin/env python

import sys
import json
import os
from timeit import default_timer as timer
from cm_to_rtjson import cm_to_rtjson
from math import sqrt

corpus_file = sys.argv[1]

with open(corpus_file, "r") as f:
    corpus_string = f.read()

corpus_json = json.loads(corpus_string)

# TODO: Is this sample size enough?
tries = 10
times = []

for i in range(0, tries):
    start = timer()

    for item in corpus_json:
        md = item[0]

        doc = cm_to_rtjson(md)
        rendered = json.dumps(doc)
        assert sys.getrefcount(doc) == 2
        del doc

    end = timer()
    total_time = end - start
    times += [total_time]

times.sort()

times = times[1:-1]

total_time = sum(times)
samples = len(times)
mean_time = total_time / samples
variance = sum((x - mean_time) ** 2 for x in times) / samples
stddev = sqrt(variance)

print("parsed {} docs".format(len(corpus_json)))
print("samples: {}".format(len(times)))
print("mean time: {:.6} s".format(mean_time))
print("variance: {:.6} s^2".format(variance))
print("std dev: {:.6} s".format(stddev))
