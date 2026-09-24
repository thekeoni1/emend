# emend open-check library (specs/0015-open-checks.md, "The open-check
# library"): the LibreOffice open check of specs/0006-passthrough.md P3 and
# its constant, moved here unchanged from scripts/passthrough-run.sh so that
# the runner and scripts/edit-open-check.sh run the one definition.
#
# Sourced, never run. The caller sets LO_IMAGE (the pinned image) and
# LO_TIMEOUT (whole seconds per conversion) before calling lo_check, and has
# podman, timeout and python3 on PATH. Grep is named by path here, as the
# runner names it, so the library does not depend on a caller's variable.
readonly LO_GREP=/usr/bin/grep
# The image has no Java by design, so every conversion writes this one line to
# stderr. Planning ruled on 2026-09-15 that it is the only allowed line holding
# Error or Warning, matched whole (records/libreoffice-2026-09-15.md).
readonly LO_ALLOWED_STDERR='Warning: failed to launch javaldx - java may not function correctly'
# The sheet elements of xl/workbook.xml, as a Python regular expression: a
# sheet element with any namespace prefix or none, followed by whitespace, a
# slash or the tag's end. <sheets> and <sheetData do not match. P1
# verification ruling 3 (a) amended T3 P3's definition, which matched only an
# unprefixed <sheet, so a prefixed workbook counted 0. A sheet element of a
# foreign namespace would count too; no corpus file or fixture has one, and
# the check is a count, not a schema (P2 ruling 4). scripts/edit-open-check.sh
# counts a kept input's sheets with this same pattern.
readonly LO_SHEET_PATTERN='<(?:[A-Za-z_][A-Za-z0-9._-]*:)?sheet(?=[\s/>])'

# lo_check <file> <work dir> <reason file> <reference workbook>: the open check
# of specs/0006-passthrough.md P3. The file's directory is mounted read-only as
# /in and the work directory's ods/ as /out, and the copy is converted to ODS
# under --lo-timeout, which is its own budget, one per conversion. It is ok
# when the conversion exits 0, its stderr holds no line with Error or Warning
# other than the allowed javaldx line, the ODS exists, and its content.xml
# holds as many <table:table elements as the reference workbook's
# xl/workbook.xml holds <sheet elements. It prints nothing: the reason goes to
# the reason file, which stays in the run directory.
# Returns 0 for ok, 1 for fail (LibreOffice refused the file, or the ODS is
# missing or wrong) and 2 for error, which is the check not running at all:
# the conversion directory, a conversion that timed out under --lo-timeout, a
# conversion killed by a signal, podman itself, python3, or an ODS or workbook
# part that cannot be read. Planning ruled the two apart on 2026-09-15 and
# fixed their order on 2026-09-16, so a broken tool is never read as a broken
# archive.
lo_check() {
  local file=$1 work=$2 reason=$3 reference=$4
  local indir base outdir ods rc counts sheets tables
  indir=$(dirname -- "$file")
  base=$(basename -- "$file")
  outdir=$work/ods
  mkdir -p "$work" && rm -rf "$outdir" && mkdir -p "$outdir" ||
    { printf 'cannot prepare the conversion directory\n' >"$reason"; return 2; }
  timeout --signal=KILL "$LO_TIMEOUT" podman run --rm --network=none --userns=keep-id \
    -v "$indir":/in:ro -v "$outdir":/out "$LO_IMAGE" \
    --convert-to ods --outdir /out "/in/$base" \
    </dev/null >"$work/stdout.txt" 2>"$work/stderr.txt" 3>&-
  rc=$?
  # Planning's ruling 1 of 2026-09-16: the conversion process itself is judged
  # first, and a status of 0 decides nothing on its own, so it falls through to
  # the checks below. 124 is this timeout's own kill; 125 to 127 are podman
  # failing to start the container; 128 and above mean the contained process
  # died by a signal (137 is KILL, which is also what the kernel's OOM killer
  # produces). None of those three is evidence about the archive, so each is an
  # error. A status of 1 to 123 is LibreOffice refusing the file: a fail, with
  # the status in the reason.
  if [ "$rc" -eq 124 ]; then
    printf 'the conversion timed out after %s s\n' "$LO_TIMEOUT" >"$reason"
    return 2
  fi
  if [ "$rc" -ge 125 ] && [ "$rc" -le 127 ]; then
    printf 'podman could not run the container, exit %s\n' "$rc" >"$reason"
    return 2
  fi
  if [ "$rc" -ge 128 ]; then
    printf 'the conversion was killed, exit status %s\n' "$rc" >"$reason"
    return 2
  fi
  if [ "$rc" -ne 0 ]; then
    printf 'the conversion exited %s\n' "$rc" >"$reason"
    return 1
  fi
  # The two greps are evaluated with their exit codes read, because an error
  # from either (exit 2 or more) must never read as "no disallowed line"
  # (specs/0001-bootstrap.md, "Rules for every phase": an error is never
  # reported as zero hits). Independent check 1 found the piped form doing
  # exactly that.
  LC_ALL=C "$LO_GREP" -a -E 'Error|Warning' "$work/stderr.txt" >"$work/flagged.txt"
  rc=$?
  if [ "$rc" -ge 2 ]; then
    printf 'the stderr scan failed, grep exit %s\n' "$rc" >"$reason"
    return 2
  fi
  LC_ALL=C "$LO_GREP" -a -q -v -x -F -e "$LO_ALLOWED_STDERR" "$work/flagged.txt"
  rc=$?
  if [ "$rc" -ge 2 ]; then
    printf 'the stderr allowance scan failed, grep exit %s\n' "$rc" >"$reason"
    return 2
  fi
  if [ "$rc" -eq 0 ]; then
    printf 'stderr holds a line with Error or Warning other than the allowed javaldx line\n' >"$reason"
    return 1
  fi
  ods=$outdir/${base%.*}.ods
  if [ ! -f "$ods" ]; then
    printf 'no ods file was written\n' >"$reason"
    return 1
  fi
  counts=$(LO_REFERENCE=$reference LO_ODS=$ods LO_SHEET=$LO_SHEET_PATTERN python3 -c '
import os, re, sys, zipfile
try:
    workbook = zipfile.ZipFile(os.environ["LO_REFERENCE"]).read("xl/workbook.xml").decode("utf-8", "replace")
    content = zipfile.ZipFile(os.environ["LO_ODS"]).read("content.xml").decode("utf-8", "replace")
except Exception as err:
    sys.stdout.write("error %s" % type(err).__name__)
else:
    # Tolerant of any whitespace after the name, and of a tag closed straight
    # away: <sheet\n, <sheet\t, <sheet/> and <sheet> all count, with any prefix
    # (LO_SHEET_PATTERN), while <sheets> and <sheetData do not, and
    # <table:table-row and <table:table-cell do not.
    sheet = os.environ["LO_SHEET"]
    table = r"<table:table(?=[\s/>])"
    sys.stdout.write("%d %d" % (len(re.findall(sheet, workbook)), len(re.findall(table, content))))
') || { printf 'cannot count sheets and tables\n' >"$reason"; return 2; }
  read -r sheets tables <<<"$counts"
  if ! [[ "$sheets" =~ ^[0-9]+$ ]] || ! [[ "${tables:-}" =~ ^[0-9]+$ ]]; then
    printf 'cannot read the ods or the workbook part\n' >"$reason"
    return 2
  fi
  if [ "$sheets" -eq 0 ] || [ "$sheets" -ne "$tables" ]; then
    printf 'the ods holds %s table elements and the workbook %s sheet elements\n' "$tables" "$sheets" >"$reason"
    return 1
  fi
  return 0
}
