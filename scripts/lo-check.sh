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
# counts a kept input's sheets with this same pattern; the open check itself
# counts with a parser since specs/0047-harness-findings.md decision 1 A
# (lo_count_check below).
readonly LO_SHEET_PATTERN='<(?:[A-Za-z_][A-Za-z0-9._-]*:)?sheet(?=[\s/>])'

# lo_check <file> <work dir> <reason file> <reference workbook>: the open check
# of specs/0006-passthrough.md P3. The file's directory is mounted read-only as
# /in and the work directory's ods/ as /out, and the copy is converted to ODS
# under --lo-timeout, which is its own budget, one per conversion. It is ok
# when the conversion exits 0, its stderr holds no line with Error or Warning
# other than the allowed javaldx line, the ODS exists, and lo_count_check
# finds the ODS's tables agree with the reference workbook's sheets. It prints
# nothing: the reason goes to the reason file, which stays in the run
# directory.
# The container is named emend-lo-<shell pid>-<subshell pid>-<n>, n counting
# this shell's calls. A timeout kills the podman client and not the
# container, so after exit 124 the container is removed by name with podman
# rm -f --time 0 (no grace period), its output and exit kept in the work
# directory as rm.txt and rm-exit.txt (specs/0037-measurement-and-close.md,
# "Planning rulings during P1", ruling A, carried to P3 as engineering-close
# item 7: the runner's form).
# Returns 0 for ok, 1 for fail (LibreOffice refused the file, or the ODS is
# missing or wrong) and 2 for error, which is the check not running at all:
# the conversion directory, a conversion that timed out under --lo-timeout, a
# conversion killed by a signal, podman itself, python3, or an ODS or workbook
# part that cannot be read. Planning ruled the two apart on 2026-09-15 and
# fixed their order on 2026-09-16, so a broken tool is never read as a broken
# archive.
LO_CHECK_N=0
lo_check() {
  local file=$1 work=$2 reason=$3 reference=$4
  local indir base outdir ods rc name
  LO_CHECK_N=$((LO_CHECK_N + 1))
  name=emend-lo-$$-$BASHPID-$LO_CHECK_N
  indir=$(dirname -- "$file")
  base=$(basename -- "$file")
  outdir=$work/ods
  mkdir -p "$work" && rm -rf "$outdir" && mkdir -p "$outdir" ||
    { printf 'cannot prepare the conversion directory\n' >"$reason"; return 2; }
  timeout --signal=KILL "$LO_TIMEOUT" podman run --rm --name "$name" --network=none --userns=keep-id \
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
    # The kill ended the podman client; the container is removed by name.
    podman rm -f --time 0 "$name" </dev/null >"$work/rm.txt" 2>&1 3>&-
    echo "$?" >"$work/rm-exit.txt"
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
  lo_count_check "$reference" "$ods" "$reason"
}

# lo_count_check <workbook package> <ods> <reason file>: the count rule of the
# open check (specs/0047-harness-findings.md, "The fixes (P2)", decision 1 A),
# callable alone so that it is tested without a conversion
# (scripts/lo-check-test.sh). Both archives are read with zipfile and
# ElementTree, never with a pattern over the raw text. From the workbook it
# reads the sheet elements of xl/workbook.xml and the sheetName elements over
# the parts xl/_rels/workbook.xml.rels types externalLink; from the ODS the
# table:table elements of content.xml. A table is an external-link cache, the
# copy LibreOffice keeps of a linked workbook's sheet, when its table:name
# holds :// or starts with file:, also holds #, and its table:print is false;
# every other table counts as a sheet. It is ok when the workbook holds a
# sheet element, the tables that are not caches equal the sheet elements, and
# the caches do not exceed the external sheet names: a cache with no name
# fails, a name with no cache does not, because LibreOffice writes a cache
# table only for an external sheet a formula references (ruling F, after
# 9956034c4326's edited output, whose one formula was the one that used its
# link, made none). Elements count by local name in any namespace, as
# LO_SHEET_PATTERN takes any prefix.
# Returns 0 for ok, 1 for fail and 2 for error (an archive or part that cannot
# be read, or python3 failing), as lo_check does, and prints nothing.
lo_count_check() {
  local reference=$1 ods=$2 reason=$3 counts sheets names tables caches
  counts=$(python3 -I -B -c '
import posixpath, sys, zipfile
import xml.etree.ElementTree as ET
TABLE = "urn:oasis:names:tc:opendocument:xmlns:table:1.0"
RELS = "http://schemas.openxmlformats.org/package/2006/relationships"
def local(tag):
    return tag.rpartition("}")[2]
def count(data, name):
    return sum(1 for e in ET.fromstring(data).iter() if local(e.tag) == name)
try:
    with zipfile.ZipFile(sys.argv[1]) as z:
        sheets = count(z.read("xl/workbook.xml"), "sheet")
        names = 0
        if "xl/_rels/workbook.xml.rels" in z.namelist():
            for rel in ET.fromstring(z.read("xl/_rels/workbook.xml.rels")).iter("{%s}Relationship" % RELS):
                if not rel.get("Type", "").endswith("/externalLink") or rel.get("TargetMode") == "External":
                    continue
                target = rel.get("Target", "")
                if target.startswith("/"):
                    part = posixpath.normpath(target[1:])
                else:
                    part = posixpath.normpath(posixpath.join("xl", target))
                names += count(z.read(part), "sheetName")
    tables = caches = 0
    with zipfile.ZipFile(sys.argv[2]) as z, z.open("content.xml") as f:
        # Only the start of each table is read; rows are let go as they end.
        for event, e in ET.iterparse(f, events=("start", "end")):
            if event == "start" and e.tag == "{%s}table" % TABLE:
                tables += 1
                name = e.get("{%s}name" % TABLE, "")
                if (("://" in name or name.startswith("file:")) and "#" in name
                        and e.get("{%s}print" % TABLE) == "false"):
                    caches += 1
            elif event == "end" and e.tag == "{%s}table-row" % TABLE:
                e.clear()
except Exception as err:
    sys.stdout.write("error %s" % type(err).__name__)
else:
    sys.stdout.write("%d %d %d %d" % (sheets, names, tables, caches))
' "$reference" "$ods") || { printf 'cannot count sheets and tables\n' >"$reason"; return 2; }
  read -r sheets names tables caches <<<"$counts"
  if ! [[ "$sheets" =~ ^[0-9]+$ ]] || ! [[ "${names:-}" =~ ^[0-9]+$ ]] ||
    ! [[ "${tables:-}" =~ ^[0-9]+$ ]] || ! [[ "${caches:-}" =~ ^[0-9]+$ ]]; then
    printf 'cannot read the ods or the workbook part\n' >"$reason"
    return 2
  fi
  if [ "$sheets" -eq 0 ] || [ "$((tables - caches))" -ne "$sheets" ] || [ "$caches" -gt "$names" ]; then
    printf 'the ods holds %s table elements, %s of them external-link caches, and the workbook %s sheet elements and %s external sheet names\n' \
      "$tables" "$caches" "$sheets" "$names" >"$reason"
    return 1
  fi
  return 0
}
