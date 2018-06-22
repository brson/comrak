#!/usr/bin/python
# python3 script/spec_tests.py --rtjson --roundtrip --spec specs/rtjson/rtjson.spec

import re
import sys, os
import json
# from spec_tests import get_tests, do_test
from cmark import CMark
import argparse

from cm_to_rtjson import cm_to_rtjson
import rtjson as rtjson_py
import re

def md_to_rtj(cmark):
    return cm_to_rtjson(cmark)

def rtj_to_md(rtj):
    return rtjson_py.cmark(rtj)[0]

def converter(md_input, exts):
    fname = "diff.txt"
    retcode = 0

    if os.path.exists(fname):
        with open(fname, 'r') as f:
            data = json.load(f)
    else:
        data = {}
        data["rtj_to_md"] = list()
        data["md_to_rtj"] = list()

    # Gen 1 RTJson
    rtj_gen1 = md_to_rtj(md_input)
    try:
        cm_gen1 = rtj_to_md(rtj_gen1)
        rtj_ex = None
    except Exception as e:
        cm_gen1 = None
        rtj_ex = e
        retcode = 1

    rtj_gen2 = {}
    cm_gen2 = ""
    if not rtj_ex:
        rtj_gen2 = md_to_rtj(cm_gen1)
        cm_gen2 = rtj_to_md(rtj_gen2)

        # The second rtj should be the same as the first rtj
        if rtj_gen1 != rtj_gen2:
            data["md_to_rtj"].append({
                "input": md_input,
                "gen2": cm_gen2,
                "rtj_1": rtj_gen1,
                "rtj_2": rtj_gen2,
            })
            with open(fname, 'w') as f:
                json.dump(data, f, indent=4)
            retcode = 1
        elif cm_gen1 != cm_gen2:
            # The second cm produced by rtjson.py should be the same as the first
            data["rtj_to_md"].append({
                "input": md_input,
                "gen2": cm_gen2,
                "rtj_1": rtj_gen1,
                "rtj_2": rtj_gen2,
            })
            with open(fname, 'w') as f:
                json.dump(data, f, indent=4)
            retcode = 1
    else:
        print rtj_ex

    if retcode != 0:
        sys.exit(retcode)
    return rtj_gen2

if __name__ == "__main__":
    file_ = sys.argv[1]

    pretty = False
    flame = False
    bench = False

    md = []
    with open(file_, "r") as f:
        md = f.read()

    doc = converter(md, [])
    rendered = json.dumps(doc)
    print(rendered)
