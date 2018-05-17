#!/usr/bin/env python

import sys
import json
import os
from timeit import default_timer as timer
from cm_to_rtjson import cm_to_rtjson

corpus_file = sys.argv[1]

with open(corpus_file, "r") as f:
    corpus_string = f.read()

corpus_json = json.loads(corpus_string)

total_time = 0
total_docs = 0

for item in corpus_json:
    md = item[0]

    tries = 5
    runs = []

    for i in range(0, tries):
        start = timer()
        doc = cm_to_rtjson(md)
        rendered = json.dumps(doc)
        assert sys.getrefcount(doc) == 2
        del doc
        end = timer()
        runs.append(end - start)

    runs.sort()
    assert (tries / 2 + 1 == 3)
    time = runs[tries / 2 + 1]
    total_time += time
    total_docs += 1

print("parsed {} docs".format(total_docs))
print("total time: {} s".format(total_time))
print("avg time: {} s".format(total_time / total_docs))
