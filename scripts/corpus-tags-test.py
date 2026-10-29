#!/usr/bin/env python3
"""The test of scripts/corpus-tags.py (specs/0046, P1).

usage: corpus-tags-test.py

Builds fictional workbooks in a temporary directory: a base of a content
types part, a workbook and one sheet, and one fixture per tag adding only what
proposes that tag. Asserts each proposal's line, the producer read from
docProps/app.xml, the chartex and threaded words, that a marker written into a
fixture's names, text and attribute values is never printed, and the refusal
of a file that is not a zip (one "refused: " line on stderr, nothing on
stdout, exit 1).

Prints one line per assertion, "ok" or "FAIL" then what was asserted, and a
last line "corpus-tags-test: PASS" or "corpus-tags-test: FAIL"; exits 0 or 1.
"""
import os
import subprocess
import sys
import tempfile
import zipfile

SCRIPT = os.path.join(os.path.dirname(os.path.realpath(__file__)), "corpus-tags.py")
NS = 'xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"'
X14 = 'xmlns:x14="http://schemas.microsoft.com/office/spreadsheetml/2009/9/main"'
XM = 'xmlns:xm="http://schemas.microsoft.com/office/excel/2006/main"'
MARK = "ZQXMARKER"
CT = ('<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">'
      '<Default Extension="xml" ContentType="application/xml"/></Types>')

failed = 0


def check(ok, what):
    global failed
    print("%s %s" % ("ok" if ok else "FAIL", what))
    if not ok:
        failed += 1


def workbook(inner="", pr=""):
    return '<?xml version="1.0"?><workbook %s>%s<sheets><sheet name="S" sheetId="1"/></sheets>%s</workbook>' % (NS, pr, inner)


def sheet(inner="", tail="", attrs=""):
    return '<?xml version="1.0"?><worksheet %s %s><sheetData>%s</sheetData>%s</worksheet>' % (NS, attrs, inner, tail)


def app(application):
    return ('<?xml version="1.0"?><Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties">'
            '<Application>%s</Application></Properties>' % application)


def build(path, parts, large=0):
    entries = {"[Content_Types].xml": CT, "xl/workbook.xml": workbook(), "xl/worksheets/sheet1.xml": sheet()}
    entries.update(parts)
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as z:
        for name, data in entries.items():
            z.writestr(name, data)
        if large:
            with z.open("fill.bin", "w", force_zip64=True) as f:
                block = b"\0" * (1 << 20)
                left = large
                while left:
                    n = min(left, len(block))
                    f.write(block[:n])
                    left -= n


def run(path):
    p = subprocess.run([sys.executable, "-I", "-B", SCRIPT, path], capture_output=True, text=True)
    return p.returncode, p.stdout, p.stderr


