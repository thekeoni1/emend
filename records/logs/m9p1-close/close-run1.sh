#!/usr/bin/env bash
# Milestone 9 part 1 close-out (specs/0051-msrv.md, "Close-out"; planning's
# close-out brief), pre and post modes. It pushes nothing and changes
# nothing in the repository: it reads <repo> and the remote, and writes
# only under <phase>.
#
# Usage: bash <phase>/scripts/close.sh pre
#        bash <phase>/scripts/close.sh post <pushed head>
#
# Where it runs: from any current directory. It changes to <repo>
# (<home>/emend) itself and writes its log to <phase>/<mode>/<mode>.log;
# <phase>/<mode> must not exist (each mode runs once).
#
# This script carries the four fixes specs/0050 ruling H names:
#   a. each control checks its fixture was written before testing it;
#   b. the non-ASCII test of the messages has a control;
#   c. this header states where the script runs from (above);
#   d. the U+2014 test covers each commit of the range, its tree and its
#      message, as fixed bytes, with a control; no grep -P.
#
# Log form (specs/0050 ruling H): a line "$ <command>" is followed by that
# command's own output (stdout and stderr) and nothing else; every line the
# script itself prints begins "# ". A check prints "# <what>: as-expected" or
# "# <what>: no"; a control "# control: <what>: ..."; a result that is not a
# failure of the run prints "# FINDING: ...". The roots <run>, <phase>,
# <repo> and <home> stand in for the paths.
set -uo pipefail
export LC_ALL=C PYTHONDONTWRITEBYTECODE=1 GH_PAGER= GIT_PAGER=cat

MODE=${1:-}
ARG=${2:-}
case "$MODE" in
  pre) [ "$#" -eq 1 ] || { echo "usage: close.sh pre | close.sh post <pushed head>" >&2; exit 2; } ;;
  post) [ "$#" -eq 2 ] || { echo "usage: close.sh pre | close.sh post <pushed head>" >&2; exit 2; } ;;
  *) echo "usage: close.sh pre | close.sh post <pushed head>" >&2; exit 2 ;;
esac
readonly REPO=$HOME/emend
readonly PHASE=$HOME/.cache/emend-build/m9p1-close
readonly RUN=$PHASE/$MODE
[ ! -e "$RUN" ] || { echo "<phase>/$MODE exists; each mode runs once" >&2; exit 2; }
mkdir -p "$RUN" || exit 2
SELF=$(readlink -f "$0")
cd "$REPO" || exit 2

readonly BASE_FULL=2da5eb94594ae4307766c641e6f35b1b25cb3d69
readonly COAUTH="Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
EMDASH=$(printf '\342\200\224')
readonly EMDASH_SHOWN="\$'\\xe2\\x80\\x94'"

FAILS=0
FINDINGS=0
note() { echo "# $*"; }
ok() { note "$1: as-expected"; }
bad() { note "$1: no"; FAILS=$((FAILS + 1)); }
check() { local d=$1; shift; if "$@"; then ok "$d"; else bad "$d"; fi; }
accept() { local d=$1; shift; if "$@"; then ok "control: $d"; else bad "control: $d"; fi; }
not() { ! "$@"; }
finding() { note "FINDING: $1"; FINDINGS=$((FINDINGS + 1)); }
same() { [ "$1" = "$2" ]; }
# run <file> <command...>: print "$ <command>" (or SHOW), run it with stdout
# and stderr into <file>, print the file; sets RC.
run() {
  local f=$1; shift
  echo "\$ ${SHOW:-$*}"
  "$@" >"$f" 2>&1
  RC=$?
  cat "$f"
  SHOW=""
}
SHOW=""

