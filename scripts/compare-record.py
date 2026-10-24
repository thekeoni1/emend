#!/usr/bin/env python3
"""The openpyxl comparison record (specs/0037-measurement-and-close.md, "The
comparison runner", "What is measured and how it is read", and the planning
rulings C, K, L, N, O and P; specs/0043-measurement-and-close.md, "The
comparison runner and the record", the sixteen ops).

usage: compare-record.py --corpus-runs <dir> --scale-runs <dir> --stamp <utc-date>-<shortsha>
                         --image-record <path> [--out <dir>] [--stem <stem>]

Reads the run directories' summary, run and empty-run files, never a log,
and writes <stem>.md (default compare-m6p6-<stamp>.md) into --out (default
records/ in the repository):

- per op, one row per tool and label (emend host, emend i686, openpyxl) from
  <corpus-runs>/<stamp>-<op> (scripts/passthrough-run.sh, both labels) and
  <corpus-runs>/<stamp>-openpyxl-<op> (scripts/compare-run.sh): files run,
  outcomes, the entries identical, recompressed, changed, dropped and added
  (totals over the files), the LibreOffice verdicts, wall time (median and
  sum, seconds) and peak RSS (median and maximum, kB). The openpyxl row's
  lo_in is "-" in its own summary (ruling C), so its input verdicts are the
  emend host run's, and the record names that directory;
- per op, the openpyxl directory's settings, its container start (the
  median of the three empty runs in empty.openpyxl.tsv, which its wall times
  include; rulings K and P) and the exception class of every error row, read
  from that row's stdout.openpyxl.txt, whose first line the ops script makes
  the class name alone;
- the scale table: tiers a and b for every op, tiers c and d for roundtrip
  (decision 2 A), from <scale-runs>/<stamp>-scale-<tier>-<op> (both labels)
  and <scale-runs>/<stamp>-openpyxl-scale-<tier>-<op> (ruling L), one row per
  tool and label with the outcome, exit, wall time, peak RSS, bytes in and
  out, the entry counts, the open verdict and, for openpyxl, the container
  start; the openpyxl settings its sentence names are read from the scale
  runs' run.openpyxl.tsv, whose op is checked as the corpus runs' is (ruling
  U of the planning verification of P2);
- the openpyxl image record's sha256.

Counts, times and bytes only: a workbook is never named, and no entry name is
listed. Medians are taken over the rows that hold a value. Standard library
only. Errors print one "error: " line without a path and exit 2.
"""

import argparse
import hashlib
import os
import re
import statistics
import sys

OPS = ("roundtrip", "read", "edit", "format", "insert-rows", "names", "table",
       "comment", "image", "add-chart", "copy", "chartsheet", "chart-format",
       "chart-analytic", "chart-text", "named-style")
SCALE = (("a", OPS), ("b", OPS), ("c", ("roundtrip",)), ("d", ("roundtrip",)))
SUMMARY_HEADER = [
    "id", "label", "producer", "tags", "outcome", "exit", "signal", "wall_s",
    "peak_rss_kb", "in_bytes", "out_bytes", "diff_exit", "identical",
    "recompressed", "changed", "dropped", "added", "flagged",
    "order_preserved", "comment_equal", "files_identical", "leftover", "lo",
    "lo_in",
]
RUN_HEADERS = {
    "emend": ["commit", "started", "ended", "rustc", "zip", "timeout_s", "lo_timeout_s",
              "memory_mib", "passthrough_sha256", "xlsx_diff_sha256", "files", "tool_args"],
    "openpyxl": ["commit", "started", "ended", "image_id", "openpyxl", "timeout_s",
                 "lo_timeout_s", "memory_mib", "ops_sha256", "xlsx_diff_sha256", "files", "op"],
}
EMPTY_HEADER = ["run", "wall_s"]
OUTCOMES = ("ok", "error", "timeout", "cap", "no-output", "panic", "crash")
ENTRY_COLUMNS = ("identical", "recompressed", "changed", "dropped", "added")
LO_VALUES = ("ok", "fail", "error", "-")
CLASS_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_.]*")


class Failure(Exception):
    pass


