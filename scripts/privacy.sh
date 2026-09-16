#!/usr/bin/env bash
# emend privacy gate. Checks the commits in scope for identity, message, path
# and content problems before anything reaches GitHub.
#
# Usage: scripts/privacy.sh [--range <rev>]
#   Default scope: every commit reachable from HEAD. <rev> is what git
#   rev-list accepts: A..B, or one commit for all of its history.
#
# Output: "PASS privacy", or one "FAIL <check>: ..." line per finding, or one
# "ERROR privacy: ..." line. When EMEND_PRIVATE_DENYLIST is set, the first line
# is a NOTICE. A one-line summary goes to stderr.
# Exit codes: 0 pass, 1 findings, 2 error.
#
# Checks per commit: every raw header line and the message (patterns, denylist,
# email), the author and committer identity, path names (patterns, denylist,
# email), counts of corpus paths, office paths, Zone.Identifier paths and
# archive or compound-file blobs, and every blob's content.
#
# Findings name only the check, the pattern or entry number, the commit, the
# path and the line number, never the matched text. A path that itself hits
# prints as <private path>, a path containing emend-testing as <corpus path>,
# an office file outside tests/fixtures/ as <office path>, and a Zone.Identifier
# file as <zone path>. Corpus, office and Zone.Identifier paths and archive or
# compound-file blobs are counted, never named. The private
# denylist is never printed, copied to a file or passed on a command line.
set -uo pipefail

readonly G=/usr/bin/grep
readonly USAGE="usage: scripts/privacy.sh [--range <rev>]"
# scripts/privacy-patterns.txt must hold exactly this many patterns. The
# self-test plants one fake value per pattern, in the file's order.
readonly EXPECTED_PATTERNS=11
# Office and spreadsheet extensions, alone or followed by #, ~, : or a further
# extension (lock files, backups and download metadata such as
# .~lock.<name>.xlsx# and <name>.xlsx:Zone.Identifier). Keep in step with
# step 7 of scripts/check.sh and with .gitignore.
readonly OFFICE_RE='\.(xlsx|xlsm|xltx|xltm|xlsb|xls|ods|csv|tsv|fods|ots|xlt|xla|xlam|numbers)($|[#~.:])'
# Download metadata written beside a file copied from Windows, matched in any
# case. It fails wherever it is, tests/fixtures/ included.
readonly ZONE_RE=':zone[.]identifier$'
readonly FIXTURES_RE='(^|/)tests/fixtures/'
readonly EMAIL_RE='[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+([.][A-Za-z0-9-]+)*[.][A-Za-z]{2,}'

# The denylist override is for negative tests only, so it is always announced.
if [ -n "${EMEND_PRIVATE_DENYLIST+set}" ]; then
  echo "NOTICE privacy: using the denylist from EMEND_PRIVATE_DENYLIST"
fi

RANGE=HEAD
case "$#" in
  0) ;;
  2)
    if [ "$1" != --range ] || [ -z "$2" ] || [ "${2#-}" != "$2" ]; then
      echo "$USAGE" >&2
      exit 2
    fi
    RANGE=$2
    ;;
  *)
    echo "$USAGE" >&2
    exit 2
    ;;
esac

error() {
  echo "ERROR privacy: $1"
  exit 2
}

ROOT=$(git rev-parse --show-toplevel 2>/dev/null) || error "not inside a git work tree"
cd "$ROOT" || error "cannot enter the repository root"

# Read the objects a push would send: replace refs must not stand in for them,
# and history that grafts or a shallow clone hide cannot be checked.
export GIT_NO_REPLACE_OBJECTS=1
[ "$(git rev-parse --is-shallow-repository 2>/dev/null)" = false ] ||
  error "shallow repository or unreadable repository state; history cannot be checked"
[ ! -e "$(git rev-parse --git-path info/grafts)" ] || error "info/grafts exists; history cannot be checked"

readonly IDENTITY_FILE=scripts/privacy-identity.txt
readonly PATTERNS_FILE=scripts/privacy-patterns.txt
readonly DENY_FILE=${EMEND_PRIVATE_DENYLIST-$HOME/.config/emend/private-denylist.txt}
# Throwaway repositories never see the caller's git environment; git sets
# some of these variables when it runs hooks.
readonly GIT_CLEAN=(env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE
  -u GIT_OBJECT_DIRECTORY -u GIT_ALTERNATE_OBJECT_DIRECTORIES -u GIT_COMMON_DIR git)