# The message tests, each over a file holding one commit message.
# nonascii_bytes <file>: the count of bytes outside printable ASCII, tab and
# newline (tr, not grep -P).
nonascii_bytes() { tr -d '\t\n -~' <"$1" | wc -c | tr -d ' '; }
# trailers_ok <file>: exactly one Co-Authored-By line, it is the last line
# and is the expected one; no Claude-Session line.
trailers_ok() {
  [ "$(grep -c '^Co-Authored-By:' "$1")" = 1 ] &&
    [ "$(grep -c '^Claude-Session:' "$1")" = 0 ] &&
    [ "$(sed -e '/^$/d' "$1" | tail -n 1)" = "$COAUTH" ]
}
# dash_in_file <file>: the file holds U+2014 (fixed bytes).
dash_in_file() { grep -q -F "$EMDASH" "$1"; }
# fixture <file> <printf format> [args]: write a control's fixture and check
# it was written as meant (fix a): the file's bytes equal the format's.
fixture() {
  local f=$1; shift
  # shellcheck disable=SC2059
  printf "$@" >"$f"
  # shellcheck disable=SC2059
  check "the fixture $(basename "$f") was written as meant" cmp -s "$f" <(printf "$@")
}

header() {
  note "M9 part 1 close-out, $MODE log (specs/0051-msrv.md, Close-out)"
  note "run started $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  note "root: <run> stands for <phase>/$MODE, this mode's directory"
  note "root: <phase> stands for <home>/.cache/emend-build/m9p1-close, the phase's directory"
  note "root: <repo> stands for <home>/emend, the repository; the script changes to it itself"
  note "root: <home> stands for the home directory of the account the run ran under"
  note "invocation: bash <phase>/scripts/close.sh $MODE${ARG:+ $ARG}"
  note "script sha256: $(sha256sum <"$SELF" | cut -d' ' -f1)"
  note "this script, as run:"
  sed 's/^/#     /' "$SELF"
}

env_check() {
  note "## the environment"
  if [ -n "${ANTHROPIC_API_KEY+set}" ]; then bad "ANTHROPIC_API_KEY is unset"; else ok "ANTHROPIC_API_KEY is unset"; fi
}

# remote_check <expected main>
remote_check() {
  note "## the remote"
  run "$RUN/rmain" git ls-remote origin refs/heads/main
  local m
  m=$(cut -f1 "$RUN/rmain")
  check "origin's main is $1" same "$m" "$1"
  accept "the same comparison refuses an all-zero id" not same "$m" 0000000000000000000000000000000000000000
  run "$RUN/rtags" git ls-remote --tags origin
  local names
  names=$(cut -f2 "$RUN/rtags" | sed 's/\^{}$//' | sort -u | tr '\n' ' ')
  check "the remote's tags are v0.1.0 and v0.1.1 alone" same "$names" "refs/tags/v0.1.0 refs/tags/v0.1.1 "
  accept "the same comparison refuses the list with a third tag added" not same "${names}refs/tags/v0.1.2 " "refs/tags/v0.1.0 refs/tags/v0.1.1 "
}

# range_check <tip>: the list, the count, the straight line.
range_check() {
  note "## the range 2da5eb9..$(git rev-parse --short=7 "$1")"
  run "$RUN/rcount" git rev-list --count "$BASE_FULL..$1"
  RCOUNT=$(cat "$RUN/rcount")
  run "$RUN/rlist" git log --reverse --format='%h %p %s' "$BASE_FULL..$1"
  check "the range is a straight line (no commit with two parents)" same "$(git rev-list --min-parents=2 "$BASE_FULL..$1" | wc -l | tr -d ' ')" 0
  check "its first commit's parent is 2da5eb9" same "$(git rev-list --reverse "$BASE_FULL..$1" | head -n 1 | xargs -I{} git rev-parse {}^1)" "$BASE_FULL"
}

