#!/usr/bin/env bash
# emend openpyxl comparison runner (specs/0037-measurement-and-close.md, "The
# comparison runner", and "Planning rulings during P1", A to E), modelled on
# scripts/passthrough-run.sh and sharing scripts/lo-check.sh.
# For every manifest row, or every id listed with --ids, in manifest order:
# run scripts/openpyxl-ops.py <op> in the openpyxl image on that one workbook,
# mounted read-only, under a timeout and an address space limit, classify the
# outcome, compare input and output with xlsx-diff on the host, and append to
# summary.openpyxl.tsv and entries.openpyxl.tsv in the run directory, in the
# runner's columns. Each invocation also appends one row to run.openpyxl.tsv
# (commit, times, the image id, the openpyxl version, the settings, the op).
# It prints one line per file (id, outcome, exit, wall seconds, peak RSS in
# kB) and a final count line. It never prints an original name, a path under
# the corpus directory or a byte of content: openpyxl's own output, warnings
# included, stays in the run directory. Errors print one "error: " line
# without a path and exit 2: once the arguments are checked, the script's own
# stderr goes to /dev/null, and the error line goes to the saved descriptor.
#
# The container: named emend-openpyxl-<this script's pid>-<id>, run with
# --network=none --userns=keep-id, the input at /in/<id>.<ext> read-only, the
# run subdirectory's out.openpyxl/ at /out and the ops script at /ops.py
# read-only, the command sh -c 'ulimit -v <MiB*1024> && exec python3 -I -B
# /ops.py <op> /in/<id>.<ext> /out/<id>.<ext>' under timeout --signal=KILL.
# The output keeps the input's extension (ruling D). A timeout kills the
# podman client and not the container, so after exit 124 the runner removes
# the container by name with podman rm -f --time 0 (no grace period), its
# output and exit kept in the run subdirectory; at the end it counts the
# image's containers and fails if any remain (ruling A). The count runs in
# an EXIT trap, so it is printed whatever ends the script once the work
# directory exists, a die inside the loop included (specs/0037 "Planning
# verification of P1", ruling I).
#
# Outcomes (ruling B): ok, exit 0 with an output file; no-output, exit 0
# without one; timeout, exit 124; cap, exit 137 (the ops script's MemoryError);
# error, exit 1 and every other exit, the exit in the exit column. Wall time is
# the runner's, around the container; peak RSS is the ops script's last line.
# lo_in is "-": the input's verdict is the emend run's (ruling C).
set -uo pipefail
umask 077

readonly USAGE="usage: scripts/compare-run.sh --op <op> --image <image> --xlsx-diff <path> [--corpus <dir>] [--out <dir>] [--timeout <secs>] [--memory <MiB>] [--ids <file>] [--libreoffice <image>] [--lo-timeout <secs>]"
readonly LABEL=openpyxl
readonly OPS_LIST=" roundtrip read edit format insert-rows names table comment image add-chart copy chartsheet chart-format chart-analytic chart-text named-style "
readonly MANIFEST_HEADER=$'id\tsha256\tbytes\text\tproducer\tproducer_version\tapp\tsource\ttags\tadded\toriginal_name\tnote'
readonly SUMMARY_HEADER=$'id\tlabel\tproducer\ttags\toutcome\texit\tsignal\twall_s\tpeak_rss_kb\tin_bytes\tout_bytes\tdiff_exit\tidentical\trecompressed\tchanged\tdropped\tadded\tflagged\torder_preserved\tcomment_equal\tfiles_identical\tleftover\tlo\tlo_in'
readonly ENTRIES_HEADER=$'id\tlabel\trow\tname\tstatus\tindex_in\tindex_out\tmethod_in\tmethod_out\tuncompressed_in\tuncompressed_out\tcompressed_in\tcompressed_out\tcrc_in\tcrc_out\tfirst_difference\tdiffering_bytes\tlast_difference\tflags'
readonly RUN_HEADER=$'commit\tstarted\tended\timage_id\topenpyxl\ttimeout_s\tlo_timeout_s\tmemory_mib\tops_sha256\txlsx_diff_sha256\tfiles\top'
DIE_FD=2
die() {
  echo "error: $1" >&"$DIE_FD"
  exit 2
}

. "$(dirname -- "${BASH_SOURCE[0]}")/lo-check.sh" || die "cannot source scripts/lo-check.sh"

