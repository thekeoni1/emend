#!/usr/bin/env bash
# The test of scripts/openpyxl-ops.py (specs/0037-measurement-and-close.md P1,
# and "Planning rulings during P1", ruling F).
#
# usage: scripts/openpyxl-ops-test.sh <image>
#
# Regenerates scale-gen tier t into a temporary directory and asserts it is
# the committed fixture tests/fixtures/openpyxl-ops.xlsx, byte for byte and by
# the sha256 tests/fixtures/README.md names. Then runs every op of the ops
# script on the fixture inside the image, as scripts/compare-run.sh runs it
# (--network=none --userns=keep-id, the fixture and the script read-only), and
# asserts for each: exit 0, an output written, the last line
# "openpyxl=3.1.5<TAB>peak_rss_kb=<n>", nothing on standard error, and the one
# thing the op asks for, read from the output's parts (the read op's count of
# 2500 cells from its own line). Then an unknown op exits 2 with the one line
# "unknown op". Last, ruling Y of specs/0037 on two planted copies of the
# script: a MemoryError from the op exits 137 with its two lines, MemoryError
# and the last line, and nothing on standard error; and a MemoryError raised
# while another exception is reported does the same, where before it escaped
# with a traceback and exit 1.
#
# Run and printed by the phase's test log, never by the gate, so CI is
# unchanged. Prints one line per assertion, "ok" or "FAIL" then what was
# asserted, and a last line "openpyxl-ops-test: PASS" or
# "openpyxl-ops-test: FAIL"; exits 0 or 1. The fixture is fictional, and
# nothing printed is a cell value.
set -uo pipefail

[ "$#" -eq 1 ] || { echo "usage: scripts/openpyxl-ops-test.sh <image>"; exit 2; }
IMAGE=$1
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd) || exit 2
FIXTURE=$ROOT/tests/fixtures/openpyxl-ops.xlsx
readonly FIXTURE_SHA=12ec7ca04a52d9253e947e8f394d04b6d18ca1b133b90500955603a825805fdd
readonly OPS="roundtrip read edit format insert-rows names table comment image add-chart copy"

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
# last_ok <line>: the ops script's last line, openpyxl 3.1.5 and a peak RSS.
last_ok() { [[ "$1" =~ ^openpyxl=3\.1\.5$'\t'peak_rss_kb=[1-9][0-9]*$ ]]; }

WORK=$(mktemp -d) || exit 2
trap 'rm -rf "${WORK:?}"' EXIT

# The fixture is the generator's.
gen_line=$(python3 -I -B "$ROOT/scripts/scale-gen.py" t "$WORK/gen" 2>&1)
gen_rc=$?
check "$(yn [ "$gen_rc" -eq 0 ])" "scale-gen.py tier t exits 0"
read -r _ tier gen_id gen_bytes gen_sha _ <<<"$gen_line"
gen_ok() { [ "${tier:-}" = t ] && [ "${gen_sha:-}" = "$FIXTURE_SHA" ]; }
check "$(yn gen_ok)" "the generator reports tier t with the fixture's sha256"
check "$(yn [ "$(sha256sum <"$FIXTURE" | cut -d' ' -f1)" = "$FIXTURE_SHA" ])" "the committed fixture's sha256 is the one tests/fixtures/README.md names"
check "$(yn cmp -s "$WORK/gen/corpus/$gen_id.xlsx" "$FIXTURE")" "the regenerated workbook is the committed fixture, byte for byte"

mkdir -p "$WORK/out" || exit 2
# run_op <op>: the op in the image, its exit, stdout and stderr kept in WORK.
run_op() {
  podman run --rm --network=none --userns=keep-id \
    -v "$FIXTURE":/in/fixture.xlsx:ro -v "$WORK/out":/out -v "$ROOT/scripts/openpyxl-ops.py":/ops.py:ro "$IMAGE" \
    sh -c 'exec python3 -I -B /ops.py "$1" /in/fixture.xlsx "/out/$1.xlsx"' sh "$1" \
    </dev/null >"$WORK/$1.stdout" 2>"$WORK/$1.stderr"
  echo $? >"$WORK/$1.rc"
}

