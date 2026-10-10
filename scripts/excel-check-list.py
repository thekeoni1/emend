#!/usr/bin/env python3
"""emend Excel check list (specs/0015-open-checks.md, "The Excel check", with
the planning rulings during P2, 5 to 10, and on the Excel check, 2 and 6;
specs/0016-new-workbook-and-sheets.md, "The carried items", with the planning
rulings during P1, 1 to 4 and 7).

Usage: scripts/excel-check-list.py --run <run dir> [--run-plain <run dir>]...
       --keep <dir> --record <.txt> --out <file>

Writes the list the maintainer carries to a machine with Excel: one
tab-separated row per file, under a header of thirteen columns (source,
id_or_test, label, file, bytes, sha256, result, ref1, expect1, ref2, expect2,
ref3, expect3). The list is written as .txt and is never committed (P2 ruling
7): its rows name cell positions in corpus workbooks.

Corpus rows come first, in manifest order: every workbook of the run
directory's summary.host.tsv, source "corpus", its id, label "host", and its
output, out.host.<ext> with the manifest's extension, named relative to the
corpus root (the run directory's grandparent, which must hold manifest.tsv).
The output's size must be the summary's out_bytes (0016 P1 ruling 1). Each
--run output is a --formula-sample output, so the row carries the three
references the sample wrote, derived here from the workbook's input by part
6's definition (specs/0013, "Harness runs"): the first sheet whose workbook
relationship is of kind worksheet, in <sheets> order; in it, in document
order, A (the first cell whose t is absent or "n", with a <v> or an <is> and
no <f>), F (the first cell with an <f> whose t is absent or "normal") and M
(the largest row number among its cells, 0 for none). ref1 is (M + 1, 1),
expected 42 (A times 2, A set to 21); ref2 is (M + 1, 2), expected 2 (1 + 1);
ref3 is F, expected 5 (2 + 3), empty when there is no F. With no A the sample
edits nothing and all six are empty.

Each --run-plain directory, repeatable, in the order given, adds the same
corpus rows from another run directory of the same corpus root (for example a
--style-sample run; specs/0015 Excel ruling 6): label the directory's
basename, and the six reference and value columns empty. A --run-plain
directory under another corpus root than --run's is refused (0016 P1 ruling
4).

A summary row whose outcome is error (the tool refused the input, exit 2, so
the run directory holds no output for it) is left out of the list, in --run and
--run-plain alike, and the count is printed to stderr once the list is
written, as "note: <n> error row(s) left out of <label>", for a run directory
with one or more such rows only; a run directory whose every row is an error
is refused (specs/0030-carries-and-api-changes.md, item 1, and "Planning
rulings during P1", ruling 6). Any other outcome but ok is refused as before.

Fixture rows follow, in name order: source "fixture", the test name, label
"-", and the kept output named relative to the keep directory, for every pair
whose input LibreOffice accepted (lo_in ok in --record, the edit open check's
record of that keep; P2 ruling 5). Each is followed by its input row (specs/0015
Excel ruling 2): source "input", the same test name, label "-", file
<test>.in.xlsx, and the record's in_bytes and in_sha256. The record must name
exactly the keep's pairs, by name, bytes and both sha256s, or the script stops
(P2 ruling 8). Fixture and input rows carry no references.

The result column is empty: scripts/excel-check-open.ps1 or the maintainer
fills it. The list is written with umask 077 (mode 600). The script prints
counts only, never a reference, a value or a path. Standard library only
(zipfile and expat). Errors, a short manifest or summary row among them, print
one "error: " line without a path and exit 2; nothing is written then.
"""

import argparse
import hashlib
import os
import re
import sys
import zipfile
from xml.parsers import expat

HEADER = ("source", "id_or_test", "label", "file", "bytes", "sha256", "result",
          "ref1", "expect1", "ref2", "expect2", "ref3", "expect3")
RECORD_HEADER = ("test", "in_bytes", "in_sha256", "out_bytes", "out_sha256",
                 "lo_in", "lo", "reason_in", "reason_out")
