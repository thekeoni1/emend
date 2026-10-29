#!/usr/bin/env python3
"""Propose the producer and the tags for one staged workbook (specs/0046, P1).

usage: corpus-tags.py <file>

Reads the file and never writes. Prints exactly one line:
  <ext> <bytes> <producer> <tags>[ chartex][ threaded]
<ext> is the file's extension when it is one of xlsx, xlsm, xltx, xltm and
"other" otherwise; <producer> is proposed from docProps/app.xml's Application;
<tags> are corpus-add.py's tags, comma-separated, proposed from the entry names
and the elements present. "chartex" and "threaded" are words beside the tags,
not tags. Elements are matched by name; no attribute value, text or name from
the file is ever printed.
A file that is not a zip prints one "refused: " line on stderr and exits 1;
any other error prints one "error: " line naming only the exception class and
exits 2.
"""
import os
import re
import sys
import xml.etree.ElementTree as ET
import zipfile

USAGE = "usage: corpus-tags.py <file>"
EXTS = {"xlsx", "xlsm", "xltx", "xltm"}
# corpus-add.py's tag list, in its order.
TAGS = ["plain", "strings", "formulas", "names", "merge", "validation",
        "hyperlinks", "autofilter", "freeze", "cf", "tables", "charts",
        "pivots", "slicers", "timelines", "macros", "comments", "images",
        "external", "sparklines", "protection", "date1904", "large"]
EP = "{http://schemas.openxmlformats.org/officeDocument/2006/extended-properties}"
# The transitional and strict main namespaces; a formula is an <f> in one of
# them, so an x14 extension's <xm:f> is not one.
MAIN = {"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        "http://purl.oclc.org/ooxml/spreadsheetml/main"}
SHEET = re.compile(r"^xl/worksheets/[^/]+\.xml$")
CHARTEX = re.compile(r"^xl/charts/chartEx[^/]*\.xml$")
# Worksheet elements, by local name in any namespace, and the tag each gives.
SHEET_ELEMENTS = {"mergeCell": "merge", "dataValidation": "validation",
                  "hyperlink": "hyperlinks", "autoFilter": "autofilter",
                  "conditionalFormatting": "cf", "sparklineGroup": "sparklines",
                  "sheetProtection": "protection"}
LARGE = 60000000


def die(msg):
    print(f"refused: {msg}", file=sys.stderr)
    sys.exit(1)


def split(tag):
    if tag.startswith("{"):
        ns, _, local = tag[1:].partition("}")
        return ns, local
    return "", tag


def elements(z, name):
    """Yields (namespace, local name, attributes) for each element of one entry."""
    with z.open(name) as f:
        for _, el in ET.iterparse(f, events=("start",)):
            ns, local = split(el.tag)
            yield ns, local, el.attrib


def producer(z, names):
    if "docProps/app.xml" not in names:
        return "unknown"
    try:
        root = ET.fromstring(z.read("docProps/app.xml"))
    except ET.ParseError:
        return "unknown"
    el = root.find(EP + "Application")
    app = " ".join(el.text.split()) if el is not None and el.text else ""
    # Excel for Mac writes "Microsoft Macintosh Excel"; Excel for Windows
    # writes "Microsoft Excel" with no platform word.
    if app == "Microsoft Macintosh Excel":
        return "excel-mac"
    if app == "Microsoft Excel":
        return "excel-win"
    return "unknown"


def propose(z):
    infos = z.infolist()
    names = {i.filename for i in infos}
    tags = {"plain"}
    words = []

    def under(prefix):
        return any(n.startswith(prefix) for n in names)

    if "xl/sharedStrings.xml" in names:
        tags.add("strings")
    for prefix, tag in (("xl/tables/", "tables"), ("xl/charts/", "charts"),
                        ("xl/pivotTables/", "pivots"), ("xl/slicers/", "slicers"),
                        ("xl/timelines/", "timelines"), ("xl/media/", "images"),
                        ("xl/externalLinks/", "external"), ("xl/comments", "comments"),
                        ("xl/threadedComments/", "comments")):
        if under(prefix):
            tags.add(tag)
    if "xl/vbaProject.bin" in names:
        tags.add("macros")
    if any(CHARTEX.match(n) for n in names):
        words.append("chartex")
    if under("xl/threadedComments/"):
        words.append("threaded")
    if sum(i.file_size for i in infos) > LARGE:
        tags.add("large")

    for name in sorted(n for n in names if SHEET.match(n)):
        for ns, local, attrib in elements(z, name):
            if local == "f" and ns in MAIN:
                tags.add("formulas")
            elif local in SHEET_ELEMENTS:
                tags.add(SHEET_ELEMENTS[local])
            elif local == "pane" and attrib.get("state") == "frozen":
                tags.add("freeze")
    if "xl/workbook.xml" in names:
        for ns, local, attrib in elements(z, "xl/workbook.xml"):
            if local == "definedName":
                tags.add("names")
            elif local == "workbookProtection":
                tags.add("protection")
            elif local == "workbookPr" and attrib.get("date1904") in ("1", "true"):
                tags.add("date1904")
    return producer(z, names), [t for t in TAGS if t in tags], words


def main(argv):
    if len(argv) != 2:
        print(USAGE, file=sys.stderr)
        sys.exit(2)
    path = argv[1]
    ext = os.path.splitext(path)[1].lstrip(".").lower()
    if ext not in EXTS:
        ext = "other"
    try:
        with open(path, "rb") as f:
            magic = f.read(4)
        size = os.path.getsize(path)
    except OSError:
        die("the file cannot be read")
    if magic != b"PK\x03\x04":
        die("not a zip-based workbook")
    try:
        with zipfile.ZipFile(path) as z:
            prod, tags, words = propose(z)
    except zipfile.BadZipFile:
        die("not a zip-based workbook")
    except ET.ParseError:
        die("an XML part is not well-formed")
    print(" ".join([ext, str(size), prod, ",".join(tags)] + words))


if __name__ == "__main__":
    try:
        main(sys.argv)
    except SystemExit:
        raise
    except Exception as e:
        print(f"error: {type(e).__name__}", file=sys.stderr)
        sys.exit(2)