# One fixture per tag: the parts it adds to the base.
FIXTURES = {
    "plain": {},
    "strings": {"xl/sharedStrings.xml": '<sst %s><si><t>%s</t></si></sst>' % (NS, MARK)},
    "formulas": {"xl/worksheets/sheet1.xml": sheet('<row r="1"><c r="A1"><f>1+1</f></c></row>')},
    "names": {"xl/workbook.xml": workbook('<definedNames><definedName name="%s">S!$A$1</definedName></definedNames>' % MARK)},
    "merge": {"xl/worksheets/sheet1.xml": sheet(tail='<mergeCells count="1"><mergeCell ref="A1:B1"/></mergeCells>')},
    "validation": {"xl/worksheets/sheet1.xml": sheet(tail='<dataValidations count="1"><dataValidation sqref="A1"/></dataValidations>')},
    "hyperlinks": {"xl/worksheets/sheet1.xml": sheet(tail='<hyperlinks><hyperlink ref="A1" display="%s"/></hyperlinks>' % MARK)},
    "autofilter": {"xl/worksheets/sheet1.xml": sheet(tail='<autoFilter ref="A1:B2"/>')},
    "freeze": {"xl/worksheets/sheet1.xml": sheet().replace("<sheetData>", '<sheetViews><sheetView workbookViewId="0"><pane ySplit="1" topLeftCell="A2" state="frozen"/></sheetView></sheetViews><sheetData>')},
    "cf": {"xl/worksheets/sheet1.xml": sheet(tail='<conditionalFormatting sqref="A1"><cfRule type="expression" priority="1"><formula>1</formula></cfRule></conditionalFormatting>')},
    "tables": {"xl/tables/table1.xml": '<table %s id="1" name="%s" ref="A1:B2"/>' % (NS, MARK)},
    "charts": {"xl/charts/chart1.xml": "<chartSpace/>"},
    "pivots": {"xl/pivotTables/pivotTable1.xml": "<pivotTableDefinition/>"},
    "slicers": {"xl/slicers/slicer1.xml": "<slicers/>"},
    "timelines": {"xl/timelines/timeline1.xml": "<timelines/>"},
    "macros": {"xl/vbaProject.bin": MARK},
    "comments": {"xl/comments1.xml": '<comments %s><commentList><comment ref="A1"><text><t>%s</t></text></comment></commentList></comments>' % (NS, MARK)},
    "images": {"xl/media/image1.png": "fictional"},
    "external": {"xl/externalLinks/externalLink1.xml": "<externalLink/>"},
    "sparklines": {"xl/worksheets/sheet1.xml": sheet(tail='<extLst><ext uri="{05C60535-1F16-4fd2-B633-F4F36F0B64E0}" %s><x14:sparklineGroups %s><x14:sparklineGroup/></x14:sparklineGroups></ext></extLst>' % (X14, XM))},
    "protection": {"xl/worksheets/sheet1.xml": sheet(tail='<sheetProtection sheet="1"/>')},
    "date1904": {"xl/workbook.xml": workbook(pr='<workbookPr date1904="1"/>')},
    "large": {},
}


