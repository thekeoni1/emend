#!/usr/bin/env python3
"""The openpyxl side of the comparison (specs/0037-measurement-and-close.md,
"The operations").

usage: python3 -I -B openpyxl-ops.py <op> <in> <out>

Runs inside the openpyxl image (tools/openpyxl-image/Containerfile), started
by scripts/compare-run.sh. Each op loads <in> with openpyxl's ordinary
load_workbook (never read_only or write_only), makes the one request that
matches an emend-copy sample, and saves to <out>. W is the first worksheet of
kind Worksheet (wb.worksheets[0]) and M its last used row (ws.max_row); every
addition goes below M, as the emend samples place theirs.

The ops and every constant they write:

  roundtrip    load; save. (emend: emend-copy <in> <out>)
  read         load; every cell of every worksheet visited by iter_rows and
               counted, the count printed as "cells=<n>"; save, since openpyxl
               has no other output to hand the harness. (--read-sample)
  edit         EDIT_NUMBER 42.5 in (M+1, A), EDIT_TEXT "emend compare" in
               (M+1, B), EDIT_BOOLEAN True in (M+1, C). (--edit-sample)
  format       FORMAT_NUMBER 1234.5 in (M+1, A), its number_format FORMAT_CODE
               "#,##0.00". (--format-sample)
  insert-rows  insert_rows(INSERT_AT 1, INSERT_COUNT 1) on W.
               (--insert-rows-sample)
  names        one defined name NAME "emend_compare" over W's $A$1.
               (--names-sample)
  table        TABLE_HEADERS "Item" and "Value" in (M+2, A) and (M+2, B),
               TABLE_ROW "first" and 1 in (M+3, A) and (M+3, B), a Table
               TABLE_NAME "EmendCompare" over A(M+2):B(M+3). (--table-sample)
  comment      a Comment COMMENT_TEXT "emend compare" by COMMENT_AUTHOR
               "emend" on (M+1, A). (--comment-sample)
  image        PNG, a 2 by 2 PNG held below as 73 bytes, added at (M+2, A).
               (--image-sample)
  add-chart    a BarChart, one series over B2:B(M+1), categories A2:A(M+1),
               added at (M+2, A). (--add-chart-sample)
  copy         copy_worksheet(W). (--copy-sample)

What it prints, on standard output: for read, the line "cells=<n>"; for an
exception, one line holding the exception's class name only, never its
message, which may quote workbook content; and last, whatever happened after
the op began, the line "openpyxl=<version>\\tpeak_rss_kb=<ru_maxrss>" from
resource.getrusage(RUSAGE_SELF). Nothing it prints names a path or a cell
value.

Exits: 0 when the op saved its output; 1 on an exception (its class printed);
137 on MemoryError, so the runner classes it as the cap, a MemoryError raised
while the script reports another exception included (specs/0037, ruling Y:
that path writes its two lines with bytes made before the op, by os.write,
and leaves by os._exit, so it needs no new memory to end); 2, with the one line
"unknown op" and nothing else, for an op it does not know or a wrong number of
arguments. openpyxl's own warnings go to standard error, which the runner
keeps in the run directory and never prints.
"""
import os
import resource
import sys

OPS = ("roundtrip", "read", "edit", "format", "insert-rows", "names", "table",
       "comment", "image", "add-chart", "copy")

EDIT_NUMBER = 42.5
EDIT_TEXT = "emend compare"
EDIT_BOOLEAN = True
FORMAT_NUMBER = 1234.5
FORMAT_CODE = "#,##0.00"
INSERT_AT = 1
INSERT_COUNT = 1
NAME = "emend_compare"
TABLE_HEADERS = ("Item", "Value")
TABLE_ROW = ("first", 1)
TABLE_NAME = "EmendCompare"
COMMENT_TEXT = "emend compare"
COMMENT_AUTHOR = "emend"
# A 2 by 2 RGB PNG, 73 bytes: red and blue on each row.
PNG = bytes.fromhex(
    "89504e470d0a1a0a0000000d4948445200000002000000020802000000fdd49a73"
    "000000104944415478da63f8cf0004ff192014001bf203fd3e975da40000000049"
    "454e44ae426082"
)


