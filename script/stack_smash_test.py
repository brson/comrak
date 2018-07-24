#!/usr/bin/python2
#
# Just a single test that cm doesn't smash the stack
# that doesn't fit into the normal spec-test framework.

import sys
import json
import os
from cm_to_rtjson import cm_to_rtjson_unvalidated

big = '>' * 150000
cm_to_rtjson_unvalidated(big)
