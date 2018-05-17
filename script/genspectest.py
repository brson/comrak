#!/usr/bin/python
#
# Parse a markdown file to rtjson and output the results
# in two formats, the first suitable for copying into a spec test,
# the second for visual inspection.

import sys
import json
import os
from cm_to_rtjson import cm_to_rtjson

file_ = sys.argv[1]

s = []
with open(file_, "r") as f:
    s = f.read()
    
r = cm_to_rtjson(s)
print "```````````````````````````````` example"
print s
print "."
print json.dumps(r)
print "````````````````````````````````"
print "$$$$"
print json.dumps(r, indent=4)
print "$$$$"
