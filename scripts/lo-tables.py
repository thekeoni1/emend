#!/usr/bin/env python3
"""Count a workbook's sheets and a LibreOffice conversion's tables
(specs/0047-harness-findings.md, "The diagnosis record (P1)").

usage: lo-tables.py [--root <dir>] (--workbook <file> | --ods <file>)...
       lo-tables.py --self-test

Reads each file and never writes. For a workbook it prints:
  workbook path=<path> sheets=<n> external_references=<n>
      external_sheet_names=<n> book_views=<yes|no> active_tab=<n|absent>
      pivot_caches=<n> pivot_cache_definitions=<n>
  workbook_sheet index=<i> state=<state>          (one per <sheet>)
  pivot_cache_definition index=<i> source=<kept|missing|none>
                                                  (one per definition part)
on one line each. <state> is the sheet's state attribute, visible when it has
none. active_tab is the first <workbookView>'s activeTab, absent when there is
no <bookViews>, no view in it or no attribute on the view. The external sheet
names are the <sheetName> elements over the parts the workbook's relationships
type externalLink; the definitions are the parts they type
pivotCacheDefinition, in relationship order. A definition's source is kept
when its <worksheetSource> has a sheet attribute naming one of the workbook's
sheets, missing when it names none of them, and none when there is no
<worksheetSource> or no sheet attribute on it.
For a conversion (an ods archive) it prints:
  ods path=<path> tables=<n>
  ods_table index=<i> name_length=<n> url=<yes|no> hash=<yes|no>
      print=<value|absent> display=<value|absent>  (one per <table:table>)
url is yes when the table's name holds :// or starts with file:, hash when it
holds #; print is the table's table:print and display its style's
table:display (content.xml's automatic styles, then styles.xml), absent when
missing.
No sheet name, table name, defined name or cell content is ever printed: names
are compared in memory and only their lengths and the booleans above leave the
script. <path> is the path as given, or relative to --root when one is given
(a file outside the root is refused). Every line is checked before any is
printed: a line holding a byte outside printable ASCII (0x20 to 0x7E) is
refused, nothing goes to stdout, one "refused: " line goes to stderr and the
exit is 2. Any other failure prints one "error: " line naming only the
exception class and exits 2.

--self-test builds fictional fixtures in a temporary directory, runs the
script over them as a child process, prints "ok" or "FAIL" with what each
assertion checks, and ends "lo-tables self-test: PASS" (exit 0) or
"lo-tables self-test: FAIL" (exit 1).
"""
import os
import posixpath
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
import zipfile