T=$(mktemp -d) || error "mktemp failed"
trap 'rm -rf "$T"' EXIT

# run_git <stdout file> <git args...>: run "${GITX[@]}" with the arguments.
# A non-zero exit or any stderr output is an error, never an empty result.
GITX=(git)
run_git() {
  local out=$1
  shift
  "${GITX[@]}" "$@" >"$out" 2>"$T/git-err" </dev/null || error "git failed: $*"
  [ ! -s "$T/git-err" ] || error "git reported a problem: $*"
}

# --- identity -------------------------------------------------------------
[ -f "$IDENTITY_FILE" ] || error "$IDENTITY_FILE is missing"
ID_NAME=$(sed -n 1p "$IDENTITY_FILE")
ID_EMAIL=$(sed -n 2p "$IDENTITY_FILE")
[ -n "$ID_NAME" ] && [ -n "$ID_EMAIL" ] || error "$IDENTITY_FILE must hold a name and an email"

# --- patterns -------------------------------------------------------------
[ -f "$PATTERNS_FILE" ] || error "$PATTERNS_FILE is missing"
rc=0
LC_ALL=C "$G" -v -E '^[[:space:]]*(#|$)' "$PATTERNS_FILE" >"$T/patterns" 2>/dev/null || rc=$?
[ "$rc" -lt 2 ] || error "grep failed while reading $PATTERNS_FILE"
mapfile -t PATTERNS <"$T/patterns"
[ "${#PATTERNS[@]}" -eq "$EXPECTED_PATTERNS" ] ||
  error "$PATTERNS_FILE holds ${#PATTERNS[@]} pattern(s); expected $EXPECTED_PATTERNS"
PATTERN_ARGS=()
for i in "${!PATTERNS[@]}"; do
  rc=0
  LC_ALL=C "$G" -q -E -e "${PATTERNS[$i]}" /dev/null 2>/dev/null || rc=$?
  [ "$rc" -lt 2 ] || error "pattern $((i + 1)) is not a valid ERE"
  PATTERN_ARGS+=(-e "${PATTERNS[$i]}")
done

# --- private denylist -----------------------------------------------------
[ -f "$DENY_FILE" ] || error "the private denylist is missing"
[ "$(stat -c %a "$DENY_FILE")" = 600 ] || error "the private denylist must be mode 600"
DENY=()
while IFS= read -r entry || [ -n "$entry" ]; do
  entry=${entry#"${entry%%[![:space:]]*}"}
  entry=${entry%"${entry##*[![:space:]]}"}
  case "$entry" in
    '' | '#'*) ;;
    # Matching is per line, so an entry with whitespace could miss text
    # split across lines. Each name, address or word is its own entry.
    *[[:space:]]*) error "denylist entry $((${#DENY[@]} + 1)) contains whitespace" ;;
    *) DENY+=("$entry") ;;
  esac
done <"$DENY_FILE" || error "cannot read the private denylist"
unset entry
[ "${#DENY[@]}" -gt 0 ] || error "the private denylist holds no entries"

# --- scanning ---------------------------------------------------------------
# DENY_ACTIVE is the denylist in use: a sentinel during the self-test, then
# DENY. Entries reach grep through a pipe (-f <(...)), never through argv.
DENY_ACTIVE=()

# screen <file> <grep args...>: return 0 on a hit, 1 on none. The file is read
# as bytes. Exit 2 or more from grep is an error.
screen() {
  local file=$1 rc=0
  shift
  LC_ALL=C "$G" -a -q "$@" -- "$file" 2>/dev/null </dev/null || rc=$?
  [ "$rc" -lt 2 ] || error "grep failed while scanning"
  return "$rc"
}

# z_indices <NUL-separated file> <output file> <grep args...>: write the
# number of each matching record, one per line.
z_indices() {
  local file=$1 out=$2 rc
  shift 2
  LC_ALL=C "$G" -a -z -n "$@" -- "$file" 2>/dev/null </dev/null |
    LC_ALL=C sed -z 's/:.*//' | tr '\000' '\n' >"$out"
  rc=${PIPESTATUS[0]}
  [ "$rc" -lt 2 ] || error "grep failed while scanning"
}

