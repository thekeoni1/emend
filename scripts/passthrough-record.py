#!/usr/bin/env python3
"""emend T3 passthrough record (specs/0006-passthrough.md, "The record").

Reads summary.<label>.tsv, entries.<label>.tsv and run.<label>.tsv from a run
directory written by scripts/passthrough-run.sh, for every label present
(host, i686), and writes <stem>.txt and <stem>.md. The table is written as
.txt, not .tsv: scripts/check.sh's hygiene step refuses any tracked path with
a spreadsheet extension outside tests/fixtures/, and .gitignore ignores *.tsv,
so a .tsv record could not be committed without widening a privacy gate that
exists to keep spreadsheet exports out (planning's ruling of 2026-09-16). The
content is unchanged: the same two tab-separated tables with the same headers.
The run directory's own summary.<label>.tsv, entries.<label>.tsv and
run.<label>.tsv keep their names; they are never committed. The stem defaults to
step0-<run directory name>, the directory to records/ in the repository. A
run directory with only one label is refused unless --allow-single is given.

It never reads the corpus manifest, so no original name or note can reach the
record: a workbook is cited by id, producer and tags only, and entry names
appear as the harness prints them. Standard library only. Errors print one
"error: " line without a path and exit 2.
"""

import argparse
import datetime
import os
import re
import sys

LABELS = ("host", "i686")
OUTCOMES = ("ok", "error", "timeout", "panic", "crash")
FLAGS = ("mtime", "extra", "gpflags", "descriptor", "attrs", "comment", "local")
STATUSES = ("identical", "recompressed", "changed", "dropped", "added")
# The LibreOffice open check of specs/0006-passthrough.md P3: "-" when no image
# was given, or when there was no ok output to convert; "error" when the check
# could not run at all (planning's ruling of 2026-09-15), which is not a
# finding about the archive.
LO_VALUES = ("ok", "fail", "error", "-")
# The four counts reported for each column. "-" is written "not-run" in the
# record so the heading reads as a count and not as an empty cell.
LO_COUNTS = tuple("%s %s" % (column, "not-run" if value == "-" else value)
                  for column in ("lo", "lo_in") for value in LO_VALUES)

SUMMARY_HEADER = [
    "id", "label", "producer", "tags", "outcome", "exit", "signal", "wall_s",
    "peak_rss_kb", "in_bytes", "out_bytes", "diff_exit", "identical",
    "recompressed", "changed", "dropped", "added", "flagged",
    "order_preserved", "comment_equal", "files_identical", "leftover", "lo",
    "lo_in",
]
ENTRIES_HEADER = [
    "id", "label", "row", "name", "status", "index_in", "index_out",
    "method_in", "method_out", "uncompressed_in", "uncompressed_out",
    "compressed_in", "compressed_out", "crc_in", "crc_out",
    "first_difference", "differing_bytes", "last_difference", "flags",
]
RUN_HEADER = [
    "commit", "started", "ended", "rustc", "zip", "timeout_s", "lo_timeout_s",
    "memory_mib", "passthrough_sha256", "xlsx_diff_sha256", "files",
]