for op in $OPS; do
  run_op "$op"
  rc=$(cat "$WORK/$op.rc")
  check "$(yn [ "$rc" = 0 ])" "$op: exit 0"
  check "$(yn [ -f "$WORK/out/$op.xlsx" ])" "$op: an output was written"
  last=$(tail -n 1 "$WORK/$op.stdout")
  check "$(yn last_ok "$last")" "$op: the last line is openpyxl=3.1.5 and a peak RSS"
  check "$(yn [ ! -s "$WORK/$op.stderr" ])" "$op: nothing on standard error"
done
check "$(yn [ "$(head -n 1 "$WORK/read.stdout")" = cells=2500 ])" "read: the line cells=2500 (2 sheets, 2500 cells)"
check "$(yn [ "$(wc -l <"$WORK/read.stdout")" = 2 ])" "read: two lines, the count and the last line"
for op in $OPS; do
  [ "$op" = read ] && continue
  check "$(yn [ "$(wc -l <"$WORK/$op.stdout")" = 1 ])" "$op: one line, the last line"
done

# What each op asked for, read from its output's parts.
OUT=$WORK/out python3 -I -B - <<'PY' >"$WORK/parts.txt"
import os, re, zipfile

out = os.environ["OUT"]


def parts(op):
    with zipfile.ZipFile(os.path.join(out, op + ".xlsx")) as z:
        return {n: z.read(n).decode("utf-8", "replace") for n in z.namelist() if n.endswith(".xml") or n.endswith(".rels")}, z.namelist()


def first_sheet(p):
    return p.get("xl/worksheets/sheet1.xml", "")


def sheets(p):
    return len(re.findall(r"<(?:\w+:)?sheet\s", p.get("xl/workbook.xml", "")))


checks = []
p, names = parts("roundtrip")
checks.append(("roundtrip: the output holds the workbook part and 2 sheets", sheets(p) == 2))
p, names = parts("edit")
s = first_sheet(p)
checks.append(("edit: cells A101, B101 and C101 written below the last row, 100",
               all(re.search(r'<c r="%s101"' % c, s) for c in "ABC")))
checks.append(("edit: C101 is a boolean", re.search(r'<c r="C101"[^>]*t="b"', s) is not None))
p, names = parts("format")
cell = re.search(r'<c r="A101"[^>]*?s="(\d+)"', first_sheet(p))
checks.append(("format: A101 written with a style", cell is not None))
# #,##0.00 is the built-in number format 4, which openpyxl names by id.
xfs = re.search(r"<cellXfs[^>]*>(.*?)</cellXfs>", p.get("xl/styles.xml", ""), re.S)
xf = re.findall(r"<xf\b[^>]*", xfs.group(1)) if xfs else []
checks.append(("format: A101's cell format is number format 4, #,##0.00",
               cell is not None and int(cell.group(1)) < len(xf) and 'numFmtId="4"' in xf[int(cell.group(1))]))
p, names = parts("insert-rows")
s = first_sheet(p)
checks.append(("insert-rows: the first sheet's rows run 2 to 101", re.search(r'<row r="1"', s) is None
               and re.search(r'<row r="2"', s) is not None and re.search(r'<row r="101"', s) is not None))
p, names = parts("names")
checks.append(("names: the defined name emend_compare over $A$1",
               re.search(r'<definedName name="emend_compare"[^>]*>[^<]*!\$A\$1</definedName>', p.get("xl/workbook.xml", "")) is not None))
p, names = parts("table")
tables = [v for n, v in p.items() if n.startswith("xl/tables/")]
checks.append(("table: one table part, displayName EmendCompare over A102:B103",
               len(tables) == 1 and 'displayName="EmendCompare"' in tables[0] and 'ref="A102:B103"' in tables[0]))
p, names = parts("comment")
checks.append(("comment: a comments part", any(re.match(r"xl/comments", n) for n in names)))
p, names = parts("image")
checks.append(("image: one media entry", sum(1 for n in names if n.startswith("xl/media/")) == 1))
p, names = parts("add-chart")
charts = [v for n, v in p.items() if re.match(r"xl/charts/chart\d+\.xml$", n)]
checks.append(("add-chart: one chart part, a bar chart over B2:B101 and A2:A101",
               len(charts) == 1 and "barChart" in charts[0] and "$B$2:$B$101" in charts[0] and "$A$2:$A$101" in charts[0]))
