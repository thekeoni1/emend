#!/usr/bin/env bash
# emend edit open check (specs/0015-open-checks.md, "The script and the
# record"). Opens every pair a test run kept (EMEND_KEEP_DIR, "The keep") in
# the pinned LibreOffice image with the open check of
# specs/0006-passthrough.md P3, sourced from scripts/lo-check.sh, and writes
# the record pair <stem>.md and <stem>.txt into --out.
#
# Each side's reason is classed from the reason file's fixed opening words
# alone, never its text beyond them (P1 verification ruling 3 (b), P2 ruling
# 1): none (no reason file, ok), refused (LibreOffice loaded nothing: the
# conversion exited non-zero, a disallowed stderr line, or no ODS), count (the
# sheet and table counts differ) and error (the check did not run). An opening
# the map does not know, or a class that does not fit the side's verdict,
# stops the script with exit 2. A count-class input whose own xl/workbook.xml
# holds no sheet element (scripts/lo-check.sh's LO_SHEET_PATTERN) is listed
# as one the sheet count did not apply to; any other count-class input is a
# finding (P2 ruling 2).
#
# The keep directory must hold nothing but pairs: for every <test>.in.xlsx a
# <test>.out.xlsx and the other way round, <test> made of letters, digits,
# '_', '.' and '-'. Each pair's input and output are checked, the input as its
# own reference and the output as its own (specs/0016, "Planning rulings during
# P3", ruling 9), as the runner checks a corpus copy. The check's reasons go to a scratch directory made
# for the run and kept; they are never printed. The script prints one line per
# pair (test, lo_in, lo), a totals line and the reasons directory's path.
#
# Exit 0 when there is no finding and no check errored; 1 when an output was
# refused on an input LibreOffice accepted, or an input's sheet and table
# counts differ while its workbook holds sheet elements (a finding); 2 when
# any check errored (the check did not run), or on a usage or setup error, or
# on a reason the map does not know, which print one "error: " line and write
# no record.
set -uo pipefail
umask 077

readonly USAGE="usage: scripts/edit-open-check.sh --keep <dir> --libreoffice <image> [--lo-timeout <secs>] --out <records dir> --stem <stem>"
readonly TXT_HEADER=$'test\tin_bytes\tin_sha256\tout_bytes\tout_sha256\tlo_in\tlo\treason_in\treason_out'

die() {
  echo "error: $1" >&2
  exit 2
}

. "$(dirname -- "${BASH_SOURCE[0]}")/lo-check.sh" || die "cannot source scripts/lo-check.sh"

KEEP="" LO_IMAGE="" LO_TIMEOUT=600 OUT="" STEM=""
while [ $# -gt 0 ]; do
  case $1 in
    --keep | --libreoffice | --lo-timeout | --out | --stem)
      [ $# -ge 2 ] || die "$1 needs a value; $USAGE"
      case $1 in
        --keep) KEEP=$2 ;;
        --libreoffice) LO_IMAGE=$2 ;;
        --lo-timeout) LO_TIMEOUT=$2 ;;
        --out) OUT=$2 ;;
        --stem) STEM=$2 ;;
      esac
      shift 2
      ;;
    *) die "unknown argument; $USAGE" ;;
  esac
done
[ -n "$KEEP" ] && [ -n "$LO_IMAGE" ] && [ -n "$OUT" ] && [ -n "$STEM" ] || die "$USAGE"
[ -d "$KEEP" ] || die "--keep is not a directory"
[ -d "$OUT" ] || die "--out is not a directory"
[[ "$LO_TIMEOUT" =~ ^[1-9][0-9]*$ ]] || die "--lo-timeout must be a positive whole number of seconds"
[[ "$LO_IMAGE" =~ ^[A-Za-z0-9][A-Za-z0-9._:/@-]*$ ]] || die "--libreoffice must name an image"
[[ "$STEM" =~ ^[a-z0-9][a-z0-9.-]*$ ]] || die "--stem must be lower-case letters, digits, '.' and '-'"
[ ! -e "$OUT/$STEM.md" ] && [ ! -e "$OUT/$STEM.txt" ] || die "the record $STEM is already there"
for tool in podman timeout python3 sha256sum; do
  command -v "$tool" >/dev/null 2>&1 || die "$tool is not on PATH"
done