def read_table(path, header, what):
    try:
        with open(path, encoding="utf-8", newline="\n") as handle:
            lines = handle.read().split("\n")
    except OSError as err:
        raise Failure("cannot read %s: %s" % (what, err.strerror))
    except UnicodeDecodeError:
        raise Failure("%s is not UTF-8" % what)
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or lines[0].split("\t") != header:
        raise Failure("%s does not start with the expected header" % what)
    rows = []
    for number, line in enumerate(lines[1:], start=2):
        cells = line.split("\t")
        if len(cells) != len(header):
            raise Failure("%s line %d has %d columns, not %d" % (what, number, len(cells), len(header)))
        rows.append(dict(zip(header, cells)))
    return rows


def summary(run_dir, label, what):
    if not os.path.isdir(run_dir):
        raise Failure("the run directory %s does not exist" % what)
    rows = read_table(os.path.join(run_dir, "summary.%s.tsv" % label), SUMMARY_HEADER,
                      "%s summary.%s.tsv" % (what, label))
    if not rows:
        raise Failure("%s summary.%s.tsv holds no row" % (what, label))
    ids = [r["id"] for r in rows]
    if len(set(ids)) != len(ids):
        raise Failure("%s summary.%s.tsv lists an id twice" % (what, label))
    for r in rows:
        if r["label"] != label or r["outcome"] not in OUTCOMES:
            raise Failure("%s summary.%s.tsv has a row of another label or an unknown outcome" % (what, label))
        for column in ("lo", "lo_in"):
            if r[column] not in LO_VALUES:
                raise Failure("%s summary.%s.tsv has an unknown %s value" % (what, label, column))
    return rows


def run_rows(run_dir, label, what):
    tool = "openpyxl" if label == "openpyxl" else "emend"
    rows = read_table(os.path.join(run_dir, "run.%s.tsv" % label), RUN_HEADERS[tool],
                      "%s run.%s.tsv" % (what, label))
    if not rows:
        raise Failure("%s run.%s.tsv holds no row" % (what, label))
    return rows


def empty_median(run_dir, what):
    rows = read_table(os.path.join(run_dir, "empty.openpyxl.tsv"), EMPTY_HEADER, "%s empty.openpyxl.tsv" % what)
    if [r["run"] for r in rows] != ["1", "2", "3"] or not all(
            re.fullmatch(r"[0-9]+[.][0-9]{2}", r["wall_s"]) for r in rows):
        raise Failure("%s empty.openpyxl.tsv does not hold runs 1 to 3, each a time in seconds" % what)
    return "%.2f" % sorted(float(r["wall_s"]) for r in rows)[1]


def number(text):
    return re.fullmatch(r"[0-9]+([.][0-9]+)?", text) is not None


def fmt(value):
    if value is None:
        return "-"
    if float(value).is_integer():
        return "%d" % value
    return "%.2f" % value


def stats(rows, lo_in_rows=None):
    """One row's figures from a summary's rows. lo_in_rows, when given, are
    the rows whose lo_in is counted (the emend host run's, ruling C)."""
    out = {"files": len(rows)}
    for o in OUTCOMES:
        out[o] = sum(1 for r in rows if r["outcome"] == o)
    for c in ENTRY_COLUMNS:
        out[c] = sum(int(r[c]) for r in rows if r[c].isdigit())
    for v in LO_VALUES:
        out["lo " + v] = sum(1 for r in rows if r["lo"] == v)
        out["lo_in " + v] = sum(1 for r in (lo_in_rows if lo_in_rows is not None else rows) if r["lo_in"] == v)
    walls = [float(r["wall_s"]) for r in rows if number(r["wall_s"])]
    rss = [int(r["peak_rss_kb"]) for r in rows if r["peak_rss_kb"].isdigit()]
    out["wall median"] = statistics.median(walls) if walls else None
    out["wall sum"] = sum(walls) if walls else None
    out["rss median"] = statistics.median(rss) if rss else None
    out["rss max"] = max(rss) if rss else None
    return out


def error_classes(run_dir, rows, what):
    counts = {}
    for r in rows:
        if r["outcome"] != "error":
            continue
        name = "unread"
        try:
            with open(os.path.join(run_dir, r["id"], "stdout.openpyxl.txt"), encoding="utf-8") as handle:
                first = handle.readline().rstrip("\n")
            if CLASS_RE.fullmatch(first) and not first.startswith("openpyxl="):
                name = first
        except (OSError, UnicodeDecodeError):
            pass
        counts[name] = counts.get(name, 0) + 1
    return counts