# messages_check <tip>: per commit, its message printed, then non-ASCII,
# trailers and U+2014 over the message, and U+2014 over its tree.
messages_check() {
  note "## each commit of the range: message (non-ASCII, trailers, U+2014) and tree (U+2014)"
  local c s
  for c in $(git rev-list --reverse "$BASE_FULL..$1"); do
    s=$(git rev-parse --short=7 "$c")
    run "$RUN/msg-$s" git log -1 --format=%B "$c"
    check "$s message: no non-ASCII byte" same "$(nonascii_bytes "$RUN/msg-$s")" 0
    check "$s message: one Co-Authored-By line, last, and no Claude-Session line" trailers_ok "$RUN/msg-$s"
    check "$s message: no U+2014" not dash_in_file "$RUN/msg-$s"
    SHOW="git grep -c -F $EMDASH_SHOWN $c"
    run "$RUN/tree-$s" git grep -c -F "$EMDASH" "$c"
    check "$s tree: git grep finds no U+2014 (exit 1, no output)" same "$RC:$(wc -c <"$RUN/tree-$s" | tr -d ' ')" "1:0"
  done

  note "## controls for the message and tree tests, over planted fixtures"
  fixture "$RUN/ctl-nonascii" 'x\n\nna\303\257ve\n\n%s\n' "$COAUTH"
  accept "the non-ASCII test counts 2 bytes in a planted message" same "$(nonascii_bytes "$RUN/ctl-nonascii")" 2
  fixture "$RUN/ctl-session" 'x\n\n%s\nClaude-Session: x\n' "$COAUTH"
  accept "the trailer test refuses a message with a Claude-Session line" not trailers_ok "$RUN/ctl-session"
  fixture "$RUN/ctl-twoco" 'x\n\n%s\n%s\n' "$COAUTH" "$COAUTH"
  accept "the trailer test refuses a message with two Co-Authored-By lines" not trailers_ok "$RUN/ctl-twoco"
  fixture "$RUN/ctl-good" 'x\n\n%s\n' "$COAUTH"
  accept "a message ending with the one Co-Authored-By line passes the trailer test" trailers_ok "$RUN/ctl-good"
  fixture "$RUN/ctl-dashmsg" 'x %s y\n\n%s\n' "$EMDASH" "$COAUTH"
  accept "the message test finds U+2014 in a planted message" dash_in_file "$RUN/ctl-dashmsg"
  # A scratch repository with one commit whose tree holds U+2014.
  local ctl=$RUN/ctl-repo
  run "$RUN/ctl-init" git init -q "$ctl"
  fixture "$RUN/ctl-dashfile" 'a %s b\n' "$EMDASH"
  cp "$RUN/ctl-dashfile" "$ctl/f.txt"
  run "$RUN/ctl-add" git -C "$ctl" add f.txt
  run "$RUN/ctl-commit" git -C "$ctl" -c user.name=control -c user.email=control@example.invalid commit -q -m control
  run "$RUN/ctl-show" git -C "$ctl" rev-parse HEAD
  check "the planted commit's file is the fixture" cmp -s "$RUN/ctl-dashfile" <(git -C "$ctl" show HEAD:f.txt)
  SHOW="git -C <run>/ctl-repo grep -c -F $EMDASH_SHOWN HEAD"
  run "$RUN/ctl-grep" git -C "$ctl" grep -c -F "$EMDASH" HEAD
  accept "the same tree test finds U+2014 in the planted commit (exit 0, one file)" same "$RC:$(cat "$RUN/ctl-grep")" "0:HEAD:f.txt:1"
}

privacy_check() {
  note "## the privacy scan over the range"
  SHOW="scripts/privacy.sh --range 2da5eb9..$(git rev-parse --short=7 "$1")"
  run "$RUN/privacy" scripts/privacy.sh --range "$BASE_FULL..$1"
  check "the privacy scan exits 0" same "$RC" 0
  check "it prints PASS privacy" same "$(tail -n 1 "$RUN/privacy")" "PASS privacy"
  check "it scanned the range's $RCOUNT commits with 0 findings" grep -q -E "^privacy: $RCOUNT commit\(s\), [0-9]+ blob\(s\) scanned, 0 finding\(s\)$" "$RUN/privacy"
}

pre() {
  env_check
  local head
  head=$(git rev-parse HEAD)
  run "$RUN/head" git log -1 --format='%H %s' HEAD
  check "HEAD is the close-out doc commit" same "$(git log -1 --format=%s HEAD)" "M9 part 1 close-out: commit planning docs (P2's rulings H to K and verification, the close-out)"
  range_check "$head"
  check "the range holds 9 commits: 636203f e1d8fc7 e07544e 62902dd 4e97262 ca5390f 1ab7caf 5f4e246 and HEAD" same "$(git rev-list --reverse --abbrev-commit --abbrev=7 "$BASE_FULL..$head" | head -n 8 | tr '\n' ' ')$RCOUNT" "636203f e1d8fc7 e07544e 62902dd 4e97262 ca5390f 1ab7caf 5f4e246 9"
  messages_check "$head"
  privacy_check "$head"
  note "## the gate's hygiene step at HEAD"
  run "$RUN/hygiene" scripts/check.sh --only hygiene
  check "hygiene exits 0 and prints PASS hygiene" same "$RC:$(tail -n 1 "$RUN/hygiene")" "0:PASS hygiene"
  remote_check "$BASE_FULL"
  note "## the tree"
  run "$RUN/status" git status --porcelain
  check "the tree is clean (no status line)" same "$(wc -c <"$RUN/status" | tr -d ' ')" 0
  fixture "$RUN/ctl-status" ' M ROADMAP.md\n'
  accept "the same test refuses one status line" not same "$(wc -c <"$RUN/ctl-status" | tr -d ' ')" 0
}

