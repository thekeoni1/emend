#!/usr/bin/env python3
"""Add one workbook to the private emend corpus (specs/0005-corpus.md, P1).
Imported from the corpus session's private helper through a recorded step.

usage: corpus-add.py <file> <producer> <producer_version> <source> <tags> [note]

Prints exactly one line on success: added <id> <ext> <bytes> <producer> <tags>
Never prints the original name, the note, the corpus path or file content.
A refusal prints one "refused: " line on stderr and exits 1; any other error
prints one "error: " line naming only the exception class and exits 2.
"""
import datetime
import hashlib
import os
import re
import shutil
import stat
import subprocess
import sys
import xml.etree.ElementTree as ET
import zipfile

USAGE = "usage: corpus-add.py <file> <producer> <producer_version> <source> <tags> [note]"
EXTS = {"xlsx", "xlsm", "xltx", "xltm"}
PRODUCERS = {"excel-win", "excel-mac", "google-sheets", "libreoffice", "unknown"}
SOURCES = {"libreoffice-export", "ms-template", "google-sheets-export", "maintainer"}
TAGS = ["plain", "strings", "formulas", "names", "merge", "validation",
        "hyperlinks", "autofilter", "freeze", "cf", "tables", "charts",
        "pivots", "slicers", "timelines", "macros", "comments", "images",
        "external", "sparklines", "protection", "date1904", "large"]
HEADER = ["id", "sha256", "bytes", "ext", "producer", "producer_version", "app",
          "source", "tags", "added", "original_name", "note"]
EP = "{http://schemas.openxmlformats.org/officeDocument/2006/extended-properties}"


def die(msg):
    print(f"refused: {msg}", file=sys.stderr)
    sys.exit(1)


def read_app(path):
    try:
        with zipfile.ZipFile(path) as z:
            data = z.read("docProps/app.xml")
    except KeyError:
        return "none"
    except zipfile.BadZipFile:
        die("not a zip-based workbook")
    try:
        root = ET.fromstring(data)
    except ET.ParseError:
        die("docProps/app.xml is not well-formed")
    vals = []
    for tag in ("Application", "AppVersion"):
        el = root.find(EP + tag)
        if el is not None and el.text and el.text.strip():
            vals.append(" ".join(el.text.split()))
    return " ".join(vals) if vals else "none"


def outside_git(corpus):
    # The caller's git environment must not point git at another repository.
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    try:
        r = subprocess.run(["git", "-C", corpus, "rev-parse"], capture_output=True, env=env)
    except OSError:
        die("git could not be run")
    return r.returncode != 0


def main(argv):
    if len(argv) not in (6, 7):
        print(USAGE, file=sys.stderr)
        sys.exit(2)
    src, producer, pver, source, tags = argv[1:6]
    note = argv[6] if len(argv) == 7 else ""

    corpus = os.environ.get("EMEND_CORPUS_DIR")
    if not corpus:
        if not os.environ.get("HOME"):
            die("HOME is not set")
        corpus = os.path.join(os.environ["HOME"], "emend-testing")
    try:
        st = os.stat(corpus)
    except OSError:
        die("corpus directory missing")
    if not stat.S_ISDIR(st.st_mode):
        die("corpus directory is not a directory")
    if stat.S_IMODE(st.st_mode) != 0o700:
        die("corpus directory is not mode 700")
    if not outside_git(corpus):
        die("corpus directory is inside a git work tree")

    ext = os.path.splitext(src)[1].lstrip(".").lower()
    if ext not in EXTS:
        die("extension not one of xlsx, xlsm, xltx, xltm")
    try:
        with open(src, "rb") as f:
            magic = f.read(4)
    except OSError:
        die("the file cannot be read")
    if magic != b"PK\x03\x04":
        die("not a zip-based workbook")
    if producer not in PRODUCERS:
        die("unknown producer")
    if source not in SOURCES:
        die("unknown source")
    taglist = tags.split(",")
    if not tags or any(t not in TAGS for t in taglist) or len(set(taglist)) != len(taglist):
        die("unknown, empty or repeated tag")
    for field in (pver, note, os.path.basename(src)):
        if re.search(r"[\t\r\n]", field):
            die("tab or newline in a text field")
        # A file name or argument the file system gave us as undecodable bytes
        # carries surrogates, which the manifest write would refuse after the
        # copy was made, leaving a corpus file with no row.
        try:
            field.encode("utf-8")
        except UnicodeEncodeError:
            die("a text field is not valid UTF-8")

    h = hashlib.sha256()
    with open(src, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    sha = h.hexdigest()
    fid = sha[:12]
    size = os.path.getsize(src)

    manifest = os.path.join(corpus, "manifest.tsv")
    if os.path.exists(manifest):
        with open(manifest, encoding="utf-8") as f:
            text = f.read()
        lines = text.splitlines()
        if not lines or lines[0].split("\t") != HEADER:
            die("manifest header mismatch")
        if not text.endswith("\n"):
            die("manifest does not end with a newline")
        if any(l.split("\t")[0] == fid for l in lines[1:]):
            die("duplicate id")
    dest_dir = os.path.join(corpus, "corpus")
    dest = os.path.join(dest_dir, f"{fid}.{ext}")
    if os.path.exists(dest):
        die("duplicate id")

    app = read_app(src)
    # The workbook is copied to a temporary name inside corpus/, the row is
    # appended, and only then is the file renamed into place; the temporary is
    # removed on any failure (planning's ruling on P1, specs/0005-corpus.md).
    # Copying straight to the final name and appending afterwards leaves a
    # corpus file with no manifest row if the append fails, which is the
    # hazard the surrogate check above guards against from the other side.
    # The rename is within one directory, so it is atomic.
    tmp = os.path.join(dest_dir, f".{fid}.{ext}.corpus-add.tmp")
    old = os.umask(0o077)
    try:
        os.makedirs(dest_dir, mode=0o700, exist_ok=True)
        try:
            shutil.copyfile(src, tmp)
            os.chmod(tmp, 0o600)
            new = not os.path.exists(manifest)
            with open(manifest, "a", encoding="utf-8") as f:
                if new:
                    f.write("\t".join(HEADER) + "\n")
                added = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
                row = [fid, sha, str(size), ext, producer, pver, app, source, tags,
                       added, os.path.basename(src), note]
                f.write("\t".join(row) + "\n")
            os.chmod(manifest, 0o600)
            os.replace(tmp, dest)
        except BaseException:
            # Any failure, including a signal, leaves no temporary behind.
            #
            # If the append succeeded and only the rename failed, the row
            # stands with no file. That is the direction the ruling accepts,
            # because a row without a file is visible to the coverage check
            # while a file without a row is not. It is the INTENT of this
            # ordering, not a measured behaviour: T2 P7 could not reach the
            # branch from outside the process, since the duplicate guard above
            # refuses a destination that already exists, and any permission
            # that blocks os.replace also blocks the staging copy in the same
            # directory. negative.log records it as present and unexercised.
            try:
                os.unlink(tmp)
            except OSError:
                pass
            raise
    finally:
        os.umask(old)
    print(f"added {fid} {ext} {size} {producer} {tags}")


if __name__ == "__main__":
    try:
        main(sys.argv)
    except Exception as e:  # noqa: BLE001 - never print a path or a value
        print(f"error: {type(e).__name__}", file=sys.stderr)
        sys.exit(2)