# hit_lines <file> <result file> <label> [-z] <grep args...>: append
# "<label> <line or record number>" per match. Only the numbers leave the
# pipe; the matched text is never stored.
hit_lines() {
  local file=$1 res=$2 label=$3 rc l
  shift 3
  if [ "$1" = -z ]; then
    shift
    z_indices "$file" "$T/lines" "$@"
  else
    LC_ALL=C "$G" -a -n "$@" -- "$file" 2>/dev/null </dev/null | cut -d: -f1 >"$T/lines"
    rc=${PIPESTATUS[0]}
    [ "$rc" -lt 2 ] || error "grep failed while scanning"
  fi
  while read -r l; do
    echo "$label $l" >>"$res"
  done <"$T/lines"
}

# email_lines <file> <result file> [-z]: append "email - <line>" for every
# line (with -z, record) holding an address outside the allowlist: the
# identity address, noreply@anthropic.com and addresses at example.invalid.
# Addresses pass only through the pipe.
email_lines() {
  local file=$1 res=$2 l z=() st=()
  [ "${3:-}" != -z ] || z=(-z)
  screen "$file" "${z[@]}" -E -e "$EMAIL_RE" || return 0
  LC_ALL=C "$G" -a "${z[@]}" -o -n -E -e "$EMAIL_RE" -- "$file" 2>/dev/null </dev/null |
    tr '\000' '\n' |
    LC_ALL=C awk -v id="$ID_EMAIL" '
      { i = index($0, ":"); a = tolower(substr($0, i + 1)) }
      a == tolower(id) || a == "noreply@anthropic.com" || a ~ /@example[.]invalid$/ { next }
      { print substr($0, 1, i - 1) }' | sort -un >"$T/lines"
  st=("${PIPESTATUS[@]}")
  # Every stage must succeed: a failure after grep would drop findings.
  [ "${st[0]}" -lt 2 ] && [ "${st[1]}" -eq 0 ] && [ "${st[2]}" -eq 0 ] && [ "${st[3]}" -eq 0 ] ||
    error "the email check failed while scanning"
  while read -r l; do
    echo "email - $l" >>"$res"
  done <"$T/lines"
}

# scan_file <file> <result file> [-z]: write "patterns <n> <line>",
# "denylist <n> <line>" and "email - <line>" for every hit.
# With -z the file is NUL-separated records and numbers are record numbers.
scan_file() {
  local file=$1 res=$2 z=() i
  [ "${3:-}" != -z ] || z=(-z)
  : >"$res"
  if screen "$file" "${z[@]}" -E "${PATTERN_ARGS[@]}"; then
    for i in "${!PATTERNS[@]}"; do
      hit_lines "$file" "$res" "patterns $((i + 1))" "${z[@]}" -E -e "${PATTERNS[$i]}"
    done
  fi
  if screen "$file" "${z[@]}" -i -w -F -f <(printf '%s\n' "${DENY_ACTIVE[@]}"); then
    for i in "${!DENY_ACTIVE[@]}"; do
      hit_lines "$file" "$res" "denylist $((i + 1))" "${z[@]}" -i -w -F -f <(printf '%s\n' "${DENY_ACTIVE[$i]}")
    done
  fi
  email_lines "$file" "$res" "${z[@]}"
}

# what <kind> <n>: the words printed before the line number.
what() {
  case "$1" in
    patterns) echo "pattern $2" ;;
    denylist) echo "denylist entry $2" ;;
    *) echo email ;;
  esac
}

# emit <finding>: record one FAIL line in $SINK.
emit() {
  echo "FAIL $1" >>"$SINK"
}

