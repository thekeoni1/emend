#!/usr/bin/env bash
# emend privacy gate. Checks the commits in scope for identity, message and
# content problems before anything reaches GitHub.
#
# Usage: scripts/privacy.sh [--range <rev>]
#   Default scope: every commit reachable from HEAD. <rev> is what git
#   rev-list accepts: A..B, or one commit for all of its history.
#
# Output: "PASS privacy", or one "FAIL <check>: ..." line per finding, or one
# "ERROR privacy: ..." line. A one-line summary goes to stderr.
# Exit codes: 0 pass, 1 findings, 2 error.
#
# Findings name only the check, the pattern or entry number, the commit, the
# path and the line number, never the matched text. A path that itself hits
# prints as <private path>, and a path containing emend-testing prints as
# <corpus path>. The private denylist is never printed, copied to a file or
# passed on a command line.
set -uo pipefail

readonly G=/usr/bin/grep
readonly USAGE="usage: scripts/privacy.sh [--range <rev>]"

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
readonly DENY_FILE=${EMEND_PRIVATE_DENYLIST:-$HOME/.config/emend/private-denylist.txt}
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

# hit_lines <file> <result file> <label> <grep args...>: append
# "<label> <line number>" per matching line. Only the line numbers leave the
# pipe; the matched text is never stored.
hit_lines() {
  local file=$1 res=$2 label=$3 rc l
  shift 3
  LC_ALL=C "$G" -a -n "$@" -- "$file" 2>/dev/null </dev/null | cut -d: -f1 >"$T/lines"
  rc=${PIPESTATUS[0]}
  [ "$rc" -lt 2 ] || error "grep failed while scanning"
  while read -r l; do
    echo "$label $l" >>"$res"
  done <"$T/lines"
}

# scan_file <file> <result file>: write "patterns <n> <line>" and
# "denylist <n> <line>" for every hit.
scan_file() {
  local file=$1 res=$2 i
  : >"$res"
  if [ "${#PATTERNS[@]}" -gt 0 ] && screen "$file" -E "${PATTERN_ARGS[@]}"; then
    for i in "${!PATTERNS[@]}"; do
      hit_lines "$file" "$res" "patterns $((i + 1))" -E -e "${PATTERNS[$i]}"
    done
  fi
  if screen "$file" -i -w -F -f <(printf '%s\n' "${DENY_ACTIVE[@]}"); then
    for i in "${!DENY_ACTIVE[@]}"; do
      hit_lines "$file" "$res" "denylist $((i + 1))" -i -w -F -f <(printf '%s\n' "${DENY_ACTIVE[$i]}")
    done
  fi
}

# what <kind>: the word printed before the number.
what() {
  if [ "$1" = patterns ]; then echo pattern; else echo "denylist entry"; fi
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

# check_commit <commit> <identity|no-identity>: the message and content checks
# for one commit, with "${GITX[@]}" as the git command.
declare -A BLOB_RESULT=()
BLOBS=0
check_commit() {
  local commit=$1 kind n l idx entry meta path type sha shown rc
  run_git "$T/raw" cat-file commit "$commit"
  [ "$2" != identity ] || check_identity "$commit"

  # Message: everything after the header.
  sed '1,/^$/d' "$T/raw" >"$T/msg"
  if screen "$T/msg" -P '[^\x00-\x7F]'; then emit "message: commit $commit has non-ASCII bytes"; fi
  if screen "$T/msg" -e '^Claude-Session:'; then emit "message: commit $commit has a Claude-Session line"; fi
  scan_file "$T/msg" "$T/msg-res"
  while read -r kind n l; do
    emit "message: commit $commit $(what "$kind") $n line $l"
  done <"$T/msg-res"

  # Path names in the tree. A path that hits is never printed.
  run_git "$T/tree" -c core.quotepath=true ls-tree -r --full-tree "$commit"
  cut -f2- "$T/tree" >"$T/paths"
  scan_file "$T/paths" "$T/paths-res"
  : >"$T/private-lines"
  while read -r kind n l; do
    emit "paths: commit $commit tree entry $l $(what "$kind") $n (path not printed)"
    echo "$l" >>"$T/private-lines"
  done <"$T/paths-res"

  # Content: every blob in the tree, read as bytes whatever git attributes say.
  idx=0
  while IFS= read -r entry; do
    idx=$((idx + 1))
    meta=${entry%%$'\t'*}
    path=${entry#*$'\t'}
    read -r _ type sha <<<"$meta"
    [ "$type" = blob ] || continue
    if [ -z "${BLOB_RESULT[$sha]+set}" ]; then
      run_git "$T/blob" cat-file blob "$sha"
      scan_file "$T/blob" "$T/res/$sha"
      BLOB_RESULT[$sha]=$T/res/$sha
      BLOBS=$((BLOBS + 1))
    fi
    [ -s "${BLOB_RESULT[$sha]}" ] || continue
    if screen "$T/private-lines" -x -F -e "$idx"; then
      shown='<private path>'
    else
      rc=0
      printf '%s\n' "$path" | LC_ALL=C "$G" -q -i -F -e emend-testing 2>/dev/null || rc=$?
      [ "$rc" -lt 2 ] || error "grep failed while checking a path"
      if [ "$rc" -eq 0 ]; then shown='<corpus path>'; else shown=$path; fi
    fi
    while read -r kind n l; do
      emit "$kind: commit $commit path $shown $(what "$kind") $n line $l"
    done <"${BLOB_RESULT[$sha]}"
  done <"$T/tree"
}

# --- self-test: a throwaway repository with a pattern hit and a sentinel ----
selftest() {
  local repo=$T/selftest sentinel token head want
  sentinel="zq$(od -An -N8 -tx1 /dev/urandom | tr -d ' \n')"
  token="gh""p_$(od -An -N18 -tx1 /dev/urandom | tr -d ' \n')"
  GITX=("${GIT_CLEAN[@]}" -c init.defaultBranch=selftest)
  run_git "$T/git-out" init -q "$repo"
  printf 'self-test token %s\n' "$token" >"$repo/a.txt"
  printf 'self-test sentinel %s\n' "$sentinel" >"$repo/b.txt"
  GITX=("${GIT_CLEAN[@]}" -C "$repo" -c user.name=selftest -c user.email=selftest@example.invalid
    -c commit.gpgsign=false -c core.hooksPath=/dev/null)
  run_git "$T/git-out" add a.txt b.txt
  run_git "$T/git-out" commit -q -m "self-test $token"
  run_git "$T/git-out" rev-parse HEAD
  head=$(cat "$T/git-out")

  DENY_ACTIVE=("$sentinel")
  SINK=$T/selftest-findings
  : >"$SINK"
  mkdir "$T/res" || error "mkdir failed"
  check_commit "$head" no-identity
  for want in \
    "^FAIL patterns: commit $head path a[.]txt pattern [0-9]+ line 1\$" \
    "^FAIL denylist: commit $head path b[.]txt denylist entry 1 line 1\$" \
    "^FAIL message: commit $head pattern [0-9]+ line 1\$"; do
    screen "$SINK" -E -e "$want" || error "self-test failed: a planted hit was not detected"
  done

  GITX=(git)
  DENY_ACTIVE=("${DENY[@]}")
  BLOB_RESULT=()
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