USAGE = "usage: lo-tables.py [--root <dir>] (--workbook <file> | --ods <file>)... | --self-test"
# The transitional and strict main namespaces.
MAIN = {"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        "http://purl.oclc.org/ooxml/spreadsheetml/main"}
RELS = "http://schemas.openxmlformats.org/package/2006/relationships"
TABLE = "urn:oasis:names:tc:opendocument:xmlns:table:1.0"
STYLE = "urn:oasis:names:tc:opendocument:xmlns:style:1.0"


class Refused(Exception):
    pass


def split(tag):
    if tag.startswith("{"):
        ns, _, local = tag[1:].partition("}")
        return ns, local
    return "", tag


def main_elements(z, name, local):
    """The elements of one entry with this local name in a main namespace."""
    root = ET.fromstring(z.read(name))
    return [e for e in root.iter() if split(e.tag) in {(ns, local) for ns in MAIN}]


def relationships(z, source):
    """(type, target part) for each relationship of the part `source`, in
    order; the target resolved against the source's directory."""
    folder, base = posixpath.split(source)
    rels = posixpath.join(folder, "_rels", base + ".rels")
    if rels not in z.namelist():
        return []
    out = []
    for e in ET.fromstring(z.read(rels)).iter("{%s}Relationship" % RELS):
        if e.get("TargetMode") == "External":
            continue
        target = e.get("Target", "")
        if target.startswith("/"):
            part = posixpath.normpath(target[1:])
        else:
            part = posixpath.normpath(posixpath.join(folder, target))
        out.append((e.get("Type", ""), part))
    return out


def workbook_lines(path, shown):
    lines = []
    with zipfile.ZipFile(path) as z:
        book = [p for t, p in relationships(z, "") if t.endswith("/officeDocument")]
        if len(book) != 1:
            raise ValueError("no single officeDocument relationship")
        book = book[0]
        sheets = main_elements(z, book, "sheet")
        names = {s.get("name") for s in sheets}
        externals = main_elements(z, book, "externalReference")
        views = main_elements(z, book, "bookViews")
        first = main_elements(z, book, "workbookView")
        active = "absent"
        if views and first and first[0].get("activeTab") is not None:
            active = first[0].get("activeTab")
        caches = main_elements(z, book, "pivotCache")
        rels = relationships(z, book)
        sheet_names = 0
        for t, part in rels:
            if t.endswith("/externalLink"):
                sheet_names += len(main_elements(z, part, "sheetName"))
        sources = []
        for t, part in rels:
            if t.endswith("/pivotCacheDefinition"):
                ws = main_elements(z, part, "worksheetSource")
                sheet = ws[0].get("sheet") if ws else None
                if sheet is None:
                    sources.append("none")
                else:
                    sources.append("kept" if sheet in names else "missing")
    lines.append("workbook path=%s sheets=%d external_references=%d external_sheet_names=%d "
                 "book_views=%s active_tab=%s pivot_caches=%d pivot_cache_definitions=%d"
                 % (shown, len(sheets), len(externals), sheet_names, "yes" if views else "no",
                    active, len(caches), len(sources)))
    for i, s in enumerate(sheets, 1):
        lines.append("workbook_sheet index=%d state=%s" % (i, s.get("state", "visible")))
    for i, source in enumerate(sources, 1):
        lines.append("pivot_cache_definition index=%d source=%s" % (i, source))
    return lines


def table_styles(stream, display):
    """Adds style name -> table:display for every style holding table
    properties in the stream; returns the tables seen as (name, print,
    style name) when the stream is content.xml."""
    tables = []
    style = None
    for event, e in ET.iterparse(stream, events=("start", "end")):
        if event == "start":
            if e.tag == "{%s}style" % STYLE:
                style = e.get("{%s}name" % STYLE)
            elif e.tag == "{%s}table-properties" % STYLE and style is not None:
                display.setdefault(style, e.get("{%s}display" % TABLE))
            elif e.tag == "{%s}table" % TABLE:
                tables.append((e.get("{%s}name" % TABLE, ""), e.get("{%s}print" % TABLE),
                               e.get("{%s}style-name" % TABLE)))
        else:
            if e.tag == "{%s}style" % STYLE:
                style = None
            e.clear()
    return tables


def ods_lines(path, shown):
    display = {}
    with zipfile.ZipFile(path) as z:
        with z.open("content.xml") as f:
            tables = table_styles(f, display)
        if "styles.xml" in z.namelist():
            with z.open("styles.xml") as f:
                table_styles(f, display)
    lines = ["ods path=%s tables=%d" % (shown, len(tables))]
    for i, (name, printed, style) in enumerate(tables, 1):
        url = "://" in name or name.startswith("file:")
        shown_display = display.get(style) if style is not None else None
        lines.append("ods_table index=%d name_length=%d url=%s hash=%s print=%s display=%s"
                     % (i, len(name), "yes" if url else "no", "yes" if "#" in name else "no",
                        "absent" if printed is None else printed,
                        "absent" if shown_display is None else shown_display))
    return lines


def printable(line):
    return all(0x20 <= b <= 0x7E for b in os.fsencode(line))


def shown_path(path, root):
    if root is None:
        return path
    full = os.path.abspath(path)
    top = os.path.abspath(root)
    if not full.startswith(top + os.sep):
        raise Refused("a file is outside the root")
    return os.path.relpath(full, top)


def run(argv):
    root = None
    files = []
    i = 0
    while i < len(argv):
        if argv[i] in ("--root", "--workbook", "--ods") and i + 1 < len(argv):
            if argv[i] == "--root":
                if root is not None or files:
                    raise Refused("--root is given once, first")
                root = argv[i + 1]
            else:
                files.append((argv[i], argv[i + 1]))
            i += 2
        else:
            raise Refused(USAGE)
    if not files:
        raise Refused(USAGE)
    lines = []
    for kind, path in files:
        shown = shown_path(path, root)
        lines += workbook_lines(path, shown) if kind == "--workbook" else ods_lines(path, shown)
    for line in lines:
        if not printable(line):
            raise Refused("a line holds a byte outside printable ASCII")
    sys.stdout.write("".join(line + "\n" for line in lines))


# ------------------------------------------------------------------ the self-test

MARK = "ZQXMARKER"
NS = 'xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"'
STRICT = 'xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main"'
REL_NS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
PKG = '<Relationships xmlns="%s">%%s</Relationships>' % RELS


def rel(n, kind, target, external=False):
    return '<Relationship Id="rId%d" Type="%s/%s" Target="%s"%s/>' % (
        n, REL_NS, kind, target, ' TargetMode="External"' if external else "")


def write_zip(path, entries):
    with zipfile.ZipFile(path, "w") as z:
        for name, data in entries:
            z.writestr(name, data)


def book(ns, body, rels):
    return [("_rels/.rels", PKG % rel(1, "officeDocument", "xl/workbook.xml")),
            ("xl/workbook.xml", '<workbook %s xmlns:r="%s">%s</workbook>' % (ns, REL_NS, body)),
            ("xl/_rels/workbook.xml.rels", PKG % "".join(rels))]


def fixtures(d):
    full = os.path.join(d, "full.xlsx")
    body = ('<bookViews><workbookView activeTab="2"/><workbookView activeTab="0"/></bookViews>'
            '<sheets><sheet name="%s1" sheetId="1" r:id="rId1"/>'
            '<sheet name="%s2" sheetId="2" state="hidden" r:id="rId2"/>'
            '<sheet name="%s3" sheetId="3" state="veryHidden" r:id="rId3"/></sheets>'
            '<externalReferences><externalReference r:id="rId4"/></externalReferences>'
            '<definedNames><definedName name="%sN">%s1!A1</definedName></definedNames>'
            '<pivotCaches><pivotCache cacheId="1" r:id="rId5"/><pivotCache cacheId="2" r:id="rId6"/>'
            '<pivotCache cacheId="3" r:id="rId7"/></pivotCaches>') % ((MARK,) * 5)
    rels = [rel(1, "worksheet", "worksheets/sheet1.xml"), rel(2, "worksheet", "worksheets/sheet2.xml"),
            rel(3, "worksheet", "worksheets/sheet3.xml"),
            rel(4, "externalLink", "externalLinks/externalLink1.xml"),
            rel(5, "pivotCacheDefinition", "pivotCache/pivotCacheDefinition1.xml"),
            rel(6, "pivotCacheDefinition", "/xl/pivotCache/pivotCacheDefinition2.xml"),
            rel(7, "pivotCacheDefinition", "pivotCache/pivotCacheDefinition3.xml"),
            rel(8, "hyperlink", "https://example.com/x", external=True)]
    link = ('<externalLink %s><externalBook><sheetNames><sheetName val="%sA"/><sheetName val="%sB"/>'
            '</sheetNames><sheetDataSet><sheetData sheetId="0"/></sheetDataSet></externalBook>'
            '</externalLink>') % (NS, MARK, MARK)

    def cache(source):
        return ('<pivotCacheDefinition %s><cacheSource type="worksheet">%s</cacheSource>'
                '</pivotCacheDefinition>') % (NS, source)
    write_zip(full, book(NS, body, rels) + [
        ("xl/externalLinks/externalLink1.xml", link),
        ("xl/pivotCache/pivotCacheDefinition1.xml",
         cache('<worksheetSource ref="A1:B2" sheet="%s2"/>' % MARK)),
        ("xl/pivotCache/pivotCacheDefinition2.xml",
         cache('<worksheetSource ref="A1:B2" sheet="%sGone"/>' % MARK)),
        ("xl/pivotCache/pivotCacheDefinition3.xml",
         cache('<worksheetSource name="%sTable"/>' % MARK))])
    noviews = os.path.join(d, "noviews.xlsx")
    write_zip(noviews, book(NS, '<sheets><sheet name="%s" sheetId="1" r:id="rId1"/></sheets>' % MARK,
                            [rel(1, "worksheet", "worksheets/sheet1.xml")]))
    noattr = os.path.join(d, "noattr.xlsx")
    write_zip(noattr, book(STRICT, '<bookViews><workbookView/></bookViews><sheets>'
                           '<sheet name="%sx" sheetId="1" r:id="rId1"/>'
                           '<sheet name="%sy" sheetId="2" r:id="rId2"/></sheets>' % (MARK, MARK),
                           [rel(1, "worksheet", "worksheets/sheet1.xml")]))
    return full, noviews, noattr


def ods(path, tables, print_value="false"):
    office = "urn:oasis:names:tc:opendocument:xmlns:office:1.0"
    content = ('<office:document-content xmlns:office="%s" xmlns:table="%s" xmlns:style="%s">'
               '<office:automatic-styles>'
               '<style:style style:name="ta1" style:family="table">'
               '<style:table-properties table:display="true"/></style:style>'
               '<style:style style:name="ta2" style:family="table">'
               '<style:table-properties/></style:style>'
               '</office:automatic-styles><office:body><office:spreadsheet>%s'
               '</office:spreadsheet></office:body></office:document-content>') % (
        office, TABLE, STYLE, tables % {"print": print_value})
    styles = ('<office:document-styles xmlns:office="%s" xmlns:table="%s" xmlns:style="%s">'
              '<office:styles><style:style style:name="ta3" style:family="table">'
              '<style:table-properties table:display="false"/></style:style></office:styles>'
              '</office:document-styles>') % (office, TABLE, STYLE)
    write_zip(path, [("mimetype", "application/vnd.oasis.opendocument.spreadsheet"),
                     ("content.xml", content), ("styles.xml", styles)])


ODS_TABLES = ('<table:table table:name="%s" table:style-name="ta1"><table:table-row><table:table-cell>'
              '<text:p xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0">%sC</text:p>'
              '</table:table-cell></table:table-row></table:table>'
              '<table:table table:name="&apos;https://example.com/%s.xlsx&apos;#$%s" '
              'table:print="%%(print)s" table:style-name="ta2"/>'
              '<table:table table:name="file:%s#x" table:style-name="ta3"/>'
              '<table:table table:name="%s://"/>') % ((MARK + "abc",) + (MARK,) * 5)

failed = 0


def check(ok, what):
    global failed
    print("%s %s" % ("ok" if ok else "FAIL", what))
    if not ok:
        failed += 1


def child(args):
    out = subprocess.run([sys.executable, "-I", "-B", os.path.realpath(__file__)] + args,
                         capture_output=True)
    return out.returncode, out.stdout.decode("ascii", "replace"), out.stderr.decode("ascii", "replace")


def self_test():
    with tempfile.TemporaryDirectory(prefix="lo-tables-self-test.") as d:
        full, noviews, noattr = fixtures(d)
        rc, out, err = child(["--root", d, "--workbook", full, "--workbook", noviews,
                              "--workbook", noattr])
        lines = out.splitlines()
        check(rc == 0 and err == "", "three workbooks: exit 0, stderr empty")
        check(lines[:1] == ["workbook path=full.xlsx sheets=3 external_references=1 "
                            "external_sheet_names=2 book_views=yes active_tab=2 pivot_caches=3 "
                            "pivot_cache_definitions=3"],
              "the full workbook's counts, the first view's active tab, path relative to the root")
        check(lines[1:4] == ["workbook_sheet index=1 state=visible", "workbook_sheet index=2 state=hidden",
                             "workbook_sheet index=3 state=veryHidden"],
              "each sheet's state, visible when absent")
        check(lines[4:7] == ["pivot_cache_definition index=1 source=kept",
                             "pivot_cache_definition index=2 source=missing",
                             "pivot_cache_definition index=3 source=none"],
              "the definitions' sources kept, missing and none, relative and absolute targets")
        check(lines[7:9] == ["workbook path=noviews.xlsx sheets=1 external_references=0 "
                             "external_sheet_names=0 book_views=no active_tab=absent pivot_caches=0 "
                             "pivot_cache_definitions=0", "workbook_sheet index=1 state=visible"],
              "no bookViews: book_views=no, active_tab=absent")
        check(lines[9:] == ["workbook path=noattr.xlsx sheets=2 external_references=0 "
                            "external_sheet_names=0 book_views=yes active_tab=absent pivot_caches=0 "
                            "pivot_cache_definitions=0", "workbook_sheet index=1 state=visible",
                            "workbook_sheet index=2 state=visible"],
              "a strict workbook whose view has no activeTab: book_views=yes, active_tab=absent")
        check(MARK not in out, "no sheet, defined or external name, and no source name, is printed")

        conv = os.path.join(d, "conv.ods")
        ods(conv, ODS_TABLES)
        rc, out, err = child(["--ods", conv])
        lines = out.splitlines()
        check(rc == 0 and err == "", "an ods: exit 0, stderr empty")
        check(lines == ["ods path=%s tables=4" % conv,
                        "ods_table index=1 name_length=12 url=no hash=no print=absent display=true",
                        "ods_table index=2 name_length=%d url=yes hash=yes print=false display=absent"
                        % len("'https://example.com/%s.xlsx'#$%s" % (MARK, MARK)),
                        "ods_table index=3 name_length=%d url=yes hash=yes print=absent display=false"
                        % len("file:%s#x" % MARK),
                        "ods_table index=4 name_length=%d url=yes hash=no print=absent display=absent"
                        % len(MARK + "://")],
              "every table column: a plain name, :// with #, file: with #, :// without #; print; "
              "display from content.xml, from styles.xml, and absent; path as given without a root")
        check(MARK not in out, "no table name and no cell content is printed")

        rc, out, err = child(["--root", d, "--ods", conv, "--workbook", noviews])
        check(rc == 0 and out.splitlines()[0] == "ods path=conv.ods tables=4"
              and out.splitlines()[5].startswith("workbook path=noviews.xlsx "),
              "an ods and a workbook in one call, in the order given")

        # The refusal and its control: a fixture whose NAME holds a byte
        # outside printable ASCII is printed as a path, so the line is refused.
        odd = os.path.join(os.fsencode(d), b"t\xe9.ods")
        os.link(conv, odd)
        out = subprocess.run([os.fsencode(sys.executable), b"-I", b"-B",
                              os.fsencode(os.path.realpath(__file__)), b"--root", os.fsencode(d),
                              b"--ods", odd], capture_output=True)
        check(out.returncode == 2 and out.stdout == b""
              and out.stderr.startswith(b"refused: ") and out.stderr.count(b"\n") == 1,
              "a fixture name with a byte outside printable ASCII: exit 2, refused, nothing printed")
        check(b"\xe9" not in out.stderr, "the refusal does not print the byte")
        ctl = os.path.join(d, "t.ods")
        os.link(conv, ctl)
        rc, out, err = child(["--root", d, "--ods", ctl])
        check(rc == 0 and out.startswith("ods path=t.ods tables=4\n"),
              "control: the same file under a printable name is printed")
        nonascii = os.path.join(d, "print.ods")
        ods(nonascii, ODS_TABLES, print_value="fals\u00e9")
        rc, out, err = child(["--ods", conv, "--ods", nonascii])
        check(rc == 2 and out == "" and err.startswith("refused: "),
              "an attribute value outside printable ASCII is refused too, and no earlier line printed")
        tab = os.path.join(d, "tab.ods")
        # A character reference: a literal tab in an attribute is normalised
        # to a space by the parser.
        ods(tab, ODS_TABLES, print_value="a&#9;b")
        rc, out, err = child(["--ods", tab])
        check(rc == 2 and out == "" and err.startswith("refused: "), "a tab is outside the range")

        rc, out, err = child(["--root", os.path.join(d, "sub"), "--ods", conv])
        check(rc == 2 and out == "" and err.startswith("refused: "), "a file outside the root is refused")
        notzip = os.path.join(d, "notzip.ods")
        with open(notzip, "w") as f:
            f.write("not a zip")
        rc, out, err = child(["--ods", notzip])
        check(rc == 2 and out == "" and err == "error: BadZipFile\n",
              "a file that is not a zip: one error line naming the class")
        rc, out, err = child([])
        check(rc == 2 and out == "" and err.startswith("refused: usage:"), "no file: the usage")
    print("lo-tables self-test: %s" % ("FAIL" if failed else "PASS"))
    return 1 if failed else 0


if __name__ == "__main__":
    if sys.argv[1:] == ["--self-test"]:
        sys.exit(self_test())
    try:
        run(sys.argv[1:])
    except Refused as e:
        print("refused: %s" % e, file=sys.stderr)
        sys.exit(2)
    except Exception as e:
        print("error: %s" % type(e).__name__, file=sys.stderr)
        sys.exit(2)