# check_identity <commit>: author and committer from the raw commit header in
# $T/raw, so no mailmap or log configuration applies.
check_identity() {
  local commit=$1 role line re='^(author|committer) (.*) <([^<>]*)> [0-9]+ [-+][0-9]{4}$'
  for role in author committer; do
    line=$(awk -v r="$role " '$0 == "" { exit } index($0, r) == 1 { print; exit }' "$T/raw")
    if [[ $line =~ $re ]]; then
      [ "${BASH_REMATCH[2]}" = "$ID_NAME" ] || emit "identity: commit $commit $role name differs from $IDENTITY_FILE"
      [ "${BASH_REMATCH[3]}" = "$ID_EMAIL" ] || emit "identity: commit $commit $role email differs from $IDENTITY_FILE"
    else
      emit "identity: commit $commit $role line could not be parsed"
    fi
  done
}

# signature <file>: print the archive or compound-file format that the file's
# first bytes (for tar, the bytes at offset 257) belong to, else nothing.
# Returns 1 when the file cannot be read; the caller treats that as an error,
# never as a finding.
signature() {
  local size head tar=""
  size=$(wc -c <"$1") || return 1
  head=$(od -An -tx1 -N8 -- "$1" | tr -d ' \n') || return 1
  if [ "$size" -ge 262 ]; then
    tar=$(od -An -tx1 -j257 -N5 -- "$1" | tr -d ' \n') || return 1
  fi
  case "$head" in
    504b0304* | 504b0708*) echo zip ;;
    1f8b*) echo gzip ;;
    377abcaf271c*) echo 7z ;;
    d0cf11e0a1b11ae1*) echo ole2 ;;
    fd377a585a00*) echo xz ;;
    425a68*) echo bzip2 ;;
    28b52ffd*) echo zstd ;;
    526172211a07*) echo rar ;;
    *) [ "$tar" != 7573746172 ] || echo tar ;;
  esac
}

