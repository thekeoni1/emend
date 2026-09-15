#!/usr/bin/env python3
"""emend T2 corpus coverage (specs/0005-corpus.md, P1).

usage: corpus-coverage.py [--verify]

Reads manifest.tsv in the corpus directory (EMEND_CORPUS_DIR, default
$HOME/emend-testing) and validates it: the header, the column count of every
row, the id as the first 12 hex digits of the sha256, the byte count as a
number, the extension, producer, source and tags against the fixed lists, the
uniqueness of ids, and that every corpus/<id>.<ext> exists. With --verify it
also recomputes every file's sha256 and size. Then it prints the file count,
one line per workbook, one line per tag, the uncovered tags and the tags
covered by a single producer.

Only ids, extensions, byte counts, producers, sources, tags and counts are
printed: never the original_name or note columns, the producer_version or app
columns, or the corpus path. Any failure prints one "FAIL <reason>" line whose
reason names a manifest line number, never a value, and exits 1. Standard
library only.
"""
import hashlib
import os
import re
import sys

USAGE = "usage: corpus-coverage.py [--verify]"
EXTS = ("xlsx", "xlsm", "xltx", "xltm")
PRODUCERS = ("excel-win", "excel-mac", "google-sheets", "libreoffice", "unknown")
SOURCES = ("libreoffice-export", "ms-template", "google-sheets-export", "maintainer")
TAGS = ("plain", "strings", "formulas", "names", "merge", "validation",
        "hyperlinks", "autofilter", "freeze", "cf", "tables", "charts",
        "pivots", "slicers", "timelines", "macros", "comments", "images",
        "external", "sparklines", "protection", "date1904", "large")
HEADER = ["id", "sha256", "bytes", "ext", "producer", "producer_version", "app",
          "source", "tags", "added", "original_name", "note"]


class Fail(Exception):
    pass


def corpus_dir():
    corpus = os.environ.get("EMEND_CORPUS_DIR")
    if corpus:
        return corpus
    if not os.environ.get("HOME"):
        raise Fail("HOME is not set")
    return os.path.join(os.environ["HOME"], "emend-testing")


def load(corpus):
    try:
        with open(os.path.join(corpus, "manifest.tsv"), "rb") as f:
            data = f.read()
    except FileNotFoundError:
        raise Fail("manifest missing")
    except OSError:
        raise Fail("manifest cannot be read")
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        raise Fail("manifest is not UTF-8")
    if not text.endswith("\n"):
        raise Fail("manifest does not end with a newline")
    lines = text[:-1].split("\n")
    if lines[0].split("\t") != HEADER:
        raise Fail("manifest header mismatch")
    rows = []
    seen = set()
    for n, line in enumerate(lines[1:], start=2):
        cols = line.split("\t")
        if len(cols) != len(HEADER):
            raise Fail(f"line {n}: {len(cols)} columns, expected {len(HEADER)}")
        r = dict(zip(HEADER, cols))
        r["line"] = n
        if not re.fullmatch(r"[0-9a-f]{64}", r["sha256"]):
            raise Fail(f"line {n}: sha256 is not 64 lowercase hex digits")
        if r["id"] != r["sha256"][:12]:
            raise Fail(f"line {n}: id is not the first 12 hex digits of sha256")
        if not re.fullmatch(r"[0-9]+", r["bytes"]):
            raise Fail(f"line {n}: bytes is not a number")
        if r["ext"] not in EXTS:
            raise Fail(f"line {n}: unknown ext")
        if r["producer"] not in PRODUCERS:
            raise Fail(f"line {n}: unknown producer")
        if r["source"] not in SOURCES:
            raise Fail(f"line {n}: unknown source")
        taglist = r["tags"].split(",")
        if any(t not in TAGS for t in taglist) or len(set(taglist)) != len(taglist):
            raise Fail(f"line {n}: unknown, empty or repeated tag")
        r["taglist"] = taglist
        if r["id"] in seen:
            raise Fail(f"line {n}: duplicate id")
        seen.add(r["id"])
        r["path"] = os.path.join(corpus, "corpus", f"{r['id']}.{r['ext']}")
        if not os.path.isfile(r["path"]):
            raise Fail(f"line {n}: corpus file missing")
        rows.append(r)
    return rows


def verify(rows):
    for r in rows:
        n = r["line"]
        try:
            if os.path.getsize(r["path"]) != int(r["bytes"]):
                raise Fail(f"line {n}: bytes mismatch")
            h = hashlib.sha256()
            with open(r["path"], "rb") as f:
                for chunk in iter(lambda: f.read(1 << 20), b""):
                    h.update(chunk)
        except OSError:
            raise Fail(f"line {n}: corpus file cannot be read")
        if h.hexdigest() != r["sha256"]:
            raise Fail(f"line {n}: sha256 mismatch")


def report(rows):
    out = [f"files {len(rows)}"]
    for r in rows:
        out.append(f"{r['id']} {r['ext']} {r['bytes']} {r['producer']} {r['source']} {r['tags']}")
    uncovered = []
    single = []
    for tag in TAGS:
        have = [r for r in rows if tag in r["taglist"]]
        producers = [p for p in PRODUCERS if any(r["producer"] == p for r in have)]
        out.append(f"tag {tag} {len(have)} {','.join(producers) or '-'}")
        if not have:
            uncovered.append(tag)
        elif len(producers) == 1:
            single.append(tag)
    out.append(f"uncovered {len(uncovered)}: {','.join(uncovered)}" if uncovered else "uncovered 0")
    out.append(f"single-producer {len(single)}: {','.join(single)}" if single else "single-producer 0")
    return out


def main(argv):
    if argv[1:] not in ([], ["--verify"]):
        print(USAGE, file=sys.stderr)
        return 2
    try:
        rows = load(corpus_dir())
        if argv[1:] == ["--verify"]:
            verify(rows)
    except Fail as e:
        print(f"FAIL {e}")
        return 1
    print("\n".join(report(rows)))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Exception as e:  # noqa: BLE001 - never print a path or a value
        print(f"FAIL unexpected error {type(e).__name__}")
        sys.exit(1)