OP="" IMAGE="" XD="" CORPUS="" OUT="" TIMEOUT=300 MEMORY=2048 IDS="" LO_IMAGE="" LO_TIMEOUT=600
while [ "$#" -gt 0 ]; do
  case "$1" in
    --op | --image | --xlsx-diff | --corpus | --out | --timeout | --memory | --ids | --libreoffice | --lo-timeout)
      [ "$#" -ge 2 ] || die "$1 needs a value; $USAGE"
      case "$1" in
        --op) OP=$2 ;;
        --image) IMAGE=$2 ;;
        --xlsx-diff) XD=$2 ;;
        --corpus) CORPUS=$2 ;;
        --out) OUT=$2 ;;
        --timeout) TIMEOUT=$2 ;;
        --memory) MEMORY=$2 ;;
        --ids) IDS=$2 ;;
        --libreoffice) LO_IMAGE=$2 ;;
        --lo-timeout) LO_TIMEOUT=$2 ;;
      esac
      shift 2
      ;;
    *) die "unknown argument; $USAGE" ;;
  esac
done

[ -n "$OP" ] && [[ "$OPS_LIST" == *" $OP "* ]] || die "--op must name one of the sixteen ops; $USAGE"
[[ "$IMAGE" =~ ^[A-Za-z0-9][A-Za-z0-9._:/@-]*$ ]] || die "--image must name an image"
[ -n "$XD" ] && [ -f "$XD" ] && [ -x "$XD" ] || die "--xlsx-diff must name an executable file"
[[ "$TIMEOUT" =~ ^[1-9][0-9]*$ ]] || die "--timeout must be a positive whole number of seconds"
[[ "$MEMORY" =~ ^[1-9][0-9]*$ ]] || die "--memory must be a positive whole number of MiB"
[[ "$LO_TIMEOUT" =~ ^[1-9][0-9]*$ ]] || die "--lo-timeout must be a positive whole number of seconds"
command -v podman >/dev/null 2>&1 || die "the runner needs podman"
podman image exists "$IMAGE" >/dev/null 2>&1 || die "--image is not an image podman holds"
if [ -n "$LO_IMAGE" ]; then
  [[ "$LO_IMAGE" =~ ^[A-Za-z0-9][A-Za-z0-9._:/@-]*$ ]] || die "--libreoffice must name an image"
  command -v python3 >/dev/null 2>&1 || die "--libreoffice needs python3 to count sheets and tables"
fi

exec 3>&2 2>/dev/null
DIE_FD=3

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd) || die "cannot find the repository"
readonly OPS_SCRIPT=$ROOT/scripts/openpyxl-ops.py
[ -f "$OPS_SCRIPT" ] || die "cannot find scripts/openpyxl-ops.py"
if [ -z "$CORPUS" ]; then
  CORPUS=${EMEND_CORPUS_DIR:-$HOME/emend-testing}
fi
[ -d "$CORPUS" ] || die "the corpus directory does not exist"
readonly MANIFEST=$CORPUS/manifest.tsv
[ -f "$MANIFEST" ] || die "the corpus has no manifest.tsv"
[ "$(head -n 1 "$MANIFEST")" = "$MANIFEST_HEADER" ] || die "the manifest header is not the one specs/0005-corpus.md defines"

COMMIT=$(git -C "$ROOT" rev-parse HEAD 2>/dev/null) || die "cannot read the repository commit"
SHORT=$(git -C "$ROOT" rev-parse --short HEAD)
if [ -z "$OUT" ]; then
  OUT=$CORPUS/runs/$(date -u +%Y-%m-%d)-$SHORT-openpyxl-$OP
fi
mkdir -p "$OUT" || die "cannot create the run directory"

WORK=$(mktemp -d) || die "cannot create a temporary directory"

# finish: on every exit from here on, count the image's containers, print the
# count line, fail if any remain or the count cannot be made, and remove the
# work directory (ruling I). A non-zero status already on its way out is kept.
finish() {
  local rc=$? remaining=""
  if podman ps -a --filter "ancestor=$IMAGE" --format '{{.Names}}' </dev/null >"$WORK/containers" 3>&-; then
    remaining=$(awk 'NF { n++ } END { print n + 0 }' "$WORK/containers")
  fi
  if [ -z "$remaining" ]; then
    echo "error: cannot count the image's containers" >&"$DIE_FD"
    rc=2
  else
    echo "containers of the image remaining: $remaining"
    if [ "$remaining" != 0 ]; then
      echo "error: containers of the image remain after the run: $remaining" >&"$DIE_FD"
      rc=2
    fi
  fi
  rm -rf "${WORK:?}"
  exit "$rc"
}
trap finish EXIT

# The manifest's id, ext, producer and tags columns, in manifest order. The
# private columns are never read into the script.
if ! awk -F'\t' -v OFS='\t' '
  NR == 1 { next }
  NF != 12 { print "bad", NR; exit 3 }
  $1 !~ /^[0-9a-f]{12}$/ || $4 !~ /^(xlsx|xlsm|xltx|xltm)$/ || $5 !~ /^[a-z-]+$/ || $9 !~ /^[a-z0-9,]+$/ { print "bad", NR; exit 3 }
  seen[$1]++ { print "duplicate", $1; exit 3 }
  { print $1, $4, $5, $9 }