# check_commit <commit> <identity|no-identity>: the header, message, path and
# content checks for one commit, with "${GITX[@]}" as the git command.
declare -A BLOB_RESULT=() BLOB_SIG=()
BLOBS=0
check_commit() {
  local commit=$1 kind n l idx entry meta type sha shown count entries archives
  run_git "$T/raw" cat-file commit "$commit"
  [ "$2" != identity ] || check_identity "$commit"

  # Header: every line before the first empty line, whatever its field
  # (tree, parent, author, committer, mergetag, gpgsig, encoding or unknown).
  awk '$0 == "" { exit } { print }' "$T/raw" >"$T/header" || error "cannot read the commit header"
  scan_file "$T/header" "$T/header-res"
  while read -r kind n l; do
    emit "header: commit $commit $(what "$kind" "$n") line $l"
  done <"$T/header-res"

  # Message: everything after the header.
  sed '1,/^$/d' "$T/raw" >"$T/msg"
  if screen "$T/msg" -P '[^\x00-\x7F]'; then emit "message: commit $commit has non-ASCII bytes"; fi
  if screen "$T/msg" -e '^Claude-Session:'; then emit "message: commit $commit has a Claude-Session line"; fi
  scan_file "$T/msg" "$T/msg-res"
  while read -r kind n l; do
    emit "message: commit $commit $(what "$kind" "$n") line $l"
  done <"$T/msg-res"

  # The tree, listed three times in the same order: entries and raw paths
  # (NUL-separated) for matching, git-quoted paths for printing.
  run_git "$T/tree.z" ls-tree -r -z --full-tree "$commit"
  run_git "$T/paths.z" ls-tree -r -z --full-tree --name-only "$commit"
  run_git "$T/quoted" -c core.quotepath=true ls-tree -r --full-tree --name-only "$commit"
  mapfile -t QUOTED <"$T/quoted"
  entries=$(tr -cd '\000' <"$T/paths.z" | wc -c)
  [ "${#QUOTED[@]}" -eq "$entries" ] || error "tree listings disagree for commit $commit"

  # Corpus and office paths: counts only.
  z_indices "$T/paths.z" "$T/corpus-idx" -i -F -e emend-testing
  count=$(wc -l <"$T/corpus-idx")
  [ "$count" -eq 0 ] || emit "paths: commit $commit $count path(s) containing emend-testing (paths not printed)"
  z_indices "$T/paths.z" "$T/office-all-idx" -i -E -e "$OFFICE_RE"
  z_indices "$T/paths.z" "$T/fixtures-idx" -E -e "$FIXTURES_RE"
  rc=0
  LC_ALL=C "$G" -v -x -F -f "$T/fixtures-idx" "$T/office-all-idx" >"$T/office-idx" 2>/dev/null || rc=$?
  [ "$rc" -lt 2 ] || error "grep failed while checking office paths"
  count=$(wc -l <"$T/office-idx")
  [ "$count" -eq 0 ] || emit "paths: commit $commit $count office file path(s) outside tests/fixtures/ (paths not printed)"
  z_indices "$T/paths.z" "$T/zone-idx" -i -E -e "$ZONE_RE"
  count=$(wc -l <"$T/zone-idx")
  [ "$count" -eq 0 ] || emit "paths: commit $commit $count Zone.Identifier path(s) (paths not printed)"

  # Path names against patterns and denylist. A path that hits is never printed.
  scan_file "$T/paths.z" "$T/paths-res" -z
  : >"$T/private-idx"
  while read -r kind n l; do
    emit "paths: commit $commit tree entry $l $(what "$kind" "$n") (path not printed)"
    echo "$l" >>"$T/private-idx"
  done <"$T/paths-res"

  # Content: every blob in the tree, read as bytes whatever git attributes say.
  # A blob with an archive or compound-file signature outside tests/fixtures/
  # is counted, whatever its name.
  idx=0
  archives=0
  while IFS= read -r -d '' entry; do
    idx=$((idx + 1))
    meta=${entry%%$'\t'*}
    read -r _ type sha <<<"$meta"
    [ "$type" = blob ] || continue
    if [ -z "${BLOB_RESULT[$sha]+set}" ]; then
      run_git "$T/blob" cat-file blob "$sha"
      scan_file "$T/blob" "$T/res/$sha"
      BLOB_RESULT[$sha]=$T/res/$sha
      signature "$T/blob" >"$T/sig" || error "cannot read the first bytes of a blob"
      BLOB_SIG[$sha]=$(<"$T/sig")
      BLOBS=$((BLOBS + 1))
    fi
    if [ -n "${BLOB_SIG[$sha]}" ] && ! screen "$T/fixtures-idx" -x -F -e "$idx"; then
      archives=$((archives + 1))
    fi
    [ -s "${BLOB_RESULT[$sha]}" ] || continue
    if screen "$T/private-idx" -x -F -e "$idx"; then
      shown='<private path>'
    elif screen "$T/corpus-idx" -x -F -e "$idx"; then
      shown='<corpus path>'
    elif screen "$T/office-idx" -x -F -e "$idx"; then
      shown='<office path>'
    elif screen "$T/zone-idx" -x -F -e "$idx"; then
      shown='<zone path>'
    else
      shown=${QUOTED[$((idx - 1))]}
    fi
    while read -r kind n l; do
      if [ "$kind" = email ]; then
        emit "email: commit $commit path $shown line $l"
      else
        emit "$kind: commit $commit path $shown $(what "$kind" "$n") line $l"
      fi
    done <"${BLOB_RESULT[$sha]}"
  done <"$T/tree.z"
  [ "$archives" -eq 0 ] ||
    emit "archives: commit $commit $archives blob(s) with an archive or compound-file signature outside tests/fixtures/ (paths not printed)"
}

# --- self-test ---------------------------------------------------------------
hex() { od -An -N"$1" -tx1 /dev/urandom | tr -d ' \n'; }

