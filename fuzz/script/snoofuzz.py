#!/usr/bin/python

import sys
import json
import snoomark

# Using the fuzz input run that command against the parser
string = sys.argv[1]
pydict = snoomark.cm_to_rtjson(string)
pydict_no_qr = snoomark.cm_to_rtjson_no_qr(string)
if pydict != pydict_no_qr:
    print "\n Found incompatible type: ", string
    print pydict
    print pydict_no_qr
    sys.exit(1);