MANIFEST_ID = 0
PAIR = re.compile(r"^([A-Za-z0-9_.-]+)\.(in|out)\.xlsx$")
REF = re.compile(r"^([A-Z]+)([0-9]+)$")
OFFICE_DOCUMENT = "/officeDocument"
WORKSHEET = "/worksheet"
# The sample's constants: A is set to 21 and (M + 1, 1) to A times 2;
# (M + 1, 2) to 1 + 1; F to 2 + 3.
EXPECT_1, EXPECT_2, EXPECT_3 = "42", "2", "5"


class Refused(Exception):
    pass


def local(name):
    return name.rsplit(":", 1)[-1]


def sha256_of(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def parse(data, start, end=None, chars=None):
    parser = expat.ParserCreate()
    parser.StartElementHandler = start
    if end is not None:
        parser.EndElementHandler = end
    if chars is not None:
        parser.CharacterDataHandler = chars
    parser.Parse(data, True)


def rels_part(part):
    folder, name = part.rsplit("/", 1) if "/" in part else ("", part)
    return (folder + "/" if folder else "") + "_rels/" + name + ".rels"


def resolve(part, target):
    if target.startswith("/"):
        return target[1:]
    folder = part.rsplit("/", 1)[0].split("/") if "/" in part else []
    for piece in target.split("/"):
        if piece == "..":
            if folder:
                folder.pop()
        elif piece not in ("", "."):
            folder.append(piece)
    return "/".join(folder)


def relationships(archive, part):
    """{id: (type, resolved target)} of one part's relationships part."""
    try:
        data = archive.read(rels_part(part) if part else "_rels/.rels")
    except KeyError:
        return {}
    found = {}

    def start(tag, attrs):
        if local(tag) == "Relationship" and attrs.get("TargetMode") != "External":
            found[attrs.get("Id")] = (attrs.get("Type", ""), resolve(part, attrs.get("Target", "")))

    parse(data, start)
    return found


def col_number(letters):
    n = 0
    for ch in letters:
        n = n * 26 + ord(ch) - ord("A") + 1
    return n


def a1(row, col):
    letters = ""
    while col:
        col, rest = divmod(col - 1, 26)
        letters = chr(ord("A") + rest) + letters
    return "%s%d" % (letters, row)


def sample_refs(path):
    """(ref1, ref2, ref3) for one --formula-sample input, each "" when absent."""
    with zipfile.ZipFile(path) as archive:
        workbook = None
        for kind, target in relationships(archive, "").values():
            if kind.endswith(OFFICE_DOCUMENT):
                workbook = target
                break
        if workbook is None:
            raise Refused("a corpus input has no workbook part")
        rels = relationships(archive, workbook)
        sheet_ids = []

        def on_sheet(tag, attrs):
            if local(tag) == "sheet":
                rid = next((v for k, v in attrs.items() if local(k) == "id" and k != "id"), None)
                sheet_ids.append(rid)

        parse(archive.read(workbook), on_sheet)
        sheet = None
        for rid in sheet_ids:
            if rid in rels and rels[rid][0].endswith(WORKSHEET):
                sheet = rels[rid][1]
                break
        if sheet is None:
            return ("", "", "")
        data = archive.read(sheet)

    state = {"row": 0, "col": 0, "cell": None, "in_sheet_data": False}
    cells = []

    def start(tag, attrs):
        name = local(tag)
        if name == "sheetData":
            state["in_sheet_data"] = True
        elif not state["in_sheet_data"]:
            return
        elif name == "row":
            r = attrs.get("r")
            state["row"] = int(r) if r and r.isdigit() else state["row"] + 1
            state["col"] = 0
        elif name == "c":
            r = attrs.get("r")
            m = REF.match(r) if r else None
            if m:
                state["col"] = col_number(m.group(1))
                row = int(m.group(2))
            else:
                state["col"] += 1
                row = state["row"]
            state["cell"] = {"ref": a1(row, state["col"]), "row": row, "t": attrs.get("t"),
                             "f": False, "ft": None, "value": False}
        elif state["cell"] is not None:
            if name == "f":
                state["cell"]["f"] = True
                state["cell"]["ft"] = attrs.get("t")
            elif name in ("v", "is"):
                state["cell"]["value"] = True

    def end(tag):
        name = local(tag)
        if name == "c" and state["cell"] is not None:
            cells.append(state["cell"])
            state["cell"] = None
        elif name == "sheetData":
            state["in_sheet_data"] = False

    parse(data, start, end)
    a = next((c for c in cells if c["t"] in (None, "n") and c["value"] and not c["f"]), None)
    f = next((c for c in cells if c["f"] and c["ft"] in (None, "normal")), None)
    top = max((c["row"] for c in cells), default=0)
    if a is None:
        return ("", "", "")
    return (a1(top + 1, 1), a1(top + 1, 2), f["ref"] if f else "")


def read_tsv(path):
    with open(path, newline="") as handle:
        lines = handle.read().split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    return [line.split("\t") for line in lines]


def corpus_rows(run, label, with_refs):
    """(corpus root, rows, error rows left out) for one run directory."""
    run = os.path.abspath(run)
    root = os.path.dirname(os.path.dirname(run))
    manifest = os.path.join(root, "manifest.tsv")
    if not os.path.isfile(manifest):
        raise Refused("the run directory's grandparent holds no manifest.tsv")
    summary_path = os.path.join(run, "summary.host.tsv")
    if not os.path.isfile(summary_path):
        raise Refused("the run directory has no summary.host.tsv")
    summary = read_tsv(summary_path)
    head = summary[0]
    if "id" not in head or "outcome" not in head or "out_bytes" not in head:
        raise Refused("summary.host.tsv has another header")
    ids, sizes = {}, {}
    for row in summary[1:]:
        cells = dict(zip(head, row))
        if cells["id"] in ids:
            raise Refused("summary.host.tsv names a workbook twice")
        ids[cells["id"]] = cells["outcome"]
        sizes[cells["id"]] = cells["out_bytes"]
    if not ids:
        raise Refused("summary.host.tsv names no workbook")
    errors = sum(1 for outcome in ids.values() if outcome == "error")
    if errors == len(ids):
        raise Refused("a run directory holds no ok row")
    manifest_rows = read_tsv(manifest)
    inputs, exts = {}, {}
    for row in manifest_rows[1:]:
        inputs[row[MANIFEST_ID]] = os.path.join(root, "corpus", "%s.%s" % (row[0], row[3]))
        exts[row[MANIFEST_ID]] = row[3]
    rows = []
    for wid in [r[MANIFEST_ID] for r in manifest_rows[1:] if r[MANIFEST_ID] in ids]:
        if ids[wid] == "error":
            continue
        if ids[wid] != "ok":
            raise Refused("a workbook of the run has an outcome other than ok")
        out = os.path.join(run, wid, "out.host." + exts[wid])
        if not os.path.isfile(out):
            raise Refused("a workbook of the run has no host output")
        if str(os.path.getsize(out)) != sizes[wid]:
            raise Refused("a corpus output's size differs from the summary")
        if not os.path.isfile(inputs[wid]):
            raise Refused("a workbook of the run has no input in the corpus")
        refs = sample_refs(inputs[wid]) if with_refs else ("", "", "")
        expects = (EXPECT_1 if refs[0] else "", EXPECT_2 if refs[1] else "", EXPECT_3 if refs[2] else "")
        rows.append(("corpus", wid, label, os.path.relpath(out, root), str(os.path.getsize(out)),
                     sha256_of(out), "", refs[0], expects[0], refs[1], expects[1], refs[2], expects[2]))
    if len(rows) + errors != len(ids):
        raise Refused("summary.host.tsv names a workbook the manifest does not")
    return root, rows, errors


def fixture_rows(keep, record):
    seen = {}
    for name in sorted(os.listdir(keep)):
        path = os.path.join(keep, name)
        m = PAIR.match(name)
        if not os.path.isfile(path) or m is None:
            raise Refused("the keep directory holds a file that is not half of a pair")
        seen.setdefault(m.group(1), set()).add(m.group(2))
    if not seen:
        raise Refused("the keep directory holds no pairs")
    if any(sides != {"in", "out"} for sides in seen.values()):
        raise Refused("the keep directory holds half a pair")
    rows = read_tsv(record)
    if not rows or tuple(rows[0]) != RECORD_HEADER:
        raise Refused("the record has another header")
    named = {}
    for row in rows[1:]:
        if len(row) != len(RECORD_HEADER) or row[0] in named:
            raise Refused("the record has a malformed or repeated row")
        named[row[0]] = row
    if set(named) != set(seen):
        raise Refused("the record does not name exactly the keep's pairs")
    out = []
    for test in sorted(seen):
        row = named[test]
        fin = os.path.join(keep, test + ".in.xlsx")
        fout = os.path.join(keep, test + ".out.xlsx")
        if (row[1], row[2], row[3], row[4]) != (str(os.path.getsize(fin)), sha256_of(fin),
                                                 str(os.path.getsize(fout)), sha256_of(fout)):
            raise Refused("the record's bytes or hashes do not match a kept file")
        if row[5] == "ok":
            out.append(("fixture", test, "-", test + ".out.xlsx", row[3], row[4], "",
                        "", "", "", "", "", ""))
            out.append(("input", test, "-", test + ".in.xlsx", row[1], row[2], "",
                        "", "", "", "", "", ""))
    return out, len(seen)


def main():
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--run", required=True)
    parser.add_argument("--run-plain", action="append", default=[])
    parser.add_argument("--keep", required=True)
    parser.add_argument("--record", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    os.umask(0o077)
    try:
        if not os.path.isdir(args.run):
            raise Refused("--run is not a directory")
        if not all(os.path.isdir(plain) for plain in args.run_plain):
            raise Refused("a --run-plain is not a directory")
        if not os.path.isdir(args.keep):
            raise Refused("--keep is not a directory")
        if not os.path.isfile(args.record):
            raise Refused("--record is not a file")
        if os.path.exists(args.out):
            raise Refused("--out is already there")
        root, corpus, errors = corpus_rows(args.run, "host", True)
        notes = [("host", errors)]
        plain = 0
        for run in args.run_plain:
            label = os.path.basename(os.path.abspath(run))
            plain_root, rows, errors = corpus_rows(run, label, False)
            if plain_root != root:
                raise Refused("a --run-plain directory is under another corpus root")
            corpus += rows
            plain += len(rows)
            notes.append((label, errors))
        fixtures, pairs = fixture_rows(args.keep, args.record)
        with open(args.out, "x") as handle:
            handle.write("\t".join(HEADER) + "\n")
            for row in corpus + fixtures:
                handle.write("\t".join(row) + "\n")
    except Refused as err:
        print("error: %s" % err, file=sys.stderr)
        return 2
    except (OSError, zipfile.BadZipFile, expat.ExpatError, KeyError, ValueError, IndexError) as err:
        print("error: cannot read the run, the keep or the record (%s)" % type(err).__name__, file=sys.stderr)
        return 2
    for label, errors in notes:
        if errors:
            print("note: %d error row(s) left out of %s" % (errors, label), file=sys.stderr)
    accepted = sum(1 for r in fixtures if r[0] == "fixture")
    print("corpus rows %d (with A %d, with F %d; %d from --run-plain); fixture rows %d and input rows %d"
          " of %d kept pairs (%d left out: input not accepted)"
          % (len(corpus), sum(1 for r in corpus if r[7]), sum(1 for r in corpus if r[11]), plain,
             accepted, len(fixtures) - accepted, pairs, pairs - accepted))
    return 0


if __name__ == "__main__":
    sys.exit(main())