def md_row(cells):
    return "| " + " | ".join(str(c) for c in cells) + " |"


CORPUS_COLUMNS = (("files",) + OUTCOMES + ENTRY_COLUMNS
                  + tuple("lo " + v for v in LO_VALUES) + tuple("lo_in " + v for v in LO_VALUES)
                  + ("wall median", "wall sum", "rss median", "rss max"))


def corpus_cells(s):
    return [fmt(s[c]) if c.startswith(("wall", "rss")) else s[c] for c in CORPUS_COLUMNS]


def heading(c):
    return {"wall median": "wall median s", "wall sum": "wall sum s", "rss median": "peak RSS median kB",
            "rss max": "peak RSS max kB"}.get(c, c.replace("lo -", "lo not-run").replace("lo_in -", "lo_in not-run"))


def build(args):
    out = []
    out += [
        "# openpyxl comparison, Milestone 6 part 6 P2 (%s)" % args.stamp,
        "",
        "Written by `scripts/compare-record.py` from the run directories' summary, run and"
        " empty-run files (`specs/0037-measurement-and-close.md`, \"The comparison runner\";"
        " rulings C, K, L, N, O and P; `specs/0043-measurement-and-close.md`, \"The comparison"
        " runner and the record\"). Counts, times and bytes only. Each tool's output is"
        " compared with its input by `xlsx-diff`; the two tools' outputs are never compared"
        " with each other. emend's wall time is the process's own, as"
        " `scripts/passthrough-run.sh` measures it; openpyxl's wraps `podman run` and so"
        " includes the container's start, whose time from an empty run of the image is"
        " named beside each op (rulings K and P). Medians are over the rows that hold a value.",
        "",
        "## The corpus, per op",
        "",
        md_row(("op", "tool", "label") + tuple(heading(c) for c in CORPUS_COLUMNS)),
        md_row(("---",) * (3 + len(CORPUS_COLUMNS))),
    ]
    details = []
    for op in OPS:
        e_dir = os.path.join(args.corpus_runs, "%s-%s" % (args.stamp, op))
        o_dir = os.path.join(args.corpus_runs, "%s-openpyxl-%s" % (args.stamp, op))
        e_name = os.path.basename(e_dir)
        o_name = os.path.basename(o_dir)
        host = summary(e_dir, "host", e_name)
        i686 = summary(e_dir, "i686", e_name)
        ox = summary(o_dir, "openpyxl", o_name)
        for label in ("host", "i686"):
            run_rows(e_dir, label, e_name)
        ox_runs = run_rows(o_dir, "openpyxl", o_name)
        if {r["op"] for r in ox_runs} != {op}:
            raise Failure("%s run.openpyxl.tsv names another op" % o_name)
        if sorted(r["id"] for r in ox) != sorted(r["id"] for r in host) or \
                sorted(r["id"] for r in i686) != sorted(r["id"] for r in host):
            raise Failure("the %s runs do not cover the same workbooks" % op)
        out.append(md_row([op, "emend", "host"] + corpus_cells(stats(host))))
        out.append(md_row([op, "emend", "i686"] + corpus_cells(stats(i686))))
        out.append(md_row([op, "openpyxl", "openpyxl"] + corpus_cells(stats(ox, lo_in_rows=host))))
        classes = error_classes(o_dir, ox, o_name)
        details.append(md_row([
            op, o_name, "timeout %s s; memory %s MiB" % (
                ", ".join(sorted({r["timeout_s"] for r in ox_runs})),
                ", ".join(sorted({r["memory_mib"] for r in ox_runs}))),
            empty_median(o_dir, o_name), e_name,
            ", ".join("%s %d" % (k, classes[k]) for k in sorted(classes)) or "none"]))
    out += [
        "",
        "## The openpyxl runs: settings, container start and error classes",
        "",
        "The openpyxl row's lo_in counts above are the emend host run's (ruling C), read"
        " from the directory named in the lo_in column here.",
        "",
        md_row(("op", "openpyxl run directory", "settings", "container start median s",
                "lo_in from", "error rows by exception class")),
        md_row(("---",) * 6),
    ] + details

    scale_cols = ("outcome", "exit", "wall_s", "peak_rss_kb", "in_bytes", "out_bytes") + ENTRY_COLUMNS + ("lo",)
    # Ruling U (specs/0037, "Planning verification of P2"): the settings
    # sentence is read from the scale runs' run.openpyxl.tsv, and their op
    # is checked as the corpus runs' is.
    scale_rows, memory, timeout = [], set(), set()
    for tier, ops in SCALE:
        for op in ops:
            e_dir = os.path.join(args.scale_runs, "%s-scale-%s-%s" % (args.stamp, tier, op))
            o_dir = os.path.join(args.scale_runs, "%s-openpyxl-scale-%s-%s" % (args.stamp, tier, op))
            for tool, label, run_dir in (("emend", "host", e_dir), ("emend", "i686", e_dir),
                                         ("openpyxl", "openpyxl", o_dir)):
                name = os.path.basename(run_dir)
                rows = summary(run_dir, label, name)
                runs = run_rows(run_dir, label, name)
                if tool == "openpyxl":
                    if {r["op"] for r in runs} != {op}:
                        raise Failure("%s run.openpyxl.tsv names another op" % name)
                    memory.update(r["memory_mib"] for r in runs)
                    timeout.update(r["timeout_s"] for r in runs)
                if len(rows) != 1:
                    raise Failure("%s summary.%s.tsv does not hold one row" % (name, label))
                r = rows[0]
                start = empty_median(run_dir, name) if tool == "openpyxl" else "-"
                scale_rows.append(md_row([tier, op, tool, label] + [r[c] or "-" for c in scale_cols] + [start]))
    out += [
        "",
        "## The scale tiers",
        "",
        "Tiers a and b for every op, tiers c and d for roundtrip (decision 2 A); one run"
        " directory per tier per op per tool (ruling L); openpyxl under --memory %s"
        " --timeout %s, emend as specs/0023 ran it (ruling M)." % (
            ", ".join(sorted(memory)), ", ".join(sorted(timeout))),
        "",
        md_row(("tier", "op", "tool", "label") + scale_cols + ("container start median s",)),
        md_row(("---",) * (5 + len(scale_cols))),
    ] + scale_rows

    try:
        with open(args.image_record, "rb") as handle:
            image_sha = hashlib.sha256(handle.read()).hexdigest()
    except OSError as err:
        raise Failure("cannot read the image record: %s" % err.strerror)
    out += [
        "",
        "## The openpyxl image",
        "",
        "`records/%s`, sha256 %s." % (os.path.basename(args.image_record), image_sha),
    ]
    return "\n".join(out) + "\n"