# gh_run_check <workflow name> <head>
gh_run_check() {
  local name=$1 head=$2 id
  SHOW="gh run list --commit $head --workflow $name --json databaseId,name,event,headSha,status,conclusion"
  run "$RUN/list-$name" gh run list --commit "$head" --workflow "$name" --json databaseId,name,event,headSha,status,conclusion
  check "gh run list for $name exits 0" same "$RC" 0
  check "it lists one run" same "$(jq length "$RUN/list-$name" 2>/dev/null)" 1
  id=$(jq -r '.[0].databaseId // empty' "$RUN/list-$name" 2>/dev/null)
  [ -n "$id" ] || { bad "a $name run id was read"; return; }
  run "$RUN/view-$name" gh run view "$id" --json name,event,headSha,status,conclusion,jobs
  check "gh run view $id exits 0" same "$RC" 0
  check "$name run $id: name $name, event push, headSha the pushed head, status completed" same "$(jq -r '[.name, .event, .headSha, .status] | join(" ")' "$RUN/view-$name")" "$name push $head completed"
  local concl
  concl=$(jq -r '.conclusion' "$RUN/view-$name")
  if [ "$concl" = success ]; then ok "$name run $id: conclusion success"; else finding "$name run $id: conclusion $concl, not success; STOP, no rerun"; fi
  note "$name run $id jobs (name: status conclusion): $(jq -r '[.jobs[] | "\(.name): \(.status) \(.conclusion)"] | join("; ")' "$RUN/view-$name")"
  check "$name run $id: every job completed with success" same "$(jq -r '[.jobs[] | select(.status != "completed" or .conclusion != "success")] | length' "$RUN/view-$name")" 0
}

post() {
  env_check
  local head
  head=$(git rev-parse --verify "$ARG^{commit}") || { bad "the argument is a commit"; return 1; }
  run "$RUN/head" git log -1 --format='%H %s' "$head"
  check "the pushed head is the local HEAD" same "$head" "$(git rev-parse HEAD)"
  remote_check "$head"
  range_check "$head"
  privacy_check "$head"
  note "## the two workflow runs of the pushed head"
  gh_run_check ci "$head"
  gh_run_check msrv "$head"
  fixture "$RUN/ctl-view.json" '{"conclusion":"failure","event":"push","headSha":"%s","jobs":[{"conclusion":"failure","name":"x","status":"completed"}],"name":"ci","status":"completed"}\n' "$head"
  accept "the conclusion test refuses a planted failure" not same "$(jq -r '.conclusion' "$RUN/ctl-view.json")" success
  accept "the job test refuses the same fixture's failed job" not same "$(jq -r '[.jobs[] | select(.status != "completed" or .conclusion != "success")] | length' "$RUN/ctl-view.json")" 0
}

main() {
  header
  "$MODE"
  note "## summary"
  note "FINDING lines: $FINDINGS"
  note "checks and controls not as expected: $FAILS"
  note "run ended $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  if [ "$FAILS" -eq 0 ] && [ "$FINDINGS" -eq 0 ]; then note "$MODE log checks all as expected: yes"; else note "$MODE log checks all as expected: no"; fi
  [ "$FAILS" -eq 0 ] && [ "$FINDINGS" -eq 0 ]
}

scrub() { sed -u -e "s#$RUN#<run>#g" -e "s#$PHASE#<phase>#g" -e "s#$REPO#<repo>#g" -e "s#$HOME#<home>#g"; }
{ main; rc=$?; note "script exit status: $rc"; exit "$rc"; } 2>&1 | scrub >"$RUN/$MODE.log"
exit "${PIPESTATUS[0]}"
