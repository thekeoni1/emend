#!/usr/bin/env python3
"""The test of scripts/scale-gen.py (specs/0023-scale.md, P1 ruling 4).

usage: scale-gen-test.py <emend-copy> [<emend-copy> ...]

Generates tier t twice into two temporary directories and asserts the two
workbooks are the same bytes (the generator is deterministic). Then runs each
emend-copy binary given (the gate's, host and i686) over the workbook with
--survey (Package::sheets), --cells (Package::cells over every sheet) and
--shared-strings (Package::shared_strings), and asserts 2 sheets, 2500 cells
(100 rows of 24 and the 10 by 10 sheet), 1000 shared strings, exit 0, and an
output byte-identical to the input for every operation.

Prints one line per assertion, "ok" or "FAIL" then what was asserted, and a
last line "scale-gen-test: PASS" or "scale-gen-test: FAIL"; exits 0 or 1.
Never prints a cell value or a string: the tool's lines are counts.
"""
import filecmp
import os
import subprocess
import sys
import tempfile

GEN = os.path.join(os.path.dirname(os.path.realpath(__file__)), "scale-gen.py")
EXPECT = {
    "--survey": ("survey", {"sheets": "2", "sheets_resolved": "2", "shared_strings": "1"}),
    "--cells": ("cells", {"sheets": "2", "cells": "2500", "unresolved": "0", "max_row": "100", "max_col": "24"}),
    "--shared-strings": ("strings", {"items": "1000"}),
}

failed = 0


def check(ok, what):
    global failed
    print("%s %s" % ("ok" if ok else "FAIL", what))
    if not ok:
        failed += 1


def generate(target):
    p = subprocess.run([sys.executable, GEN, "t", target], capture_output=True, text=True)
    check(p.returncode == 0 and p.stderr == "", "tier t generated into a temporary directory (exit %d)" % p.returncode)
    fields = p.stdout.split()
    check(len(fields) == 7 and fields[:2] == ["generated", "t"], "the generator printed one generated line for tier t")
    ident = fields[2] if len(fields) > 2 else ""
    return os.path.join(target, "corpus", ident + ".xlsx"), ident


def main(argv):
    tools = argv[1:]
    if not tools:
        print("usage: scale-gen-test.py <emend-copy> [<emend-copy> ...]", file=sys.stderr)
        return 2
    with tempfile.TemporaryDirectory() as one, tempfile.TemporaryDirectory() as two:
        first, ident = generate(one)
        second, ident2 = generate(two)
        check(ident == ident2 and os.path.isfile(first) and filecmp.cmp(first, second, shallow=False),
              "two generations of tier t are the same bytes (id %s)" % ident)
        for tool in tools:
            name = "i686" if "i686-unknown-linux-musl" in tool.split(os.sep) else "host"
            for op, (word, keys) in EXPECT.items():
                out = os.path.join(one, "out.xlsx")
                if os.path.exists(out):
                    os.remove(out)
                p = subprocess.run([tool, first, out, op], capture_output=True, text=True)
                check(p.returncode == 0 and p.stderr == "", "%s %s exit 0, no stderr (exit %d)" % (name, op, p.returncode))
                lines = p.stdout.splitlines()
                pairs = dict(f.split("=", 1) for f in lines[0].split("\t")[1:]) if len(lines) == 1 else {}
                check(len(lines) == 1 and lines[0].split("\t")[0] == word, "%s %s printed one %s line" % (name, op, word))
                for k, v in keys.items():
                    check(pairs.get(k) == v, "%s %s %s=%s (read %s)" % (name, op, k, v, pairs.get(k)))
                check(os.path.isfile(out) and filecmp.cmp(first, out, shallow=False),
                      "%s %s output byte-identical to the input" % (name, op))
    print("scale-gen-test: %s" % ("FAIL" if failed else "PASS"))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
