#!/usr/bin/env bash
# emend T3 passthrough runner (specs/0006-passthrough.md, "The runner").
# For every manifest row, or every id listed with --ids, in manifest order:
# copy the workbook with the passthrough tool in a child process under a
# timeout and an address space limit, classify the outcome, compare input and
# output with xlsx-diff, and append to summary.<label>.tsv and
# entries.<label>.tsv in the run directory. Each invocation also appends one
# row to run.<label>.tsv (commit, times, tool versions, settings). A temporary
# file the tool left behind, in the run directory or in the child's own
# TMPDIR, is removed after the child ends and noted in the summary's leftover
# column.
# It prints one line per file (id, label, outcome, diff exit, wall seconds,
# peak RSS in kB) and a final count line. It never prints an original name, a
# path under the corpus directory or a byte of content: the child's own output
# stays in the run directory. Errors print one "error: " line without a path
# and exit 2: once the arguments are checked, the script's own stderr goes to
# /dev/null, so a failing tool or redirection cannot print a corpus path, and
# the error line goes to the saved descriptor.
set -uo pipefail
umask 077

readonly USAGE="usage: scripts/passthrough-run.sh --label <host|i686> --passthrough <path> --xlsx-diff <path> [--corpus <dir>] [--out <dir>] [--timeout <secs>] [--memory <MiB>] [--ids <file>] [--libreoffice <image>] [--lo-timeout <secs>] [--tool-arg <arg>]..."
readonly G=/usr/bin/grep
readonly MANIFEST_HEADER=$'id\tsha256\tbytes\text\tproducer\tproducer_version\tapp\tsource\ttags\tadded\toriginal_name\tnote'
readonly SUMMARY_HEADER=$'id\tlabel\tproducer\ttags\toutcome\texit\tsignal\twall_s\tpeak_rss_kb\tin_bytes\tout_bytes\tdiff_exit\tidentical\trecompressed\tchanged\tdropped\tadded\tflagged\torder_preserved\tcomment_equal\tfiles_identical\tleftover\tlo\tlo_in'
readonly ENTRIES_HEADER=$'id\tlabel\trow\tname\tstatus\tindex_in\tindex_out\tmethod_in\tmethod_out\tuncompressed_in\tuncompressed_out\tcompressed_in\tcompressed_out\tcrc_in\tcrc_out\tfirst_difference\tdiffering_bytes\tlast_difference\tflags'
readonly RUN_HEADER=$'commit\tstarted\tended\trustc\tzip\ttimeout_s\tlo_timeout_s\tmemory_mib\tpassthrough_sha256\txlsx_diff_sha256\tfiles\ttool_args'
DIE_FD=2
die() {
  echo "error: $1" >&"$DIE_FD"
  exit 2
}

# The open check and its constant live in scripts/lo-check.sh
# (specs/0015-open-checks.md, "The open-check library"), shared with
# scripts/edit-open-check.sh.
. "$(dirname -- "${BASH_SOURCE[0]}")/lo-check.sh" || die "cannot source scripts/lo-check.sh"

LABEL="" PT="" XD="" CORPUS="" OUT="" TIMEOUT=300 MEMORY=2048 IDS="" LO_IMAGE="" LO_TIMEOUT=600
# Arguments passed to the tool after the output path, in the order given
# (specs/0006-passthrough.md, "Planning ruling for Milestone 1 part 1 P2:
# tool-arg"). The word after --tool-arg is ALWAYS the value, so
# "--tool-arg --replace" passes --replace rather than reading it as an option.
TOOL_ARGS=()
while [ "$#" -gt 0 ]; do
  case "$1" in
    --label | --passthrough | --xlsx-diff | --corpus | --out | --timeout | --memory | --ids | --libreoffice | --lo-timeout | --tool-arg)
      [ "$#" -ge 2 ] || die "$1 needs a value; $USAGE"
      case "$1" in
        --label) LABEL=$2 ;;
        --passthrough) PT=$2 ;;
        --xlsx-diff) XD=$2 ;;
        --corpus) CORPUS=$2 ;;
        --out) OUT=$2 ;;
        --timeout) TIMEOUT=$2 ;;
        --memory) MEMORY=$2 ;;
        --ids) IDS=$2 ;;
        --libreoffice) LO_IMAGE=$2 ;;
        --lo-timeout) LO_TIMEOUT=$2 ;;
        --tool-arg) TOOL_ARGS+=("$2") ;;
      esac
      shift 2
      ;;
    *) die "unknown argument; $USAGE" ;;
  esac