def run(op, source, target):
    import io
    import openpyxl
    from openpyxl.chart import BarChart, Reference
    from openpyxl.comments import Comment
    from openpyxl.drawing.image import Image
    from openpyxl.utils import quote_sheetname
    from openpyxl.workbook.defined_name import DefinedName
    from openpyxl.worksheet.table import Table

    wb = openpyxl.load_workbook(source)
    ws = wb.worksheets[0]
    m = ws.max_row
    if op == "read":
        cells = 0
        for sheet in wb.worksheets:
            for row in sheet.iter_rows():
                cells += len(row)
        print("cells=%d" % cells)
    elif op == "edit":
        ws.cell(row=m + 1, column=1, value=EDIT_NUMBER)
        ws.cell(row=m + 1, column=2, value=EDIT_TEXT)
        ws.cell(row=m + 1, column=3, value=EDIT_BOOLEAN)
    elif op == "format":
        ws.cell(row=m + 1, column=1, value=FORMAT_NUMBER).number_format = FORMAT_CODE
    elif op == "insert-rows":
        ws.insert_rows(INSERT_AT, INSERT_COUNT)
    elif op == "names":
        ref = "%s!$A$1" % quote_sheetname(ws.title)
        wb.defined_names[NAME] = DefinedName(NAME, attr_text=ref)
    elif op == "table":
        ws.cell(row=m + 2, column=1, value=TABLE_HEADERS[0])
        ws.cell(row=m + 2, column=2, value=TABLE_HEADERS[1])
        ws.cell(row=m + 3, column=1, value=TABLE_ROW[0])
        ws.cell(row=m + 3, column=2, value=TABLE_ROW[1])
        ws.add_table(Table(displayName=TABLE_NAME, ref="A%d:B%d" % (m + 2, m + 3)))
    elif op == "comment":
        ws.cell(row=m + 1, column=1).comment = Comment(COMMENT_TEXT, COMMENT_AUTHOR)
    elif op == "image":
        ws.add_image(Image(io.BytesIO(PNG)), "A%d" % (m + 2))
    elif op == "add-chart":
        chart = BarChart()
        chart.add_data(Reference(ws, min_col=2, min_row=2, max_row=m + 1), titles_from_data=False)
        chart.set_categories(Reference(ws, min_col=1, min_row=2, max_row=m + 1))
        ws.add_chart(chart, "A%d" % (m + 2))
    elif op == "copy":
        wb.copy_worksheet(ws)
    wb.save(target)
    return openpyxl.__version__


def last_line(version):
    rss = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    print("openpyxl=%s\tpeak_rss_kb=%d" % (version, rss))


# The cap's two lines, made before the op (ruling Y): the class name, and the
# last line's prefix, its version filled in once openpyxl is imported.
MEMORY_LINE = b"MemoryError\n"
NO_RSS = b"-\n"
LAST_PREFIX = [b"openpyxl=-\tpeak_rss_kb="]


def cap_exit():
    """Ends the script with 137 after a MemoryError, wherever it was raised:
    what print has buffered is flushed if it can be, the two lines go out by
    os.write, the peak RSS as "-" if even its digits cannot be made, and
    os._exit leaves whatever happens on the way."""
    try:
        try:
            sys.stdout.flush()
        except BaseException:
            pass
        os.write(1, MEMORY_LINE)
        os.write(1, LAST_PREFIX[0])
        try:
            rss = b"%d\n" % resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        except MemoryError:
            rss = NO_RSS
        os.write(1, rss)
    finally:
        os._exit(137)


def main(argv):
    if len(argv) != 3 or argv[0] not in OPS:
        print("unknown op")
        return 2
    version = "-"
    try:
        import openpyxl
        version = openpyxl.__version__
        LAST_PREFIX[0] = ("openpyxl=%s\tpeak_rss_kb=" % version).encode()
        run(*argv)
    except MemoryError:
        cap_exit()
    except Exception as err:
        print(type(err).__name__)
        last_line(version)
        return 1
    last_line(version)
    return 0


if __name__ == "__main__":
    # A MemoryError raised while another exception is reported, or anywhere
    # outside the op, ends as the cap too (ruling Y).
    try:
        code = main(sys.argv[1:])
    except MemoryError:
        cap_exit()
    sys.exit(code)