# The pairs, sorted by name, and a refusal of anything that is not one.
tests=()
declare -A seen=()
while IFS= read -r -d '' path; do
  name=${path##*/}
  [ -f "$path" ] || die "the keep directory holds something that is not a file"
  [[ "$name" =~ ^([A-Za-z0-9_.-]+)\.(in|out)\.xlsx$ ]] ||
    die "the keep directory holds a file that is not half of a pair"
  test=${BASH_REMATCH[1]}
  seen[$test]=$((${seen[$test]:-0} + 1))
done < <(find "$KEEP" -mindepth 1 -maxdepth 1 -print0)
[ "${#seen[@]}" -gt 0 ] || die "the keep directory holds no pairs"
for test in "${!seen[@]}"; do
  [ -f "$KEEP/$test.in.xlsx" ] && [ -f "$KEEP/$test.out.xlsx" ] ||
    die "the keep directory holds half a pair"
done
mapfile -t tests < <(printf '%s\n' "${!seen[@]}" | LC_ALL=C sort)

WORK=$(mktemp -d "${TMPDIR:-/tmp}/emend-open-check.XXXXXX") || die "cannot make the reasons directory"

verdict() {
  case $1 in
    0) echo ok ;;
    1) echo fail ;;
    *) echo error ;;
  esac
}

# reason_class <reason file> <verdict>: the class word of one side, read from
# the reason file's fixed opening words (P2 ruling 1), checked against the
# side's verdict.
reason_class() {
  local file=$1 verdict=$2 first class
  if [ ! -e "$file" ]; then
    class=none
  else
    IFS= read -r first <"$file" || [ -n "$first" ] || die "cannot read a reason file"
    case $first in
      "the conversion exited "* | \
        "stderr holds a line with Error or Warning other than the allowed javaldx line" | \
        "no ods file was written") class=refused ;;
      "the ods holds "*" table elements and the workbook "*" sheet elements") class=count ;;
      "cannot prepare the conversion directory" | "the conversion timed out after "* | \
        "podman could not run the container, exit "* | "the conversion was killed, exit status "* | \
        "the stderr scan failed, grep exit "* | "the stderr allowance scan failed, grep exit "* | \
        "cannot count sheets and tables" | "cannot read the ods or the workbook part") class=error ;;
      *) die "an unknown reason" ;;
    esac
  fi
  case $verdict:$class in
    ok:none | fail:refused | fail:count | error:error) echo "$class" ;;
    *) die "a reason that does not fit its verdict" ;;
  esac
}

# sheet_count <workbook package>: the sheet elements in its xl/workbook.xml,
# by the library's pattern.
sheet_count() {
  LO_SHEET=$LO_SHEET_PATTERN python3 -c '
import os, re, sys, zipfile
workbook = zipfile.ZipFile(sys.argv[1]).read("xl/workbook.xml").decode("utf-8", "replace")
print(len(re.findall(os.environ["LO_SHEET"], workbook)))
' "$1" 2>/dev/null
}

rows=$WORK/rows.txt
: >"$rows" || die "cannot write the rows"
n=0
for test in "${tests[@]}"; do
  n=$((n + 1))
  input=$KEEP/$test.in.xlsx
  output=$KEEP/$test.out.xlsx
  lo_check "$input" "$WORK/$n/in" "$WORK/$n/in-reason.txt" "$input"
  lo_in=$(verdict $?)
  # The output is counted against its own sheets, as the input is against its
  # own: an edit may change the sheet list (specs/0016, "Planning rulings
  # during P3", ruling 9).
  lo_check "$output" "$WORK/$n/out" "$WORK/$n/out-reason.txt" "$output"
  lo=$(verdict $?)
  reason_in=$(reason_class "$WORK/$n/in-reason.txt" "$lo_in") || exit 2
  reason_out=$(reason_class "$WORK/$n/out-reason.txt" "$lo") || exit 2
  in_sheets=-
  if [ "$reason_in" = count ]; then
    in_sheets=$(sheet_count "$input") && [[ "$in_sheets" =~ ^[0-9]+$ ]] || die "cannot count a kept input's sheets"
  fi
  in_bytes=$(stat -c %s -- "$input") || die "cannot read a kept file's size"
  out_bytes=$(stat -c %s -- "$output") || die "cannot read a kept file's size"
  in_sha=$(sha256sum -- "$input") || die "cannot hash a kept file"
  out_sha=$(sha256sum -- "$output") || die "cannot hash a kept file"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$test" "$in_bytes" "${in_sha%% *}" \
    "$out_bytes" "${out_sha%% *}" "$lo_in" "$lo" "$reason_in" "$reason_out" "$in_sheets" >>"$rows" ||
    die "cannot write the rows"
  printf '%s\t%s\t%s\n' "$test" "$lo_in" "$lo"
done