done

# A tool argument goes into a run row, which is tab separated with one row per
# line, so a tab or a newline in a value is refused rather than written.
if [ "${#TOOL_ARGS[@]}" -gt 0 ]; then
  for arg in "${TOOL_ARGS[@]}"; do
    case "$arg" in
      *$'\t'* | *$'\n'*) die "a --tool-arg value must hold no tab or newline; $USAGE" ;;
    esac
  done
fi
# The values joined by single spaces for the run row, empty when there are
# none. IFS is still its default here, so [*] joins with one space.
TOOL_ARGS_JOINED=""
if [ "${#TOOL_ARGS[@]}" -gt 0 ]; then
  TOOL_ARGS_JOINED="${TOOL_ARGS[*]}"
fi
readonly TOOL_ARGS_JOINED

case "$LABEL" in
  host | i686) ;;
  *) die "--label must be host or i686; $USAGE" ;;
esac
[ -n "$PT" ] && [ -f "$PT" ] && [ -x "$PT" ] || die "--passthrough must name an executable file"
[ -n "$XD" ] && [ -f "$XD" ] && [ -x "$XD" ] || die "--xlsx-diff must name an executable file"
[[ "$TIMEOUT" =~ ^[1-9][0-9]*$ ]] || die "--timeout must be a positive whole number of seconds"
[[ "$MEMORY" =~ ^[1-9][0-9]*$ ]] || die "--memory must be a positive whole number of MiB"
# Each conversion gets its own budget, independent of --timeout, which keeps
# bounding the copy child only (planning's ruling 2 of 2026-09-16).
[[ "$LO_TIMEOUT" =~ ^[1-9][0-9]*$ ]] || die "--lo-timeout must be a positive whole number of seconds"
if [ -n "$LO_IMAGE" ]; then
  [[ "$LO_IMAGE" =~ ^[A-Za-z0-9][A-Za-z0-9._:/@-]*$ ]] || die "--libreoffice must name an image"
  command -v podman >/dev/null 2>&1 || die "--libreoffice needs podman"
  command -v python3 >/dev/null 2>&1 || die "--libreoffice needs python3 to count sheets and tables"
fi

exec 3>&2 2>/dev/null
DIE_FD=3

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd) || die "cannot find the repository"
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
  OUT=$CORPUS/runs/$(date -u +%Y-%m-%d)-$SHORT
fi
mkdir -p "$OUT" || die "cannot create the run directory"

WORK=$(mktemp -d) || die "cannot create a temporary directory"
trap 'rm -rf "$WORK"' EXIT

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

# An id already in this label's summary would give it a second row, which the
# record script refuses; a new run needs a new run directory.
dup=$(awk -F'\t' 'FILENAME == ARGV[1] { if (FNR > 1) have[$1] = 1; next } ($1 in have) { print $1; exit }' \
  "$SUMMARY" "$WORK/selected") || die "cannot read summary.$LABEL.tsv"
[ -z "$dup" ] || die "id $dup already has a row in summary.$LABEL.tsv; use a new run directory"

STARTED=$(date -u +%Y-%m-%dT%H:%M:%SZ)
RUSTC=$(cd "$ROOT" && RUSTUP_AUTO_INSTALL=0 rustc --version 2>/dev/null) || RUSTC=unknown
ZIP_VERSION=$(awk '$0 == "name = \"zip\"" { getline; gsub(/^version = "|"$/, ""); print; exit }' "$ROOT/Cargo.lock")
PT_SHA=$(sha256sum <"$PT" | cut -d' ' -f1) || die "cannot hash the passthrough binary"
XD_SHA=$(sha256sum <"$XD" | cut -d' ' -f1) || die "cannot hash the xlsx-diff binary"

# time_value <time file> <label>: the value after "<label>: " in GNU time's
# verbose report, or nothing.
time_value() {
  LC_ALL=C "$G" -a -m 1 -F "$2: " "$1" 2>/dev/null | sed 's/^.*: //'
}

# seconds <h:mm:ss or m:ss>: the wall time in seconds, two decimals.
seconds() {
  awk -v t="$1" 'BEGIN {
    n = split(t, p, ":")
    s = (n == 3) ? p[1] * 3600 + p[2] * 60 + p[3] : p[1] * 60 + p[2]
    printf "%.2f", s
  }'
}