def main():
    with tempfile.TemporaryDirectory() as d:
        for tag, parts in FIXTURES.items():
            path = os.path.join(d, "%s-%s.xlsx" % (MARK, tag))
            build(path, parts, large=60000001 if tag == "large" else 0)
            rc, out, err = run(path)
            fields = out.split()
            want = "plain" if tag == "plain" else "plain," + tag
            check(rc == 0 and err == "" and out.count("\n") == 1, "%s: exit 0, one line, empty stderr" % tag)
            check(len(fields) == 4 and fields[0] == "xlsx" and fields[1] == str(os.path.getsize(path))
                  and fields[2] == "unknown" and fields[3] == want,
                  "%s: the line reads xlsx, the byte count, unknown, %s" % (tag, want))
            check(MARK not in out and MARK not in err, "%s: the fixture's marker is not printed" % tag)

        # More fixtures, each beside one other fact.
        cases = [
            ("protection by the workbook", {"xl/workbook.xml": workbook('<workbookProtection lockStructure="1"/>')}, "xlsx", "unknown", "plain,protection", []),
            ("date1904 written true", {"xl/workbook.xml": workbook(pr='<workbookPr date1904="true"/>')}, "xlsx", "unknown", "plain,date1904", []),
            ("date1904 written 0 is not proposed", {"xl/workbook.xml": workbook(pr='<workbookPr date1904="0"/>')}, "xlsx", "unknown", "plain", []),
            ("a pane that is not frozen is not freeze", {"xl/worksheets/sheet1.xml": sheet().replace("<sheetData>", '<sheetViews><sheetView workbookViewId="0"><pane ySplit="1" state="split"/></sheetView></sheetViews><sheetData>')}, "xlsx", "unknown", "plain", []),
            ("an x14 xm:f is not a formula", {"xl/worksheets/sheet1.xml": sheet(tail='<extLst><ext uri="{78C0D931-6437-407d-A8EE-F0AAD7539E65}" %s><x14:conditionalFormattings %s><x14:conditionalFormatting><x14:cfRule type="expression"><xm:f>1</xm:f></x14:cfRule></x14:conditionalFormatting></x14:conditionalFormattings></ext></extLst>' % (X14, XM))}, "xlsx", "unknown", "plain,cf", []),
            ("mergeCells without a mergeCell is not merge", {"xl/worksheets/sheet1.xml": sheet(tail='<mergeCells count="0"/>')}, "xlsx", "unknown", "plain", []),
            ("a chartex part is charts with the word chartex", {"xl/charts/chartEx1.xml": "<chartSpace/>"}, "xlsx", "unknown", "plain,charts", ["chartex"]),
            ("threaded comments are comments with the word threaded", {"xl/threadedComments/threadedComment1.xml": "<ThreadedComments/>"}, "xlsx", "unknown", "plain,comments", ["threaded"]),
            ("Excel for Windows by docProps/app.xml", {"docProps/app.xml": app("Microsoft Excel")}, "xlsx", "excel-win", "plain", []),
            ("Excel for Mac by docProps/app.xml", {"docProps/app.xml": app("Microsoft Macintosh Excel")}, "xlsx", "excel-mac", "plain", []),
            ("another application is unknown", {"docProps/app.xml": app(MARK)}, "xlsx", "unknown", "plain", []),
            ("a malformed docProps/app.xml is unknown", {"docProps/app.xml": "<Properties"}, "xlsx", "unknown", "plain", []),
        ]
        for i, (what, parts, ext, prod, tags, words) in enumerate(cases):
            path = os.path.join(d, "%s-case%d.xlsx" % (MARK, i))
            build(path, parts)
            rc, out, err = run(path)
            want = [ext, str(os.path.getsize(path)), prod, tags] + words
            check(rc == 0 and err == "" and out.split() == want and MARK not in out,
                  "%s: the line reads %s" % (what, " ".join(want[:1] + ["<bytes>"] + want[2:])))

        # The extension: one of the four is printed, any other is "other".
        for ext, want in (("xlsm", "xlsm"), ("XLTX", "xltx"), (MARK, "other")):
            path = os.path.join(d, "fixture." + ext)
            build(path, {})
            rc, out, err = run(path)
            check(rc == 0 and out.split()[:1] == [want] and MARK not in out, "the extension %s reads %s" % ("<marker>" if ext == MARK else ext, want))

        # The refusal: a file that is not a zip.
        path = os.path.join(d, "%s-notzip.xlsx" % MARK)
        with open(path, "w") as f:
            f.write("not a zip %s\n" % MARK)
        rc, out, err = run(path)
        lines = err.splitlines()
        check(rc == 1 and out == "" and len(lines) == 1 and lines[0].startswith("refused: ") and MARK not in err,
              "a file that is not a zip: exit 1, nothing on stdout, one refused line on stderr")
        # The refusal of a zip whose head is right but whose directory is gone.
        path = os.path.join(d, "%s-cut.xlsx" % MARK)
        build(os.path.join(d, "whole.xlsx"), {})
        with open(os.path.join(d, "whole.xlsx"), "rb") as f:
            data = f.read()
        with open(path, "wb") as f:
            f.write(data[:len(data) // 2])
        rc, out, err = run(path)
        lines = err.splitlines()
        check(rc == 1 and out == "" and len(lines) == 1 and lines[0].startswith("refused: "),
              "a zip cut in half: exit 1, nothing on stdout, one refused line on stderr")
        # Read-only: the file is unchanged by a proposal.
        path = os.path.join(d, "whole.xlsx")
        before = (os.stat(path).st_mtime_ns, data)
        run(path)
        with open(path, "rb") as f:
            check((os.stat(path).st_mtime_ns, f.read()) == before, "the file read is unchanged, bytes and mtime")

    print("corpus-tags-test: %s" % ("PASS" if failed == 0 else "FAIL"))
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