' "$MANIFEST" >"$WORK/rows"; then
  die "manifest row $(awk '{ print $2 }' "$WORK/rows" | tail -n 1) is not valid (id, ext, producer, tags or column count)"
fi

if [ -n "$IDS" ]; then
  [ -f "$IDS" ] || die "--ids must name a file"
  : >"$WORK/wanted" || die "cannot write a scratch file"
  while IFS= read -r line || [ -n "$line" ]; do
    id=${line//[[:space:]]/}
    [ -z "$id" ] && continue
    [[ "$id" =~ ^[0-9a-f]{12}$ ]] || die "an entry in the --ids file is not a 12-digit id"
    awk -F'\t' -v id="$id" '$1 == id { found = 1 } END { exit !found }' "$WORK/rows" ||
      die "id $id is not in the manifest"
    printf '%s\n' "$id" >>"$WORK/wanted" || die "cannot write a scratch file"
  done <"$IDS" || die "cannot read the --ids file"
  awk -F'\t' 'NR == FNR { want[$1] = 1; next } want[$1]' "$WORK/wanted" "$WORK/rows" >"$WORK/selected" ||
    die "cannot write a scratch file"
else
  cp "$WORK/rows" "$WORK/selected" || die "cannot write a scratch file"
fi

# header_ok <file> <header> <name>: the file is absent (the header is written)
# or starts with that header; anything else is an error.
header_ok() {
  if [ -e "$1" ]; then
    [ "$(head -n 1 "$1")" = "$2" ] || die "$3 in the run directory has another header"
  else
    printf '%s\n' "$2" >"$1" || die "cannot write the header of $3"
  fi
}
SUMMARY=$OUT/summary.$LABEL.tsv
ENTRIES=$OUT/entries.$LABEL.tsv
RUNS=$OUT/run.$LABEL.tsv
header_ok "$SUMMARY" "$SUMMARY_HEADER" "summary.$LABEL.tsv"
header_ok "$ENTRIES" "$ENTRIES_HEADER" "entries.$LABEL.tsv"
header_ok "$RUNS" "$RUN_HEADER" "run.$LABEL.tsv"

# An id already in the summary would give it a second row; a run directory
# holding an earlier run's conversion or output directories is refused, as the
# passthrough runner refuses its own (specs/0030 decision 1 A).
dup=$(awk -F'\t' 'FILENAME == ARGV[1] { if (FNR > 1) have[$1] = 1; next } ($1 in have) { print $1; exit }' \
  "$SUMMARY" "$WORK/selected") || die "cannot read summary.$LABEL.tsv"
[ -z "$dup" ] || die "id $dup already has a row in summary.$LABEL.tsv; use a new run directory"
earlier=$(find "$OUT" -mindepth 2 -maxdepth 2 \( -name "lo.$LABEL" -o -name "out.$LABEL" \) -print -quit) ||
  die "cannot read the run directory"
[ -z "$earlier" ] || die "the run directory holds an earlier run; name a fresh one"

STARTED=$(date -u +%Y-%m-%dT%H:%M:%SZ)
IMAGE_ID=$(podman image inspect --format '{{.Id}}' "$IMAGE" </dev/null) || die "cannot read the image id"
VERSION=$(podman run --rm --network=none --userns=keep-id "$IMAGE" \
  python3 -I -B -c 'import openpyxl; print(openpyxl.__version__)' </dev/null) || die "cannot read the openpyxl version"
[[ "$VERSION" =~ ^[0-9A-Za-z.+-]+$ ]] || die "the openpyxl version is not a version"
OPS_SHA=$(sha256sum <"$OPS_SCRIPT" | cut -d' ' -f1) || die "cannot hash the ops script"
XD_SHA=$(sha256sum <"$XD" | cut -d' ' -f1) || die "cannot hash the xlsx-diff binary"

# now_ns: the time in nanoseconds since the epoch.
now_ns() {
  date +%s%N
}

n_files=0 n_ok=0 n_error=0 n_timeout=0 n_cap=0 n_nooutput=0
while IFS=$'\t' read -r id ext producer tags; do
  n_files=$((n_files + 1))
  dir=$OUT/$id
  outdir=$dir/out.$LABEL
  mkdir -p "$dir" "$outdir" || die "cannot create a run subdirectory"
  input=$CORPUS/corpus/$id.$ext
  output=$outdir/$id.$ext
  name=emend-openpyxl-$$-$id
  in_bytes=$(stat -c %s -- "$input" 2>/dev/null) || in_bytes=""

  t0=$(now_ns)
  timeout --signal=KILL "$TIMEOUT" podman run --rm --name "$name" --network=none --userns=keep-id \
    -v "$input":"/in/$id.$ext":ro -v "$outdir":/out -v "$OPS_SCRIPT":/ops.py:ro "$IMAGE" \
    sh -c 'ulimit -v "$1" && exec python3 -I -B /ops.py "$2" "$3" "$4"' sh \
    "$((MEMORY * 1024))" "$OP" "/in/$id.$ext" "/out/$id.$ext" \
    </dev/null >"$dir/stdout.$LABEL.txt" 2>"$dir/stderr.$LABEL.txt" 3>&-
  rc=$?
  t1=$(now_ns)
  wall=$(awk -v a="$t0" -v b="$t1" 'BEGIN { printf "%.2f", (b - a) / 1e9 }')

  signal=""
  if [ "$rc" -eq 124 ]; then
    # The kill ended the podman client; the container is removed by name.
    signal=9
    podman rm -f --time 0 "$name" </dev/null >"$dir/rm.$LABEL.txt" 2>&1 3>&-
    echo "$?" >"$dir/rm-exit.$LABEL.txt" || die "cannot write the removal's exit"
  fi

  case "$rc" in
    0)
      if [ -f "$output" ]; then outcome=ok; else outcome=no-output; fi
      ;;
    124) outcome=timeout ;;
    137) outcome=cap ;;
    *) outcome=error ;;
  esac

  # The ops script's last line, when it wrote one.
  rss=$(tail -n 1 "$dir/stdout.$LABEL.txt" 2>/dev/null | sed -n 's/^openpyxl=[^\t]*\tpeak_rss_kb=\([0-9][0-9]*\)$/\1/p')

  # Anything in the output directory beside the output is a leftover.
  leftover=no
  held=$(find "$outdir" -mindepth 1 ! -path "$output" -print -quit) || die "cannot read the output directory"
  [ -n "$held" ] && leftover=yes

  out_bytes="" diff_exit="" values=$'\t\t\t\t\t\t\t\t'
  if [ "$outcome" = ok ]; then
    out_bytes=$(stat -c %s -- "$output" 2>/dev/null) || out_bytes=""
    "$XD" "$input" "$output" </dev/null >"$dir/diff.$LABEL.tsv" 2>"$dir/diff-stderr.$LABEL.txt" 3>&-
    diff_exit=$?
    if [ "$diff_exit" -eq 0 ] || [ "$diff_exit" -eq 1 ]; then
      values=$(awk -F'\t' -v OFS='\t' '
        $1 == "archive" { v[$2] = $3 }
        END { print v["identical"], v["recompressed"], v["changed"], v["dropped"], v["added"], v["flagged"], v["order_preserved"], v["comment_equal"], v["files_identical"] }
      ' "$dir/diff.$LABEL.tsv") || die "cannot read the diff rows"
      awk -F'\t' -v OFS='\t' -v id="$id" -v label="$LABEL" '$1 == "entry" { print id, label, $0 }' \
        "$dir/diff.$LABEL.tsv" >>"$ENTRIES" || die "cannot append to entries.$LABEL.tsv"
    fi
  fi

  lo=- lo_in=-
  if [ -n "$LO_IMAGE" ] && [ "$outcome" = ok ]; then
    lo_check "$output" "$dir/lo.$LABEL" "$dir/lo-reason.$LABEL.txt" "$output"
    case $? in
      0) lo=ok ;;
      1) lo=fail ;;
      *) lo=error ;;
    esac
  fi

  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$id" "$LABEL" "$producer" "$tags" \
    "$outcome" "$rc" "$signal" "$wall" "$rss" "$in_bytes" "$out_bytes" "$diff_exit" "$values" "$leftover" \
    "$lo" "$lo_in" >>"$SUMMARY" ||
    die "cannot append to summary.$LABEL.tsv"
  printf '%s %s %s %s %s\n' "$id" "$outcome" "$rc" "$wall" "${rss:--}"
  case "$outcome" in
    ok) n_ok=$((n_ok + 1)) ;;
    error) n_error=$((n_error + 1)) ;;
    timeout) n_timeout=$((n_timeout + 1)) ;;
    cap) n_cap=$((n_cap + 1)) ;;
    no-output) n_nooutput=$((n_nooutput + 1)) ;;
  esac
done <"$WORK/selected"

printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$COMMIT" "$STARTED" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  "$IMAGE_ID" "$VERSION" "$TIMEOUT" "$LO_TIMEOUT" "$MEMORY" "$OPS_SHA" "$XD_SHA" "$n_files" "$OP" >>"$RUNS" ||
  die "cannot append to run.$LABEL.tsv"
echo "files $n_files ok $n_ok error $n_error timeout $n_timeout cap $n_cap no-output $n_nooutput"

# No container of the image may outlive the run (ruling A): the EXIT trap,
# finish, counts them and prints the line (ruling I).