# A throwaway repository with one realistic fake value per pattern, a
# denylist sentinel, a disallowed and allowed email addresses, corpus, office,
# fixture and private paths, an address in a path name, archive and
# compound-file blobs inside and outside tests/fixtures/, Zone.Identifier files,
# the P6b forms of patterns 10 and 11, and an extra commit header line. Every planted
# hit must be reported, each pattern under its own number, and no allowed
# address.
selftest() {
  local repo=$T/selftest sentinel email head plain w i
  local -a fakes want
  sentinel="zq$(hex 8)"
  email="zq$(hex 4)@example"".com"
  fakes=(
    "gh""p_$(hex 18)"
    "github""_pat_$(hex 11)_$(hex 30)"
    "sk""-ant-api03-$(hex 40)"
    "-----BEGIN OPENSSH ""PRIVATE"" KEY-----"
    "https://claude"".ai/code/""session_$(hex 12)"
    "session""_$(hex 13)"
    "/tmp/""claude-$((RANDOM + 1000))/-home-user-project/$(hex 4)"
    "/mnt/""c/""Users/user$(hex 2)/Documents"
    "DESKTOP""-$(hex 4 | tr a-f A-F)"
    "C:""\\\\USERS\\\\user$(hex 2)\\\\Documents"
    "%USERPROFILE%\\"".claude""\\projects\\-home-user-project\\$(hex 8).jsonl"
  )
  [ "${#fakes[@]}" -eq "$EXPECTED_PATTERNS" ] || error "self-test holds ${#fakes[@]} fake values; expected $EXPECTED_PATTERNS"

  GITX=("${GIT_CLEAN[@]}" -c init.defaultBranch=selftest)
  run_git "$T/git-out" init -q "$repo"
  mkdir -p "$repo/Emend-Testing" "$repo/tools/tests/fixtures" || error "mkdir failed"
  printf '%s\n' "${fakes[@]}" >"$repo/a.txt"
  # The P6b forms of patterns 10 and 11 (a single separator, lower-case Users,
  # forward slashes), so an edit that narrows either pattern is caught.
  printf '%s\n' "C:""\\Users\\user$(hex 2)\\Documents" \
    "/home/user/"".claude""/projects/-home-user-project/$(hex 8).jsonl" >"$repo/w.txt"
  # P6d: one bare shape per narrowed path pattern, with nothing after the
  # trailing separator. A real path always has a following segment; a name
  # written in prose does not, and must not be reported, so that a record, a
  # log or a checker's answer may name the shapes.
  printf '%s\n' "/mnt/""c/""Users/" "C:""\\\\USERS\\\\" "%USERPROFILE%\\"".claude""\\projects\\" >"$repo/bare.txt"
  printf 'self-test sentinel %s\n' "$sentinel" >"$repo/b.txt"
  printf 'contact %s\n' "$email" >"$repo/e.txt"
  printf '%s noreply@anthropic.com zq%s@example.invalid\n' "$ID_EMAIL" "$(hex 2)" >"$repo/ok.txt"
  printf '%s\n' "${fakes[0]}" >"$repo/Emend-Testing/x.txt"
  printf '%s\n' "${fakes[0]}" >"$repo/r.XLSX"
  printf '%s\n' "${fakes[0]}" >"$repo/n-$sentinel.txt"
  printf 'fixture\n' >"$repo/tools/tests/fixtures/f.xlsx"
  printf 'x\n' >"$repo/q.CSV.bak"
  printf 'plain\n' >"$repo/m-$email.txt"
  printf 'PK\003\004self-test\n' >"$repo/z.bin"
  printf 'PK\003\004self-test\n' >"$repo/tools/tests/fixtures/z.bin"
  printf '\037\213self-test\n' >"$repo/g.dat"
  printf '7z\274\257\047\034self-test\n' >"$repo/s.txt"
  printf '\320\317\021\340\241\261\032\341self-test\n' >"$repo/sig-ole2"
  printf '\375\067\172\130\132\000self-test\n' >"$repo/sig-xz"
  printf 'BZh9self-test\n' >"$repo/sig-bzip2"
  printf '\050\265\057\375self-test\n' >"$repo/sig-zstd"
  printf 'Rar!\032\007self-test\n' >"$repo/sig-rar"
  printf 'PK\007\010self-test\n' >"$repo/sig-split-zip"
  { printf '%257s' ''; printf 'ustar self-test\n'; } >"$repo/sig-tar"
  printf '[ZoneTransfer]\nZoneId=3\n' >"$repo/q.xlsx:Zone.Identifier"
  printf '%s\n' "${fakes[0]}" >"$repo/tools/tests/fixtures/f.xlsx:Zone.Identifier"
  GITX=("${GIT_CLEAN[@]}" -C "$repo" -c user.name=selftest -c user.email=selftest@example.invalid
    -c commit.gpgsign=false -c core.hooksPath=/dev/null)
  run_git "$T/git-out" add -f .
  run_git "$T/git-out" commit -q -m "self-test ${fakes[0]}" -m "$email"
  run_git "$T/git-out" rev-parse HEAD
  plain=$(cat "$T/git-out")
  run_git "$T/raw-plain" cat-file commit "$plain"
  awk -v x="x-emend-selftest ${fakes[0]} $sentinel $email" \
    '{ print } /^committer / && !done { print x; done = 1 }' "$T/raw-plain" >"$T/raw-extra" ||
    error "cannot build the self-test commit"
  run_git "$T/git-out" hash-object -t commit -w "$T/raw-extra"
  head=$(cat "$T/git-out")

  DENY_ACTIVE=("$sentinel")
  SINK=$T/selftest-findings
  : >"$SINK"
  mkdir "$T/res" || error "mkdir failed"
  check_commit "$head" no-identity
  GITX=(git)

  for i in "${!fakes[@]}"; do
    screen "$SINK" -x -F -e "FAIL patterns: commit $head path a.txt pattern $((i + 1)) line $((i + 1))" ||
      error "self-test failed: pattern $((i + 1)) did not detect its fake value"
  done
  for i in 10 11; do
    screen "$SINK" -x -F -e "FAIL patterns: commit $head path w.txt pattern $i line $((i - 9))" ||
      error "self-test failed: pattern $i did not detect its P6b-form fake value"
  done
  want=(
    "FAIL denylist: commit $head path b.txt denylist entry 1 line 1"
    "FAIL email: commit $head path e.txt line 1"
    "FAIL message: commit $head pattern 1 line 1"
    "FAIL message: commit $head email line 3"
    "FAIL paths: commit $head 1 path(s) containing emend-testing (paths not printed)"
    "FAIL paths: commit $head 3 office file path(s) outside tests/fixtures/ (paths not printed)"
    "FAIL paths: commit $head 2 Zone.Identifier path(s) (paths not printed)"
    "FAIL archives: commit $head 10 blob(s) with an archive or compound-file signature outside tests/fixtures/ (paths not printed)"
    "FAIL patterns: commit $head path <zone path> pattern 1 line 1"
    "FAIL header: commit $head pattern 1 line 4"
    "FAIL header: commit $head denylist entry 1 line 4"
    "FAIL header: commit $head email line 4"
    "FAIL patterns: commit $head path <corpus path> pattern 1 line 1"
    "FAIL patterns: commit $head path <office path> pattern 1 line 1"
    "FAIL patterns: commit $head path <private path> pattern 1 line 1"
  )
  for w in "${want[@]}"; do
    screen "$SINK" -x -F -e "$w" || error "self-test failed: a planted hit was not detected"
  done
  screen "$SINK" -E -e "^FAIL paths: commit $head tree entry [0-9]+ denylist entry 1 [(]path not printed[)]\$" ||
    error "self-test failed: a planted hit was not detected"
  screen "$SINK" -E -e "^FAIL paths: commit $head tree entry [0-9]+ email [(]path not printed[)]\$" ||
    error "self-test failed: a planted hit was not detected"
  if screen "$SINK" -F -e "path ok.txt"; then
    error "self-test failed: an allowed address was reported"
  fi
  if screen "$SINK" -F -e "path bare.txt"; then
    error "self-test failed: a bare pattern shape was reported"
  fi

  DENY_ACTIVE=("${DENY[@]}")
  BLOB_RESULT=()
  BLOB_SIG=()
  BLOBS=0
  rm -rf "$T/res" "$T/selftest" && mkdir "$T/res" || error "cannot reset after the self-test"
}
selftest

# --- commits in scope ------------------------------------------------------
SINK=$T/findings
: >"$SINK"
[ "$(git config --local --get user.name)" = "$ID_NAME" ] ||
  emit "identity: local git config user.name differs from $IDENTITY_FILE"
[ "$(git config --local --get user.email)" = "$ID_EMAIL" ] ||
  emit "identity: local git config user.email differs from $IDENTITY_FILE"

run_git "$T/commits" rev-list "$RANGE"
COMMITS=0
while read -r commit; do
  COMMITS=$((COMMITS + 1))
  check_commit "$commit" identity
done <"$T/commits"

FINDINGS=$(wc -l <"$SINK")
echo "privacy: $COMMITS commit(s), $BLOBS blob(s) scanned, $FINDINGS finding(s)" >&2
if [ "$FINDINGS" -gt 0 ]; then
  cat "$SINK"
  exit 1
fi
echo "PASS privacy"
