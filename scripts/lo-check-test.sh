#!/usr/bin/env bash
# The test of the open check's count rule, scripts/lo-check.sh's
# lo_count_check (specs/0047-harness-findings.md, "The fixes (P2)",
# decision 1 A, and "Planning rulings during P1", ruling D).
#
# usage: scripts/lo-check-test.sh
#
# Builds fictional workbook and ODS archives with python3 -I -B in a
# temporary directory, runs lo_count_check over each pair and asserts its exit
# and its reason:
#   (i)   one sheet, one external sheet name, one displayed table and one
#         cache table: ok;
#   (ii)  one sheet, no external reference, one table: ok;
#   (iii) one sheet, no external reference, one table and one cache table:
#         fail, one cache against no external sheet name;
#   (iv)  two sheets, one table: fail;
#   (v)   control: one sheet, a displayed table and a table whose name holds
#         :// and # but has no table:print, which counts as a sheet: fail.
# A cache table is one whose table:name holds :// or starts with file:, also
# holds #, and whose table:print is false. Every name is fictional. No
# conversion runs, so neither podman nor the image is needed.
#
# Prints one line per assertion, "ok" or "FAIL" then the case and what was
# asserted, and a last line "lo-check-test: PASS" or "lo-check-test: FAIL";
# exits 0 or 1, or 2 when the fixtures cannot be built.
set -uo pipefail

[ "$#" -eq 0 ] || { echo "usage: scripts/lo-check-test.sh"; exit 2; }
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd) || exit 2
. "$ROOT/scripts/lo-check.sh" || { echo "error: cannot source scripts/lo-check.sh"; exit 2; }

failed=0
check() {
  if [ "$1" = yes ]; then
    echo "ok $2"
  else
    echo "FAIL $2"
    failed=$((failed + 1))
  fi
}
yn() { if "$@"; then echo yes; else echo no; fi; }

WORK=$(mktemp -d) || exit 2
trap 'rm -r "${WORK:?}"' EXIT

# Each case's directory gets workbook.zip and conversion.zip. A case is
# "<sheets> <external sheet names> <table>...", a table being displayed,
# cache (a cache table) or noprint (a cache-like name with no table:print).
python3 -I -B - "$WORK" <<'EOF' || { echo "error: cannot build the fixtures"; exit 2; }
import os, sys, zipfile

MAIN = "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
R = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
PKG = "http://schemas.openxmlformats.org/package/2006/relationships"
CASES = {
    "i": (1, 1, ["displayed", "cache"]),
    "ii": (1, 0, ["displayed"]),
    "iii": (1, 0, ["displayed", "cache"]),
    "iv": (2, 0, ["displayed"]),
    "v": (1, 0, ["displayed", "noprint"]),
}
TABLES = {
    "displayed": '<table:table table:name="Fictional"/>',
    "cache": ('<table:table table:name="&apos;https://example.com/fictional&apos;#$SheetA"'
              ' table:print="false"/>'),
    "noprint": '<table:table table:name="&apos;https://example.com/fictional&apos;#$SheetA"/>',
}


def workbook(path, sheets, names):
    rels = ['<Relationship Id="rId%d" Type="%s/worksheet" Target="worksheets/sheet%d.xml"/>'
            % (i, R, i) for i in range(1, sheets + 1)]
    parts = {}
    refs = ""
    if names:
        rels.append('<Relationship Id="rIdX" Type="%s/externalLink" '
                    'Target="externalLinks/externalLink1.xml"/>' % R)
        refs = '<externalReferences><externalReference r:id="rIdX"/></externalReferences>'
        parts["xl/externalLinks/externalLink1.xml"] = (
            '<externalLink xmlns="%s"><externalBook><sheetNames>%s</sheetNames>'
            '</externalBook></externalLink>'
            % (MAIN, "".join('<sheetName val="SheetA%d"/>' % i for i in range(names))))
    parts["xl/workbook.xml"] = (
        '<workbook xmlns="%s" xmlns:r="%s"><sheets>%s</sheets>%s</workbook>'
        % (MAIN, R, "".join('<sheet name="Sheet%d" sheetId="%d" r:id="rId%d"/>' % (i, i, i)
                            for i in range(1, sheets + 1)), refs))
    parts["xl/_rels/workbook.xml.rels"] = '<Relationships xmlns="%s">%s</Relationships>' % (PKG, "".join(rels))
    with zipfile.ZipFile(path, "w") as z:
        for name, text in parts.items():
            z.writestr(name, text)


def conversion(path, tables):
    content = ('<office:document-content'
               ' xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"'
               ' xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0">'
               '<office:body><office:spreadsheet>%s</office:spreadsheet></office:body>'
               '</office:document-content>' % "".join(TABLES[t] for t in tables))
    with zipfile.ZipFile(path, "w") as z:
        z.writestr("content.xml", content)


for case, (sheets, names, tables) in CASES.items():
    d = os.path.join(sys.argv[1], case)
    os.mkdir(d)
    workbook(os.path.join(d, "workbook.zip"), sheets, names)
    conversion(os.path.join(d, "conversion.zip"), tables)
EOF

# run_case <case> <expected exit> <expected reason, empty for none>: the rule
# over the case's pair, its exit and reason file compared.
run_case() {
  local case=$1 want=$2 want_reason=$3 rc got
  lo_count_check "$WORK/$case/workbook.zip" "$WORK/$case/conversion.zip" "$WORK/$case/reason.txt"
  rc=$?
  check "$(yn [ "$rc" = "$want" ])" "($case) lo_count_check exits $want (got $rc)"
  if [ -z "$want_reason" ]; then
    check "$(yn [ ! -e "$WORK/$case/reason.txt" ])" "($case) no reason is written"
  else
    got=$(cat "$WORK/$case/reason.txt" 2>&1)
    check "$(yn [ "$got" = "$want_reason" ])" "($case) the reason reads: $want_reason"
  fi
}

run_case i 0 ""
run_case ii 0 ""
run_case iii 1 "the ods holds 2 table elements, 1 of them external-link caches, and the workbook 1 sheet elements and 0 external sheet names"
run_case iv 1 "the ods holds 1 table elements, 0 of them external-link caches, and the workbook 2 sheet elements and 0 external sheet names"
run_case v 1 "the ods holds 2 table elements, 0 of them external-link caches, and the workbook 1 sheet elements and 0 external sheet names"

if [ "$failed" -eq 0 ]; then
  echo "lo-check-test: PASS"
  exit 0
fi
echo "lo-check-test: FAIL"
exit 1