p, names = parts("copy")
checks.append(("copy: 3 sheets", sheets(p) == 3))
for what, ok in checks:
    print("%s\t%s" % ("yes" if ok else "no", what))
PY
py_rc=$?
check "$(yn [ "$py_rc" -eq 0 ])" "the parts reader exits 0"
while IFS=$'\t' read -r verdict what; do
  check "$verdict" "$what"
done <"$WORK/parts.txt"
check "$(yn [ "$(wc -l <"$WORK/parts.txt")" = 12 ])" "the parts reader made its 12 checks"

# An op the script does not know.
podman run --rm --network=none --userns=keep-id -v "$ROOT/scripts/openpyxl-ops.py":/ops.py:ro "$IMAGE" \
  python3 -I -B /ops.py no-such-op /in/x.xlsx /out/x.xlsx </dev/null >"$WORK/unknown.stdout" 2>"$WORK/unknown.stderr"
rc=$?
check "$(yn [ "$rc" = 2 ])" "an unknown op exits 2"
unknown_ok() { [ "$(cat "$WORK/unknown.stdout")" = "unknown op" ] && [ ! -s "$WORK/unknown.stderr" ]; }
check "$(yn unknown_ok)" "an unknown op prints the one line unknown op"

# Ruling Y: planted copies of the script. M1 raises MemoryError from the op
# (the control: the cap's two lines). M2 raises ValueError from the op and a
# MemoryError while that exception's class is printed.
plant() {
  local name=$1 from=$2 to=$3
  python3 -I -B -c 'import sys
s = open(sys.argv[1]).read()
if s.count(sys.argv[2]) != 1:
    sys.exit(1)
open(sys.argv[3], "w").write(s.replace(sys.argv[2], sys.argv[4]))' \
    "$ROOT/scripts/openpyxl-ops.py" "$from" "$WORK/$name.py" "$to"
}
run_plant() {
  podman run --rm --network=none --userns=keep-id -v "$WORK/$1.py":/ops.py:ro "$IMAGE" \
    python3 -I -B /ops.py roundtrip /in/x.xlsx /out/x.xlsx </dev/null >"$WORK/$1.stdout" 2>"$WORK/$1.stderr"
  echo "$?" >"$WORK/$1.rc"
}
cap_lines_ok() {
  [ "$(cat "$WORK/$1.rc")" = 137 ] && [ "$(wc -l <"$WORK/$1.stdout")" = 2 ] &&
    [ "$(head -n 1 "$WORK/$1.stdout")" = MemoryError ] && last_ok "$(tail -n 1 "$WORK/$1.stdout")" &&
    [ ! -s "$WORK/$1.stderr" ]
}
plant M1 "        run(*argv)" "        raise MemoryError"
m1_rc=$?
plant M2 "        run(*argv)" "        raise ValueError"
m2_rc=$?
if [ "$m2_rc" = 0 ]; then
  mv "$WORK/M2.py" "$WORK/M2a.py"
  python3 -I -B -c 'import sys
s = open(sys.argv[1]).read()
old = "        print(type(err).__name__)"
if s.count(old) != 1:
    sys.exit(1)
open(sys.argv[2], "w").write(s.replace(old, "        raise MemoryError"))' "$WORK/M2a.py" "$WORK/M2.py"
  m2_rc=$?
fi
check "$(yn [ "$m1_rc$m2_rc" = 00 ])" "the two planted copies are made, each replacement found once"
run_plant M1
run_plant M2
check "$(yn cap_lines_ok M1)" "control M1: a MemoryError from the op exits 137 with MemoryError and the last line, nothing on standard error (ruling Y)"
check "$(yn cap_lines_ok M2)" "M2: a MemoryError raised while another exception is reported exits 137 with the same two lines, nothing on standard error (ruling Y)"

if [ "$failed" -eq 0 ]; then
  echo "openpyxl-ops-test: PASS"
  exit 0
fi
echo "openpyxl-ops-test: FAIL"
exit 1