# The record pair, from the rows alone.
summary=$(RECORD_ROWS=$rows RECORD_OUT=$OUT RECORD_STEM=$STEM RECORD_IMAGE=$LO_IMAGE \
  RECORD_TIMEOUT=$LO_TIMEOUT RECORD_HEADER=$TXT_HEADER python3 -c '
import datetime, os
env = os.environ
# Each scratch row carries the sheet count of the input after the nine record
# columns ("-" unless its class is count); the record takes the nine.
scratch = [line.rstrip("\n").split("\t") for line in open(env["RECORD_ROWS"])]
rows = [row[:9] for row in scratch]
in_sheets = {row[0]: row[9] for row in scratch}
stem, out = env["RECORD_STEM"], env["RECORD_OUT"]

def count(column, value):
    return sum(1 for row in rows if row[column] == value)

CLASSES = ("none", "refused", "count", "error")
refused_by_lo = [row[0] for row in rows if row[7] == "refused"]
count_na = [row[0] for row in rows if row[7] == "count" and in_sheets[row[0]] == "0"]
count_differ = [row[0] for row in rows if row[7] == "count" and in_sheets[row[0]] != "0"]
refused_outputs = [row[0] for row in rows if row[6] == "fail" and row[5] == "ok"]
errored = [row[0] for row in rows if "error" in (row[5], row[6])]
totals = [len(rows)] + [count(5, v) for v in ("ok", "fail", "error")] + [count(6, v) for v in ("ok", "fail", "error")]
classes = {side: [count(col, c) for c in CLASSES] for side, col in (("reason_in", 7), ("reason_out", 8))}

with open(os.path.join(out, stem + ".txt"), "x") as txt:
    txt.write(env["RECORD_HEADER"] + "\n")
    for row in rows:
        txt.write("\t".join(row) + "\n")

def listed(names):
    return "\n".join("- `%s`" % name for name in names) if names else "None."

written = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
md = []
md.append("# edit open check %s" % stem)
md.append("")
md.append(
    "Written by `scripts/edit-open-check.sh` from a keep directory of test"
    " fixtures (`specs/0015-open-checks.md`, \"The keep\"). Each kept pair is"
    " cited by test name, byte counts and sha256 only; the files are fictional"
    " fixtures and are not committed. The machine-readable companion to this"
    " file is `%s.txt`, the same rows as a tab-separated table; it is written as"
    " `.txt` because the hygiene gate refuses a tracked spreadsheet"
    " extension." % stem)
md.append("")
md.append("- Record written: %s" % written)
md.append("- Image: %s" % env["RECORD_IMAGE"])
md.append("- Open-check timeout: %s s" % env["RECORD_TIMEOUT"])
md.append("- Pairs: %d" % len(rows))
md.append("")
md.append("## Totals")
md.append("")
md.append("| pairs | lo_in ok | lo_in fail | lo_in error | lo ok | lo fail | lo error |")
md.append("| --- | --- | --- | --- | --- | --- | --- |")
md.append("| " + " | ".join(str(t) for t in totals) + " |")
md.append("")
md.append("| side | none | refused | count | error |")
md.append("| --- | --- | --- | --- | --- |")
for side in ("reason_in", "reason_out"):
    md.append("| %s | " % side + " | ".join(str(v) for v in classes[side]) + " |")
md.append("")
md.append("## Inputs refused by LibreOffice (allowed: a fixture LibreOffice cannot load: zip-level, or a package with no relationships part or an empty content-types root)")
md.append("")
md.append(listed(refused_by_lo))
md.append("")
md.append("## Inputs the sheet count did not apply to (allowed: a workbook fixture with no sheet element)")
md.append("")
md.append(listed(count_na))
md.append("")
md.append("## Inputs whose sheet and table counts differ (a finding)")
md.append("")
md.append(listed(count_differ))
md.append("")
md.append("## Outputs refused on an input LibreOffice accepted (a finding)")
md.append("")
md.append(listed(refused_outputs))
md.append("")
md.append("## Checks that did not run (error)")
md.append("")
md.append(listed(errored))
md.append("")
md.append("## Pairs")
md.append("")
md.append("| test | in bytes | in sha256 | out bytes | out sha256 | lo_in | lo | reason_in | reason_out |")
md.append("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
for row in rows:
    md.append("| `%s` | %s | %s | %s | %s | %s | %s | %s | %s |" % tuple(row))
with open(os.path.join(out, stem + ".md"), "x") as f:
    f.write("\n".join(md) + "\n")
print("%d %d %d %d %d" % (len(refused_outputs) + len(count_differ), len(errored), len(refused_by_lo), len(count_na), len(count_differ)))
print("pairs %d; lo_in ok %d fail %d error %d; lo ok %d fail %d error %d" % tuple(totals))
') || die "cannot write the record"

{ read -r n_findings n_errored n_inputs n_na n_differ; read -r totals; } <<<"$summary"
echo "wrote $STEM.md and $STEM.txt: $totals; inputs refused by LibreOffice $n_inputs; the sheet count did not apply $n_na; counts differ $n_differ; findings $n_findings"
echo "reasons: $WORK"
if [ "$n_errored" -gt 0 ]; then
  exit 2
fi
if [ "$n_findings" -gt 0 ]; then
  exit 1
fi
exit 0
