#!/usr/bin/env bash
# emend T3 passthrough runner (specs/0006-passthrough.md, "The runner").
# For every manifest row, or every id listed with --ids, in manifest order:
# copy the workbook with the passthrough tool in a child process under a
# timeout and an address space limit, classify the outcome, compare input and
# output with xlsx-diff, and append to summary.<label>.tsv and
# entries.<label>.tsv in the run directory. Each invocation also appends one
# row to run.<label>.tsv (commit, times, tool versions, settings).
# It prints one line per file (id, label, outcome, diff exit, wall seconds,
# peak RSS in kB) and a final count line. It never prints an original name, a
# path under the corpus directory or a byte of content: the child's own output
# stays in the run directory. Errors print one "error: " line without a path
# and exit 2.
set -uo pipefail
umask 077

readonly USAGE="usage: scripts/passthrough-run.sh --label <host|i686> --passthrough <path> --xlsx-diff <path> [--corpus <dir>] [--out <dir>] [--timeout <secs>] [--memory <MiB>] [--ids <file>]"
readonly G=/usr/bin/grep
readonly MANIFEST_HEADER=$'id\tsha256\tbytes\text\tproducer\tproducer_version\tapp\tsource\ttags\tadded\toriginal_name\tnote'
readonly SUMMARY_HEADER=$'id\tlabel\tproducer\ttags\toutcome\texit\tsignal\twall_s\tpeak_rss_kb\tin_bytes\tout_bytes\tdiff_exit\tidentical\trecompressed\tchanged\tdropped\tadded\tflagged\torder_preserved\tcomment_equal\tfiles_identical'
readonly ENTRIES_HEADER=$'id\tlabel\trow\tname\tstatus\tindex_in\tindex_out\tmethod_in\tmethod_out\tuncompressed_in\tuncompressed_out\tcompressed_in\tcompressed_out\tcrc_in\tcrc_out\tfirst_difference\tdiffering_bytes\tlast_difference\tflags'
readonly RUN_HEADER=$'commit\tstarted\tended\trustc\tzip\ttimeout_s\tmemory_mib\tpassthrough_sha256\txlsx_diff_sha256\tfiles'

die() {
  echo "error: $1" >&2
  exit 2
}

LABEL="" PT="" XD="" CORPUS="" OUT="" TIMEOUT=300 MEMORY=2048 IDS=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --label | --passthrough | --xlsx-diff | --corpus | --out | --timeout | --memory | --ids)
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
      esac
      shift 2
      ;;
    *) die "unknown argument; $USAGE" ;;
  esac
done

case "$LABEL" in
  host | i686) ;;
  *) die "--label must be host or i686; $USAGE" ;;
