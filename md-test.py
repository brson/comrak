import snoomark
import sys


if __name__ == '__main__':
    with open(sys.argv[1], "r") as f:
        md = f.read()
        print snoomark.cm_to_rtjson(md)
