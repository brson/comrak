import os
import sys
import json

sys.path.append("target/release/")
import snoomark

if __name__ == '__main__':
    with open(sys.argv[1], "r") as f:
        md = f.read()

    snoomark.flame_exec_start()
    snoomark.flame_convert_start()

    r = snoomark.cm_to_rtjson(md)

    snoomark.flame_convert_end()
    snoomark.flame_dumps_start()

    print(json.dumps(r))

    snoomark.flame_dumps_end()
    snoomark.flame_exec_end()
    snoomark.flame_write()