def main(argv):
    parser = argparse.ArgumentParser(prog="scripts/compare-record.py")
    parser.add_argument("--corpus-runs", required=True)
    parser.add_argument("--scale-runs", required=True)
    parser.add_argument("--stamp", required=True)
    parser.add_argument("--image-record", required=True)
    parser.add_argument("--out")
    parser.add_argument("--stem")
    args = parser.parse_args(argv)
    if not re.fullmatch(r"\d{4}-\d{2}-\d{2}-[0-9a-f]{7,40}", args.stamp):
        raise Failure("--stamp is not <utc-date>-<shortsha>")
    if not re.fullmatch(r"openpyxl-image-\d{4}-\d{2}-\d{2}[.]md", os.path.basename(args.image_record)):
        raise Failure("--image-record is not an openpyxl image record")
    stem = args.stem or "compare-m6p6-" + args.stamp
    if not re.fullmatch(r"[A-Za-z0-9._-]+", stem):
        raise Failure("the stem may hold only letters, digits, dot, dash and underscore")
    out_dir = args.out or os.path.join(os.path.dirname(os.path.abspath(__file__)), os.pardir, "records")
    if not os.path.isdir(out_dir):
        raise Failure("the output directory does not exist")
    text = build(args)
    target = os.path.join(out_dir, stem + ".md")
    if os.path.exists(target):
        raise Failure("%s.md exists; this script does not overwrite" % stem)
    with open(target, "w", encoding="utf-8", newline="\n") as handle:
        handle.write(text)
    print("wrote %s.md: ops %d; scale rows %d" % (
        stem, len(OPS), 3 * sum(len(ops) for _, ops in SCALE)))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except Failure as err:
        print("error: %s" % err, file=sys.stderr)
        sys.exit(2)
