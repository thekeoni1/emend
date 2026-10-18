#!/usr/bin/env python3
"""The test of scripts/compare-record.py (specs/0037-measurement-and-close.md
P2, "the comparison record by scripts/compare-record.py with its test").

usage: compare-record-test.py

Builds fictional run directories in a temporary directory, in the forms
scripts/passthrough-run.sh and scripts/compare-run.sh write (summary, run and
empty-run files; one stdout file for an error row), for the eleven ops on a
corpus of three made-up ids and for the scale tiers, runs compare-record.py
on them, and asserts the record's rows: the outcome, entry and open-check
counts, the medians, sums and maxima, the openpyxl row's lo_in taken from
the emend host run (ruling C), the container start median (rulings K and P),
the exception class of the error row, the scale rows, and the image record's
sha256. Then each refusal: a missing run directory, a missing empty-run file,
a summary with another header, a run row naming another op, a scale run row
naming another op, runs that do not cover the same workbooks, an existing
record, a bad stamp. The scale settings sentence is read from the scale
runs' run.openpyxl.tsv (ruling U of the planning verification of P2), shown
on the fixture's values and on a fixture whose runs hold two.

Prints one line per assertion, "ok" or "FAIL" then what was asserted, and a
last line "compare-record-test: PASS" or "compare-record-test: FAIL"; exits 0
or 1. Everything it writes is fictional.
"""
import hashlib
import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.realpath(__file__))
SCRIPT = os.path.join(HERE, "compare-record.py")
STAMP = "2026-01-01-abcdef0"
OPS = ("roundtrip", "read", "edit", "format", "insert-rows", "names", "table",
       "comment", "image", "add-chart", "copy")
IDS = ("aaaaaaaaaaa1", "aaaaaaaaaaa2", "aaaaaaaaaaa3")
SUMMARY_HEADER = ("id\tlabel\tproducer\ttags\toutcome\texit\tsignal\twall_s\tpeak_rss_kb\tin_bytes\tout_bytes\t"
                  "diff_exit\tidentical\trecompressed\tchanged\tdropped\tadded\tflagged\torder_preserved\t"
                  "comment_equal\tfiles_identical\tleftover\tlo\tlo_in")
EMEND_RUN = ("commit\tstarted\tended\trustc\tzip\ttimeout_s\tlo_timeout_s\tmemory_mib\tpassthrough_sha256\t"
             "xlsx_diff_sha256\tfiles\ttool_args")
OX_RUN = ("commit\tstarted\tended\timage_id\topenpyxl\ttimeout_s\tlo_timeout_s\tmemory_mib\tops_sha256\t"
          "xlsx_diff_sha256\tfiles\top")

failed = 0


def check(ok, what):
    global failed
    print("%s %s" % ("ok" if ok else "FAIL", what))
    if not ok:
        failed += 1


def row(id_, label, outcome, wall, rss, counts, lo, lo_in):
    ident, recomp, changed, dropped, added = counts
    return "\t".join([id_, label, "made-up", "test", outcome, "0" if outcome == "ok" else "1", "", wall, rss,
                      "100", "90" if outcome == "ok" else "", "1" if outcome == "ok" else "",
                      ident, recomp, changed, dropped, added, "0", "yes", "yes", "no", "no", lo, lo_in])


def write(path, lines):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        handle.write("\n".join(lines) + "\n")


def emend_dir(root, name, ids, label_rows):
    for label in ("host", "i686"):
        write(os.path.join(root, name, "summary.%s.tsv" % label), [SUMMARY_HEADER] + label_rows(label))
        write(os.path.join(root, name, "run.%s.tsv" % label),
              [EMEND_RUN, "c\ts\te\trustc\t2\t300\t600\t2048\tp\tx\t%d\t" % len(ids)])