# The part families of specs/0000-step-zero-plan.md, in its order, and the
# fixed mapping from an entry name (as the harness prints it) to a family.
# The first matching rule wins; anything else is "other".
FAMILIES = (
    "content-types", "rels", "workbook", "worksheet", "sharedStrings",
    "styles", "theme", "calcChain", "drawing", "chart", "pivotTable",
    "pivotCache", "table", "queryTable", "connections", "externalLink",
    "comments", "threadedComments", "persons", "vml", "media", "embeddings",
    "vbaProject", "customXml", "docProps", "printerSettings", "slicer",
    "timeline", "richData", "metadata", "other",
)
RULES = (
    (r"^\[Content_Types\]\.xml$", "content-types"),
    (r"(^|/)_rels/[^/]*\.rels$", "rels"),
    (r"\.vml$", "vml"),
    (r"^xl/workbook\.xml$", "workbook"),
    (r"^xl/(worksheets|chartsheets|dialogsheets|macrosheets)/", "worksheet"),
    (r"^xl/sharedStrings\.xml$", "sharedStrings"),
    (r"^xl/styles\.xml$", "styles"),
    (r"^xl/theme/", "theme"),
    (r"^xl/calcChain\.xml$", "calcChain"),
    (r"^xl/drawings/", "drawing"),
    (r"^xl/charts/", "chart"),
    (r"^xl/pivotTables/", "pivotTable"),
    (r"^xl/pivotCache/", "pivotCache"),
    (r"^xl/tables/", "table"),
    (r"^xl/queryTables/", "queryTable"),
    (r"^xl/connections\.xml$", "connections"),
    (r"^xl/externalLinks/", "externalLink"),
    (r"^xl/comments[^/]*\.xml$", "comments"),
    (r"^xl/threadedComments/", "threadedComments"),
    (r"^xl/persons/", "persons"),
    (r"^xl/media/", "media"),
    (r"^xl/embeddings/", "embeddings"),
    (r"^xl/vbaProject[^/]*\.bin$", "vbaProject"),
    (r"^customXml/", "customXml"),
    (r"^docProps/", "docProps"),
    (r"^xl/printerSettings/", "printerSettings"),
    (r"^xl/(slicers|slicerCaches)/", "slicer"),
    (r"^xl/(timelines|timelineCaches)/", "timeline"),
    (r"^xl/richData/", "richData"),
    (r"^xl/metadata\.xml$", "metadata"),
)
COMPILED = tuple((re.compile(pattern), family) for pattern, family in RULES)

# What every summary value is expected to be for a clean passthrough.
EXPECTED = (
    ("recompressed", "0"), ("changed", "0"), ("dropped", "0"), ("added", "0"),
    ("flagged", "0"), ("order_preserved", "yes"), ("comment_equal", "yes"),
    ("files_identical", "yes"),
)


class Failure(Exception):
    pass


def family(name):
    for pattern, fam in COMPILED:
        if pattern.search(name):
            return fam
    return "other"


def read_table(path, header, what):
    """Rows of a tab-separated file as dicts, after checking the header and
    every row's width."""
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


def load(run_dir):
    labels = [label for label in LABELS if os.path.exists(os.path.join(run_dir, "summary.%s.tsv" % label))]
    if not labels:
        raise Failure("the run directory has no summary.<label>.tsv")
    data = {}
    for label in labels:
        def table(kind, header):
            name = "%s.%s.tsv" % (kind, label)
            return read_table(os.path.join(run_dir, name), header, name)

        summary = table("summary", SUMMARY_HEADER)
        entries = table("entries", ENTRIES_HEADER)
        runs = table("run", RUN_HEADER)
        seen = set()
        for row in summary:
            if row["label"] != label:
                raise Failure("summary.%s.tsv holds a row for another label" % label)
            if row["id"] in seen:
                raise Failure("summary.%s.tsv lists id %s twice" % (label, row["id"]))
            seen.add(row["id"])
            if row["outcome"] not in OUTCOMES:
                raise Failure("summary.%s.tsv has an unknown outcome" % label)
            if row["leftover"] not in ("yes", "no"):
                raise Failure("summary.%s.tsv has a leftover value other than yes or no" % label)
            for column in ("lo", "lo_in"):
                if row[column] not in LO_VALUES:
                    raise Failure("summary.%s.tsv has a %s value other than ok, fail, error or -" % (label, column))
        for row in entries:
            if row["label"] != label or row["id"] not in seen:
                raise Failure("entries.%s.tsv holds a row without a summary row" % label)
        data[label] = (summary, entries, runs)
    return labels, data


def lo_tally(label, summary):
    """The four counts (ok, fail, error, not-run) for each of lo and lo_in.

    Planning's ruling 4 of 2026-09-16 asks that the four account for every
    file. load() has already refused any value outside LO_VALUES, so a
    mismatch here cannot be caused by the input: it would mean this function
    miscounted. The check guards the script, not the run.
    """
    counts = {}
    for column in ("lo", "lo_in"):
        tally = dict.fromkeys(LO_VALUES, 0)
        for row in summary:
            tally[row[column]] += 1
        if sum(tally.values()) != len(summary):
            raise Failure("summary.%s.tsv: the %s counts total %d, not the %d files"
                          % (label, column, sum(tally.values()), len(summary)))
        for value in LO_VALUES:
            counts["%s %s" % (column, "not-run" if value == "-" else value)] = tally[value]
    return counts