esac
[ -n "$PT" ] && [ -f "$PT" ] && [ -x "$PT" ] || die "--passthrough must name an executable file"
[ -n "$XD" ] && [ -f "$XD" ] && [ -x "$XD" ] || die "--xlsx-diff must name an executable file"
[[ "$TIMEOUT" =~ ^[1-9][0-9]*$ ]] || die "--timeout must be a positive whole number of seconds"
[[ "$MEMORY" =~ ^[1-9][0-9]*$ ]] || die "--memory must be a positive whole number of MiB"

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
  : >"$WORK/wanted"
  while IFS= read -r line || [ -n "$line" ]; do
    id=${line//[[:space:]]/}
    [ -z "$id" ] && continue
    [[ "$id" =~ ^[0-9a-f]{12}$ ]] || die "an entry in the --ids file is not a 12-digit id"
    awk -F'\t' -v id="$id" '$1 == id { found = 1 } END { exit !found }' "$WORK/rows" ||
      die "id $id is not in the manifest"
    printf '%s\n' "$id" >>"$WORK/wanted"
  done <"$IDS"
  awk -F'\t' 'NR == FNR { want[$1] = 1; next } want[$1]' "$WORK/wanted" "$WORK/rows" >"$WORK/selected"
else
  cp "$WORK/rows" "$WORK/selected"
fi

# header_ok <file> <header>: the file is absent (the header is written) or
# starts with that header.
header_ok() {
  if [ -e "$1" ]; then
    [ "$(head -n 1 "$1")" = "$2" ]
  else
    printf '%s\n' "$2" >"$1"
  fi
}
SUMMARY=$OUT/summary.$LABEL.tsv
ENTRIES=$OUT/entries.$LABEL.tsv
RUNS=$OUT/run.$LABEL.tsv
header_ok "$SUMMARY" "$SUMMARY_HEADER" || die "summary.$LABEL.tsv in the run directory has another header"
header_ok "$ENTRIES" "$ENTRIES_HEADER" || die "entries.$LABEL.tsv in the run directory has another header"
header_ok "$RUNS" "$RUN_HEADER" || die "run.$LABEL.tsv in the run directory has another header"

STARTED=$(date -u +%Y-%m-%dT%H:%M:%SZ)
RUSTC=$(cd "$ROOT" && RUSTUP_AUTO_INSTALL=0 rustc --version 2>/dev/null) || RUSTC=unknown
ZIP_VERSION=$(awk '$0 == "name = \"zip\"" { getline; gsub(/^version = "|"$/, ""); print; exit }' "$ROOT/Cargo.lock")
PT_SHA=$(sha256sum <"$PT" | cut -d' ' -f1)
XD_SHA=$(sha256sum <"$XD" | cut -d' ' -f1)

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
  rm -f "$output" "$timefile" "$dir/diff.$LABEL.tsv" "$dir/diff-stderr.$LABEL.txt"
  in_bytes=$(stat -c %s -- "$input" 2>/dev/null) || in_bytes=""

  (
    ulimit -v $((MEMORY * 1024)) || exit 125
    exec timeout --signal=KILL "$TIMEOUT" /usr/bin/time -v -o "$timefile" "$PT" "$input" "$output"
  ) </dev/null >"$dir/stdout.$LABEL.txt" 2>"$dir/stderr.$LABEL.txt"
  rc=$?

  # GNU time writes its report when the child ends. When timeout kills the
  # group, time dies with it and leaves no report; a child killed by any other
  # signal is reported as "Command terminated by signal N".
  signal_line=$(LC_ALL=C "$G" -a -m 1 -E '^Command terminated by signal [0-9]+$' "$timefile" 2>/dev/null)
  status_value=$(time_value "$timefile" "Exit status")
  exit_status="" signal=""
  if { [ "$rc" -eq 124 ] || [ "$rc" -eq 137 ]; } && [ -z "$signal_line" ] && [ -z "$status_value" ]; then
    outcome=timeout exit_status=$rc signal=9
  elif [ -n "$signal_line" ]; then
    outcome=crash signal=${signal_line##* }
  elif [ -n "$status_value" ]; then
    exit_status=$status_value
    case "$status_value" in
      0) outcome=ok ;;
      2) outcome=error ;;
      101) outcome=panic ;;
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
    "$XD" "$input" "$output" </dev/null >"$dir/diff.$LABEL.tsv" 2>"$dir/diff-stderr.$LABEL.txt"
    diff_exit=$?
    # Rows are trusted only for exit 0 or 1 (specs/0004-xlsx-diff.md).
    if [ "$diff_exit" -eq 0 ] || [ "$diff_exit" -eq 1 ]; then
      values=$(awk -F'\t' -v OFS='\t' '
        $1 == "archive" { v[$2] = $3 }
        END { print v["identical"], v["recompressed"], v["changed"], v["dropped"], v["added"], v["flagged"], v["order_preserved"], v["comment_equal"], v["files_identical"] }
      ' "$dir/diff.$LABEL.tsv")
      awk -F'\t' -v OFS='\t' -v id="$id" -v label="$LABEL" '$1 == "entry" { print id, label, $0 }' \
        "$dir/diff.$LABEL.tsv" >>"$ENTRIES"
    fi
  fi

  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$id" "$LABEL" "$producer" "$tags" \
    "$outcome" "$exit_status" "$signal" "$wall" "$rss" "$in_bytes" "$out_bytes" "$diff_exit" "$values" >>"$SUMMARY"
  printf '%s %s %s %s %s %s\n' "$id" "$LABEL" "$outcome" "${diff_exit:--}" "${wall:--}" "${rss:--}"
  case "$outcome" in
    ok) n_ok=$((n_ok + 1)) ;;
    error) n_error=$((n_error + 1)) ;;
    timeout) n_timeout=$((n_timeout + 1)) ;;
    panic) n_panic=$((n_panic + 1)) ;;
    crash) n_crash=$((n_crash + 1)) ;;
  esac
done <"$WORK/selected"

printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$COMMIT" "$STARTED" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  "$RUSTC" "$ZIP_VERSION" "$TIMEOUT" "$MEMORY" "$PT_SHA" "$XD_SHA" "$n_files" >>"$RUNS"
echo "files $n_files ok $n_ok error $n_error timeout $n_timeout panic $n_panic crash $n_crash"