def ox_dir(root, name, op, rows, memory, empty=("0.50", "0.40", "0.70")):
    write(os.path.join(root, name, "summary.openpyxl.tsv"), [SUMMARY_HEADER] + rows)
    write(os.path.join(root, name, "run.openpyxl.tsv"),
          [OX_RUN, "c\ts\te\timg\t3.1.5\t600\t600\t%s\to\tx\t%d\t%s" % (memory, len(rows), op)])
    if empty is not None:
        write(os.path.join(root, name, "empty.openpyxl.tsv"),
              ["run\twall_s"] + ["%d\t%s" % (i + 1, t) for i, t in enumerate(empty)])


def build(base):
    corpus = os.path.join(base, "corpus-runs")
    scale = os.path.join(base, "scale-runs")
    for op in OPS:
        def emend_rows(label):
            return [row(IDS[0], label, "ok", "1.00", "1000", ("5", "0", "1", "0", "0"), "ok", "ok"),
                    row(IDS[1], label, "ok", "2.00", "3000", ("4", "1", "0", "0", "0"), "ok", "fail"),
                    row(IDS[2], label, "ok", "4.00", "2000", ("3", "0", "0", "0", "1"), "ok", "ok")]
        emend_dir(corpus, "%s-%s" % (STAMP, op), IDS, emend_rows)
        ox = [row(IDS[0], "openpyxl", "ok", "3.00", "40000", ("0", "0", "4", "2", "3"), "ok", "-"),
              row(IDS[1], "openpyxl", "error", "1.50", "38000", ("", "", "", "", ""), "-", "-"),
              row(IDS[2], "openpyxl", "ok", "5.00", "50000", ("1", "0", "3", "1", "2"), "fail", "-")]
        name = "%s-openpyxl-%s" % (STAMP, op)
        ox_dir(corpus, name, op, ox, "4096")
        write(os.path.join(corpus, name, IDS[1], "stdout.openpyxl.txt"),
              ["KeyError", "openpyxl=3.1.5\tpeak_rss_kb=38000"])
    for tier, ops in (("a", OPS), ("b", OPS), ("c", ("roundtrip",)), ("d", ("roundtrip",))):
        for op in ops:
            sid = "bbbbbbbbbbb" + tier
            emend_dir(scale, "%s-scale-%s-%s" % (STAMP, tier, op), (sid,),
                      lambda label: [row(sid, label, "ok", "9.50", "7000", ("6", "0", "1", "0", "0"),
                                         "ok" if tier == "a" else "-", "ok" if tier == "a" else "-")])
            outcome = "cap" if tier in ("c", "d") else "ok"
            ox_dir(scale, "%s-openpyxl-scale-%s-%s" % (STAMP, tier, op), op,
                   [row(sid, "openpyxl", outcome, "60.00", "900000",
                        ("0", "0", "5", "1", "2") if outcome == "ok" else ("", "", "", "", ""),
                        "ok" if tier == "a" and outcome == "ok" else "-", "-")], "16384")
    record = os.path.join(base, "openpyxl-image-2026-01-01.md")
    write(record, ["# a made-up image record"])
    return corpus, scale, record


def run(args):
    proc = subprocess.run([sys.executable, "-I", "-B", SCRIPT] + args, capture_output=True, text=True)
    return proc.returncode, proc.stdout, proc.stderr


def table_rows(text, heading):
    lines = text.split("\n")
    i = lines.index(heading)
    rows = []
    for line in lines[i + 1:]:
        if line.startswith("## "):
            break
        if line.startswith("| ") and not line.startswith("| ---"):
            rows.append([c.strip() for c in line.strip("|").split("|")])
    return rows[0], rows[1:]