def distinct(values):
    out = []
    for value in values:
        if value not in out:
            out.append(value)
    return out


def md_row(cells):
    return "| " + " | ".join(str(c) for c in cells) + " |"


def write_tsv(path, labels, data):
    lines = ["\t".join(SUMMARY_HEADER)]
    for label in labels:
        lines += ["\t".join(row[k] for k in SUMMARY_HEADER) for row in data[label][0]]
    lines.append("")
    lines.append("\t".join(ENTRIES_HEADER))
    for label in labels:
        lines += ["\t".join(row[k] for k in ENTRIES_HEADER) for row in data[label][1]]
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        handle.write("\n".join(lines) + "\n")


def write_md(path, run_name, stem, labels, data):
    """The record's prose table. `stem` names the companion .txt, which is not
    always "step0-" + run_name: --stem overrides it."""
    runs = [row for label in labels for row in data[label][2]]
    out = [
        "# step0 passthrough run %s" % run_name,
        "",
        "Written by `scripts/passthrough-record.py` from the run directory `%s`"
        " (raw outputs stay under the corpus directory's `runs/`). Workbooks"
        " are cited by id, producer and tags only (`specs/0005-corpus.md`)."
        " The machine-readable companion to this file is `%s.txt`, the same two"
        " tab-separated tables; it is written as `.txt` because the hygiene gate"
        " refuses a tracked spreadsheet extension." % (run_name, stem),
        "",
        "- Record written: %s" % datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "- Commit: %s" % ", ".join(distinct(r["commit"] for r in runs)),
        "- Runs: %d, from %s to %s" % (
            len(runs), min((r["started"] for r in runs), default="-"), max((r["ended"] for r in runs), default="-")),
        "- Targets: %s" % ", ".join(labels),
        "- Tool versions: %s; zip %s" % (
            "; ".join(distinct(r["rustc"] for r in runs)), ", ".join(distinct(r["zip"] for r in runs))),
        "- Settings: timeout %s s; open-check timeout %s s; memory %s MiB" % (
            ", ".join(distinct(r["timeout_s"] for r in runs)),
            ", ".join(distinct(r["lo_timeout_s"] for r in runs)),
            ", ".join(distinct(r["memory_mib"] for r in runs))),
        "",
        "## Files by outcome",
        "",
        md_row(("label", "files") + OUTCOMES + ("leftover",) + LO_COUNTS),
        md_row(("---",) * (3 + len(OUTCOMES) + len(LO_COUNTS))),
    ]
    for label in labels:
        summary = data[label][0]
        counts = lo_tally(label, summary)
        out.append(md_row([label, len(summary)] + [sum(1 for r in summary if r["outcome"] == o) for o in OUTCOMES]
                          + [sum(1 for r in summary if r["leftover"] == "yes")]
                          + [counts[k] for k in LO_COUNTS]))

    columns = ("family", "entries") + STATUSES + ("flagged",) + FLAGS
    for label in labels:
        counts = {fam: dict.fromkeys(columns[1:], 0) for fam in FAMILIES}
        for row in data[label][1]:
            c = counts[family(row["name"])]
            c["entries"] += 1
            if row["status"] in STATUSES:
                c[row["status"]] += 1
            flags = [f for f in row["flags"].split(",") if f]
            if flags:
                c["flagged"] += 1
            for flag in flags:
                if flag in FLAGS:
                    c[flag] += 1
        total = dict.fromkeys(columns[1:], 0)
        out += ["", "## Entries by part family: %s" % label, "", md_row(columns), md_row(("---",) * len(columns))]
        for fam in FAMILIES:
            if counts[fam]["entries"]:
                out.append(md_row([fam] + [counts[fam][k] for k in columns[1:]]))
                for k in columns[1:]:
                    total[k] += counts[fam][k]
        out.append(md_row(["total"] + [total[k] for k in columns[1:]]))

    out += [
        "",
        "## Files not ok, with a diff exit other than 0, with a leftover temporary"
        " file, or failing the LibreOffice open check",
        "",
        md_row(("id", "label", "producer", "tags", "outcome", "exit", "signal", "diff_exit", "leftover",
                "lo", "lo_in", "differs from the expectation")),
        md_row(("---",) * 12),
    ]
    listed = 0
    for label in labels:
        for row in data[label][0]:
            # Only "fail" belongs under a heading that says failing: planning's
            # ruling 4 of 2026-09-16. A check that could not run is not a
            # finding about the archive, so an "error" row that is otherwise
            # clean is listed under the next heading and not here. A row listed
            # here for another reason still shows its real lo value.
            if (row["outcome"] == "ok" and row["diff_exit"] == "0" and row["leftover"] == "no"
                    and row["lo"] != "fail" and row["lo_in"] != "fail"):
                continue
            listed += 1
            if row["outcome"] != "ok":
                differs = "not compared"
            elif row["diff_exit"] not in ("0", "1"):
                differs = "comparison failed"
            else:
                differs = ", ".join("%s=%s" % (k, row[k]) for k, want in EXPECTED if row[k] != want) or "-"
            out.append(md_row([row["id"], label, row["producer"], row["tags"], row["outcome"],
                               row["exit"] or "-", row["signal"] or "-", row["diff_exit"] or "-",
                               row["leftover"], row["lo"], row["lo_in"], differs]))
    if not listed:
        out.append(md_row(("none",) + ("",) * 11))

    # An open check that could not run says nothing about the archive, so its
    # files are listed apart from the ones LibreOffice refused.
    could_not_run = [(label, row) for label in labels for row in data[label][0]
                     if "error" in (row["lo"], row["lo_in"])]
    out += [
        "",
        "## Files whose open check could not run",
        "",
        md_row(("id", "label", "lo", "lo_in")),
        md_row(("---",) * 4),
    ]
    for label, row in could_not_run:
        out.append(md_row([row["id"], label, row["lo"], row["lo_in"]]))
    if not could_not_run:
        out.append(md_row(("none",) + ("",) * 3))
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        handle.write("\n".join(out) + "\n")