n_files=0 n_ok=0 n_error=0 n_timeout=0 n_panic=0 n_crash=0
while IFS=$'\t' read -r id ext producer tags; do
  n_files=$((n_files + 1))
  dir=$OUT/$id
  mkdir -p "$dir" || die "cannot create a run subdirectory"
  input=$CORPUS/corpus/$id.$ext
  output=$dir/out.$LABEL.$ext
  timefile=$dir/time.$LABEL.txt
  rm -f "$output" "$timefile" "$dir/diff.$LABEL.tsv" "$dir/diff-stderr.$LABEL.txt" ||
    die "cannot remove an earlier output"
  rm -rf "$dir/lo.$LABEL" "$dir/lo-in.$LABEL" || die "cannot remove an earlier conversion directory"
  # A reason from an earlier run must not outlive it (independent check 1).
  rm -f "$dir/lo-reason.$LABEL.txt" "$dir/lo-in-reason.$LABEL.txt" ||
    die "cannot remove an earlier reason file"
  # A temporary file from an earlier, stopped run into this directory is
  # removed first, so leftover below counts only this child's.
  # The glob matches ANY copy tool's temporary, not only the passthrough
  # tool's (planning ruling, Milestone 1 part 1 P1, 2026-09-16, recorded in
  # specs/0006-passthrough.md). It read ".out.$LABEL.$ext.passthrough-"*.tmp
  # until then, while tools/emend-copy writes .<name>.emend-copy-<pid>.tmp,
  # so for any tool but one the leftover column read no whatever happened.
  # Here <name> is the output's own name, out.$LABEL.$ext, so that tool's
  # temporary is .out.$LABEL.$ext.emend-copy-<pid>.tmp and the glob's
  # .out. prefix does match it.
  # The leading dot and the .tmp suffix keep it off every other file in the
  # run directory, the output itself included.
  for temporary in "$dir/.out.$LABEL.$ext."*.tmp; do
    [ -e "$temporary" ] || continue
    rm -f -- "$temporary" || die "cannot remove an earlier temporary file"
  done
  in_bytes=$(stat -c %s -- "$input" 2>/dev/null) || in_bytes=""

  # The child's own TMPDIR (specs/0029-engineering-close.md, item 5 and "Planning
  # rulings during P1", rulings 2 and 3): a directory made for this child
  # alone under $WORK and exported to the child only, so anything in it after
  # the child ends is the child's leftover, never another process's, as a
  # before-and-after look at a shared TMPDIR could not tell. $WORK is made by
  # mktemp -d under the caller's TMPDIR, so the child's temporaries stay on
  # the filesystem the caller chose (specs/0023-scale.md P1 verification
  # ruling 4: the run scripts export TMPDIR as a directory on the disk).
  ctmp=$(mktemp -d "$WORK/tmp.XXXXXX") || die "cannot create the child's temporary directory"

  (
    ulimit -v $((MEMORY * 1024)) || exit 125
    export TMPDIR="$ctmp"
    exec /usr/bin/time -v -o "$timefile" timeout --signal=KILL "$TIMEOUT" "$PT" "$input" "$output" ${TOOL_ARGS[@]+"${TOOL_ARGS[@]}"}
  ) </dev/null >"$dir/stdout.$LABEL.txt" 2>"$dir/stderr.$LABEL.txt" 3>&-
  rc=$?

  # A tool stopped by a signal (the timeout's kill included) cannot remove
  # its temporary file, so the runner does, and notes it. Any copy tool's
  # temporary, on the same ruling as the clearing step above. leftover is
  # yes when the run directory's glob or the child's TMPDIR held anything,
  # no otherwise; the child's TMPDIR is then removed whole, a directory this
  # run made (specs/0029, "Planning rulings during P1", rulings 2 and 3).
  leftover=no
  for temporary in "$dir/.out.$LABEL.$ext."*.tmp; do
    [ -e "$temporary" ] || continue
    rm -f -- "$temporary" || die "cannot remove a leftover temporary file"
    leftover=yes
  done
  held=$(find "$ctmp" -mindepth 1 -print -quit) || die "cannot read the child's temporary directory"
  if [ -n "$held" ]; then
    leftover=yes
  fi
  rm -rf "${ctmp:?}" || die "cannot remove the child's temporary directory"

  # GNU time runs timeout, which runs the tool as its direct child, so the
  # timeout's SIGKILL reaches the tool itself; with time inside timeout the
  # kill would stop time and leave the tool running. time reports the tool's
  # signal as "Command terminated by signal N" (timeout re-raises it) and
  # otherwise the exit status, 124 when timeout stopped the tool. The peak RSS
  # covers timeout and the tool.
  signal_line=$(LC_ALL=C "$G" -a -m 1 -E '^Command terminated by signal [0-9]+$' "$timefile" 2>/dev/null)
  status_value=$(time_value "$timefile" "Exit status")
  exit_status="" signal=""
  if [ -n "$signal_line" ]; then
    outcome=crash signal=${signal_line##* }
  elif [ -n "$status_value" ]; then
    exit_status=$status_value
    case "$status_value" in
      0) outcome=ok ;;
      2) outcome=error ;;
      101) outcome=panic ;;
      124) outcome=timeout signal=9 ;;
      *) outcome=crash ;;
    esac
  else
    outcome=crash exit_status=$rc
  fi

  elapsed=$(time_value "$timefile" "Elapsed (wall clock) time (h:mm:ss or m:ss)")
  wall=""
  [ -n "$elapsed" ] && wall=$(seconds "$elapsed")
  rss=$(time_value "$timefile" "Maximum resident set size (kbytes)")

  out_bytes="" diff_exit="" values=$'\t\t\t\t\t\t\t\t'
  if [ "$outcome" = ok ]; then
    out_bytes=$(stat -c %s -- "$output" 2>/dev/null) || out_bytes=""
    "$XD" "$input" "$output" </dev/null >"$dir/diff.$LABEL.tsv" 2>"$dir/diff-stderr.$LABEL.txt" 3>&-
    diff_exit=$?
    # Rows are trusted only for exit 0 or 1 (specs/0004-xlsx-diff.md).
    if [ "$diff_exit" -eq 0 ] || [ "$diff_exit" -eq 1 ]; then
      values=$(awk -F'\t' -v OFS='\t' '
        $1 == "archive" { v[$2] = $3 }
        END { print v["identical"], v["recompressed"], v["changed"], v["dropped"], v["added"], v["flagged"], v["order_preserved"], v["comment_equal"], v["files_identical"] }
      ' "$dir/diff.$LABEL.tsv") || die "cannot read the diff rows"
      awk -F'\t' -v OFS='\t' -v id="$id" -v label="$LABEL" '$1 == "entry" { print id, label, $0 }' \
        "$dir/diff.$LABEL.tsv" >>"$ENTRIES" || die "cannot append to entries.$LABEL.tsv"
    fi
  fi

  # The open check, when an image was given: once on the input, and on the copy
  # when there is one. The reasons stay in the run subdirectory.
  lo=- lo_in=-
  if [ -n "$LO_IMAGE" ]; then
    lo_check "$input" "$dir/lo-in.$LABEL" "$dir/lo-in-reason.$LABEL.txt" "$input"
    case $? in
      0) lo_in=ok ;;
      1) lo_in=fail ;;
      *) lo_in=error ;;
    esac
    if [ "$outcome" = ok ]; then
      # The copy is counted against its own sheets, as the input is against
      # its own: an edit may change the sheet list (specs/0016, "Planning
      # rulings during P3", ruling 9).
      lo_check "$output" "$dir/lo.$LABEL" "$dir/lo-reason.$LABEL.txt" "$output"
      case $? in
        0) lo=ok ;;
        1) lo=fail ;;
        *) lo=error ;;
      esac
    fi
  fi

  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$id" "$LABEL" "$producer" "$tags" \
    "$outcome" "$exit_status" "$signal" "$wall" "$rss" "$in_bytes" "$out_bytes" "$diff_exit" "$values" "$leftover" \
    "$lo" "$lo_in" >>"$SUMMARY" ||
    die "cannot append to summary.$LABEL.tsv"
  printf '%s %s %s %s %s %s\n' "$id" "$LABEL" "$outcome" "${diff_exit:--}" "${wall:--}" "${rss:--}"
  case "$outcome" in
    ok) n_ok=$((n_ok + 1)) ;;
    error) n_error=$((n_error + 1)) ;;
    timeout) n_timeout=$((n_timeout + 1)) ;;
    panic) n_panic=$((n_panic + 1)) ;;
    crash) n_crash=$((n_crash + 1)) ;;
  esac
done <"$WORK/selected"

printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$COMMIT" "$STARTED" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  "$RUSTC" "$ZIP_VERSION" "$TIMEOUT" "$LO_TIMEOUT" "$MEMORY" "$PT_SHA" "$XD_SHA" "$n_files" \
  "$TOOL_ARGS_JOINED" >>"$RUNS" ||
  die "cannot append to run.$LABEL.tsv"
echo "files $n_files ok $n_ok error $n_error timeout $n_timeout panic $n_panic crash $n_crash"