def main():
    with tempfile.TemporaryDirectory() as base:
        corpus, scale, record = build(base)
        out = os.path.join(base, "out")
        os.makedirs(out)
        common = ["--corpus-runs", corpus, "--scale-runs", scale, "--stamp", STAMP, "--image-record", record, "--out", out]
        rc, so, se = run(common)
        check(rc == 0 and se == "", "the record is written, exit 0, nothing on stderr")
        check(so.strip() == "wrote compare-m5p6-%s.md: ops 11; scale rows 72" % STAMP, "the one line names the stem, 11 ops and 72 scale rows")
        text = open(os.path.join(out, "compare-m5p6-%s.md" % STAMP), encoding="utf-8").read()
        head, rows = table_rows(text, "## The corpus, per op")
        check(len(rows) == 33, "the corpus table holds 33 rows, three per op")
        by = {(r[0], r[1], r[2]): dict(zip(head, r)) for r in rows}
        h = by[("edit", "emend", "host")]
        check(h["files"] == "3" and h["ok"] == "3" and h["error"] == "0", "emend host: 3 files, 3 ok")
        check((h["identical"], h["recompressed"], h["changed"], h["dropped"], h["added"]) == ("12", "1", "1", "0", "1"),
              "emend host: the entry totals 12, 1, 1, 0, 1")
        check(h["lo ok"] == "3" and h["lo_in ok"] == "2" and h["lo_in fail"] == "1", "emend host: lo ok 3; lo_in ok 2, fail 1")
        check((h["wall median s"], h["wall sum s"], h["peak RSS median kB"], h["peak RSS max kB"]) == ("2", "7", "2000", "3000"),
              "emend host: wall median 2, sum 7; peak RSS median 2000, max 3000")
        x = by[("edit", "openpyxl", "openpyxl")]
        check(x["ok"] == "2" and x["error"] == "1", "openpyxl: 2 ok, 1 error")
        check((x["identical"], x["changed"], x["dropped"], x["added"]) == ("1", "7", "3", "5"),
              "openpyxl: the entry totals over its ok rows 1, 7, 3, 5")
        check(x["lo ok"] == "1" and x["lo fail"] == "1" and x["lo not-run"] == "1", "openpyxl: lo ok 1, fail 1, not-run 1")
        check(x["lo_in ok"] == "2" and x["lo_in fail"] == "1" and x["lo_in not-run"] == "0",
              "openpyxl: lo_in counted from the emend host run, ok 2 and fail 1 (ruling C)")
        check((x["wall median s"], x["wall sum s"], x["peak RSS median kB"], x["peak RSS max kB"]) == ("3", "9.50", "40000", "50000"),
              "openpyxl: wall median 3, sum 9.50; peak RSS median 40000, max 50000")
        dhead, drows = table_rows(text, "## The openpyxl runs: settings, container start and error classes")
        d = {r[0]: dict(zip(dhead, r)) for r in drows}
        check(len(drows) == 11, "the openpyxl detail table holds 11 rows")
        check(d["edit"]["container start median s"] == "0.50", "the container start median of 0.40, 0.50, 0.70 is 0.50 (rulings K and P)")
        check(d["edit"]["settings"] == "timeout 600 s; memory 4096 MiB", "the settings named from run.openpyxl.tsv (ruling N)")
        check(d["edit"]["error rows by exception class"] == "KeyError 1", "the error row's class read from its stdout file")
        check(d["edit"]["lo_in from"] == "%s-edit" % STAMP, "the lo_in source directory named (ruling C)")
        shead, srows = table_rows(text, "## The scale tiers")
        check(len(srows) == 72, "the scale table holds 72 rows: 24 ops over tiers a to d, three rows each")
        s = {(r[0], r[1], r[2], r[3]): dict(zip(shead, r)) for r in srows}
        sc = s[("c", "roundtrip", "openpyxl", "openpyxl")]
        check(sc["outcome"] == "cap" and sc["identical"] == "-" and sc["container start median s"] == "0.50",
              "a scale cap row: outcome cap, no entry counts, the container start named")
        sa = s[("a", "copy", "emend", "i686")]
        check(sa["outcome"] == "ok" and sa["wall_s"] == "9.50" and sa["lo"] == "ok" and sa["container start median s"] == "-",
              "a scale emend row: ok, its wall time, lo ok on tier a, no container start")
        check(s[("b", "copy", "emend", "host")]["lo"] == "-", "tier b records lo not-run as -")
        check("openpyxl under --memory 16384 --timeout 600, emend" in text,
              "the scale settings sentence read from the scale runs' run.openpyxl.tsv, not fixed text (ruling U)")
        want = hashlib.sha256(open(record, "rb").read()).hexdigest()
        check("`records/openpyxl-image-2026-01-01.md`, sha256 %s." % want in text, "the image record's sha256")
        check(all(ord(c) < 128 for c in text), "the record is ASCII")
        check(not any(i in text for i in IDS), "no workbook id is named")

        rc, so, se = run(common)
        check(rc == 2 and "does not overwrite" in se, "an existing record is refused")

        def refused(mutate, words, what):
            with tempfile.TemporaryDirectory() as b2:
                c2, s2, r2 = build(b2)
                mutate(c2, s2)
                os.makedirs(os.path.join(b2, "out"))
                rc, so, se = run(["--corpus-runs", c2, "--scale-runs", s2, "--stamp", STAMP, "--image-record", r2,
                                  "--out", os.path.join(b2, "out")])
                check(rc == 2 and words in se and so == "" and not os.listdir(os.path.join(b2, "out")), what)

        refused(lambda c, s: os.rename(os.path.join(c, "%s-openpyxl-copy" % STAMP), os.path.join(c, "moved")),
                "does not exist", "a missing openpyxl run directory refused, nothing written")
        refused(lambda c, s: os.remove(os.path.join(c, "%s-openpyxl-names" % STAMP, "empty.openpyxl.tsv")),
                "cannot read", "a missing empty-run file refused")
        refused(lambda c, s: write(os.path.join(c, "%s-table" % STAMP, "summary.i686.tsv"), ["id\tlabel"]),
                "expected header", "a summary with another header refused")
        refused(lambda c, s: write(os.path.join(c, "%s-openpyxl-image" % STAMP, "run.openpyxl.tsv"),
                                   [OX_RUN, "c\ts\te\timg\t3.1.5\t600\t600\t4096\to\tx\t3\tcopy"]),
                "names another op", "a run row naming another op refused")
        refused(lambda c, s: write(os.path.join(c, "%s-openpyxl-read" % STAMP, "summary.openpyxl.tsv"),
                                   [SUMMARY_HEADER, row(IDS[0], "openpyxl", "ok", "1.00", "1", ("1", "0", "0", "0", "0"), "ok", "-")]),
                "same workbooks", "runs that do not cover the same workbooks refused")
        refused(lambda c, s: write(os.path.join(s, "%s-openpyxl-scale-b-names" % STAMP, "run.openpyxl.tsv"),
                                   [OX_RUN, "c\ts\te\timg\t3.1.5\t600\t600\t16384\to\tx\t1\tcopy"]),
                "names another op", "a scale run row naming another op refused (ruling U)")
        with tempfile.TemporaryDirectory() as b3:
            c3, s3, r3 = build(b3)
            write(os.path.join(s3, "%s-openpyxl-scale-a-edit" % STAMP, "run.openpyxl.tsv"),
                  [OX_RUN, "c\ts\te\timg\t3.1.5\t3600\t600\t8192\to\tx\t1\tedit"])
            os.makedirs(os.path.join(b3, "out"))
            rc, so, se = run(["--corpus-runs", c3, "--scale-runs", s3, "--stamp", STAMP, "--image-record", r3,
                              "--out", os.path.join(b3, "out")])
            t3 = open(os.path.join(b3, "out", "compare-m5p6-%s.md" % STAMP), encoding="utf-8").read() if rc == 0 else ""
            check(rc == 0 and "openpyxl under --memory 16384, 8192 --timeout 3600, 600, emend" in t3,
                  "a scale run with other settings: the sentence names every value the runs hold (ruling U)")
        rc, so, se = run(["--corpus-runs", corpus, "--scale-runs", scale, "--stamp", "bad", "--image-record", record, "--out", out])
        check(rc == 2 and "--stamp" in se, "a bad stamp refused")
    if failed:
        print("compare-record-test: FAIL")
        return 1
    print("compare-record-test: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