def main(argv):
    parser = argparse.ArgumentParser(prog="scripts/passthrough-record.py")
    parser.add_argument("--run", required=True, help="the run directory")
    parser.add_argument("--out", help="the directory to write into (default: records/ in the repository)")
    parser.add_argument("--stem", help="the file name stem (default: step0-<run directory name>)")
    parser.add_argument("--allow-single", action="store_true",
                        help="write a record from a run directory with only one label")
    args = parser.parse_args(argv)
    run_dir = os.path.abspath(args.run)
    run_name = os.path.basename(run_dir)
    if args.stem:
        stem = args.stem
    elif re.fullmatch(r"\d{4}-\d{2}-\d{2}-[0-9a-f]{7,40}", run_name):
        stem = "step0-" + run_name
    else:
        raise Failure("the run directory name is not <utc-date>-<shortsha>; give --stem")
    if not re.fullmatch(r"[A-Za-z0-9._-]+", stem):
        raise Failure("the stem may hold only letters, digits, dot, dash and underscore")
    out_dir = args.out or os.path.join(os.path.dirname(os.path.abspath(__file__)), os.pardir, "records")
    if not os.path.isdir(run_dir):
        raise Failure("the run directory does not exist")
    if not os.path.isdir(out_dir):
        raise Failure("the output directory does not exist")
    labels, data = load(run_dir)
    if len(labels) == 1 and not args.allow_single:
        raise Failure("the run directory has only the %s label; give --allow-single for a single-label record" % labels[0])
    write_tsv(os.path.join(out_dir, stem + ".txt"), labels, data)
    write_md(os.path.join(out_dir, stem + ".md"), run_name, stem, labels, data)
    print("wrote %s.txt and %s.md: labels %s; files %s; entry rows %s" % (
        stem, stem, ",".join(labels),
        ",".join(str(len(data[l][0])) for l in labels),
        ",".join(str(len(data[l][1])) for l in labels)))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except Failure as err:
        print("error: %s" % err, file=sys.stderr)
        sys.exit(2)
