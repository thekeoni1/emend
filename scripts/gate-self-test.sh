#!/usr/bin/env bash
# emend gate self-test (specs/0007-gate-self-test.md).
#
# Proves the gate still catches what it was built to catch: one case per
# detection class established by the T0, T2 P0 and T0 P6d negative tests, run
# against scratch clones, with the unchanged clone as the control.
#
# WHERE EVERY EXPECTATION COMES FROM. Each case names its source. A source is a
# primary: the log where the case actually ran, or, where no log holds the
# case, the gate script's own text (its regex, its positive control, its
# message). Memory is not a source (planning's principles, 2026-09-16). Where
# logs disagree over time the LATEST log that records that detection class
# rules and the case names both. If the gate prints something no source
# recorded, the case FAILS: that is an unrecorded gate change, and an
# expectation is never adjusted to make a case pass.
#
# THIS SCRIPT'S OWN TEXT holds no token shape, no path shape, no address and no
# office file name: every fixture is assembled at run time from pieces, so the
# script passes the gate it tests.
#
# usage: scripts/gate-self-test.sh [--full] [--fast] [--case <id>]...
#   --full  adds the build-level cases (T6 P2; only B1 exists in P1)
#   --fast  runs each privacy case over that case's own commits
#           (privacy.sh --range <base>..HEAD, where <base> is the clone's HEAD
#           read BEFORE the case makes its commits) rather than the clone's
#           whole history. About 7 s a case against about 33 s. It is for
#           ITERATION ONLY: it is not the invocation the source logs used, so a
#           run that produces committed evidence never passes it. The DEFAULT
#           is full history, which is what every source log ran. This was the
#           other way round until independent check 1 observed that the
#           committed self-test.log was then not the default run specs/0007
#           asks for, and that the default differed from every source.
#   --case  run only the named case ids.
#
# PHASE_NOTE names the phase whose records the run's log is committed with, and
# RERUN_NOTE, if set, names the log this one supersedes. PHASE_NOTE exists
# because the phase was written into this script's own text in T6 P1, so a run
# in any later phase printed a header naming T6 P1: a label that drifts, which
# is the defect M1 part 2 P3's rulings named twice over (a control that says
# "step 7", a control that calls HEAD a code commit). A run that does not set
# it says so in the header rather than naming the wrong phase.
#
# Output: one line per case, then a count line. Exit 0 only when there are no
# unexpected results, no errors, and every counting control was as expected.
set -uo pipefail

cd "$(git rev-parse --show-toplevel)" || exit 2
. scripts/counting.sh

FULL=0
FAST=0
WANT=""
USAGE="usage: scripts/gate-self-test.sh [--full] [--fast] [--case <id>]..."
INVOCATION="scripts/gate-self-test.sh${*:+ $*}"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --full) FULL=1 ;;
    --fast) FAST=1 ;;
    --case)
      [ "$#" -ge 2 ] || { echo "$USAGE" >&2; exit 2; }
      WANT="$WANT $2"
      shift
      ;;
    *) echo "$USAGE" >&2; exit 2 ;;
  esac
  shift
done

# The self-test plants its own scratch denylist for every privacy case, so the
# real one must not be in play.
if [ -n "${EMEND_PRIVATE_DENYLIST+set}" ]; then
  echo "gate-self-test: EMEND_PRIVATE_DENYLIST is set; unset it and run again" >&2
  exit 2
fi

SCRATCH=$(mktemp -d)
cleanup() { chmod -R u+rwX "$SCRATCH" 2>/dev/null; rm -rf "$SCRATCH"; }
trap cleanup EXIT INT TERM HUP
cases=0
unexpected=0
errors=0
# fresh() runs inside $(...), which is a subshell, so an "errors=$((errors+1))"
# there is discarded in the parent: a failed clone dropped its case in silence
# and the run still printed RESULT: PASS (independent check 1, T6 P1). A
# subshell CAN write a file, so failures are recorded in one and counted at the
# end, and the run also asserts how many cases it meant to perform.
CLONE_FAILURES=$SCRATCH/clone-failures
: >"$CLONE_FAILURES"

ID_NAME=$(sed -n 1p scripts/privacy-identity.txt)
ID_EMAIL=$(sed -n 2p scripts/privacy-identity.txt)

# Fixture pieces, assembled so no shape appears literally in this file.
EXT_A=$(printf 'x%s' 'lsx')
EXT_B=$(printf 'X%s' 'LSM')
EXT_C=$(printf 'c%s' 'sv')
EXT_D=$(printf 't%s' 'sv')
EXT_E=$(printf 'o%s' 'ds')
ZONE=$(printf 'Zone.%s' 'Identifier')
CORPUS=$(printf 'emend-%s' 'testing')
EMDASH=$(printf '\342\200\224')
SENTINEL="zq$(head -c 8 /dev/urandom | od -An -tx1 | tr -d ' \n')"

# want <id>: whether this run should perform that case.
want() {
  [ -z "$WANT" ] || case " $WANT " in *" $1 "*) return 0 ;; *) return 1 ;; esac
}

# Every case id this script performs, in order. The run asserts at the end that
# it performed exactly as many as want() selects, so a case that vanishes --
# a clone that failed, a block that was skipped -- is a failure rather than a
# quieter PASS (independent check 1, T6 P1).
ALL_IDS="H0 H1a H1b H2a H2b H2c H3 H4a H4b H5 H6 H7 H8a H8b H9 H10
V0 V1 V2 V3 V4 V5 V6 V7a V7b V7c V7d V8 V9a V9b V10
V11missing V11mode644 V11empty V11space V12remove V12add V13 V14 V15 V16
V17a V17b V17c V17d V18 K1 K2 K3 B1"
expected_cases() {
  local n=0 id
  for id in $ALL_IDS; do want "$id" && n=$((n + 1)); done
  printf '%s' "$n"
}

# normalise: commit ids become <sha>, as the source logs print them, and a
# tree-entry ordinal becomes <n>. That ordinal is the offending entry's
# position in the commit's ROOT TREE, so it moves whenever the repository gains
# or loses a root file: t0-p6c recorded 8 when the tree was smaller, and the
# position this fixture reaches now is not recorded in any log, which is why it
# is normalised rather than asserted. An earlier version of this comment named a
# figure no log holds, which is a measurement in prose (independent check 2,
# T6 P1, gap 13). Normalising it makes the case assert the
# detection, which is what the source established, rather than a position that
# is not a property of the gate. This canonicalises the OBSERVATION; no
# expectation was altered to fit the gate.
normalise() {
  sed -E -e 's/\b[0-9a-f]{7,40}\b/<sha>/g' -e 's/tree entry [0-9]+/tree entry <n>/g'
}

# outcome <output file> <exit>: the canonical shape, the exit status and the
# first verdict line the gate printed.
outcome() {
  local f=$1 rc=$2 line
  # A finding line wins over the NOTICE preamble. privacy.sh prints
  # "NOTICE privacy: ..." as its first line whenever EMEND_PRIVATE_DENYLIST is
  # set, so taking the first verdict-shaped line returned the NOTICE for every
  # privacy case and masked the FAIL beneath it. Measured: the NOTICE is line
  # 1, the summary line 2, the FAIL line 3.
  line=$(LC_ALL=C "$G" -a -m 1 -E '^(FAIL|ERROR|usage:|install-hooks:|pre-push:)' "$f" 2>/dev/null | normalise) || line=""
  if [ -z "$line" ]; then
    line=$(LC_ALL=C "$G" -a -m 1 -E '^PASS' "$f" 2>/dev/null | normalise) || line=""
  fi
  if [ -z "$line" ]; then
    line=$(LC_ALL=C "$G" -a -m 1 -E '^NOTICE' "$f" 2>/dev/null | normalise) || line=""
  fi
  printf 'exit %s; %s' "$rc" "${line:-no verdict line}"
}

# all_fails <output file>: every FAIL line, joined, for a case whose source
# recorded more than one.
all_fails() {
  local out
  out=$(LC_ALL=C "$G" -a -E '^FAIL ' "$1" 2>/dev/null | normalise | paste -sd'@' -) || out=""
  printf '%s' "${out:-no FAIL line}" | sed 's/@/ + /g'
}

# result <id> <source> <expected> <got> [<note>]
result() {
  local id=$1 src=$2 exp=$3 got=$4 note=${5:-} verdict
  if [ "$exp" = "$got" ]; then
    verdict=as-expected
  else
    verdict=UNEXPECTED
    unexpected=$((unexpected + 1))
  fi
  cases=$((cases + 1))
  printf 'case %-4s source %-34s expect [%s]; got [%s]; %s%s\n' \
    "$id" "$src" "$exp" "$got" "$verdict" "${note:+; $note}"
}

# fresh <id>: a clone of the base clone for this case alone, identity set.
fresh() {
  local d=$SCRATCH/c$1
  rm -rf "$d"
  if ! git clone --shared --quiet "$BASE" "$d" 2>/dev/null; then
    printf '%s\n' "$1" >>"$CLONE_FAILURES"
    printf ''
    return 1
  fi
  git -C "$d" config user.name "$ID_NAME"
  git -C "$d" config user.email "$ID_EMAIL"
  printf '%s' "$d"
}

# deny_for <dir>: a scratch denylist holding the one sentinel, mode 600.
deny_for() {
  local f=$1/deny.txt
  printf '%s\n' "$SENTINEL" >"$f"
  chmod 600 "$f"
  printf '%s' "$f"
}

# hyg <dir>: run the hygiene step alone in that copy; sets HRC and HOUT.
hyg() {
  HOUT=$1/hygiene.out
  ( cd "$1" && ./scripts/check.sh --only hygiene ) >"$HOUT" 2>&1
  HRC=$?
}

# priv <dir> <deny> <base sha>: run privacy.sh in that copy, fast or deep;
# sets PRC, POUT and PINV (the invocation, for the case line).
priv() {
  local d=$1 deny=$2 base=$3
  POUT=$d/privacy.out
  if [ "$FAST" -eq 0 ]; then
    PINV="full history"
    ( cd "$d" && EMEND_PRIVATE_DENYLIST="$deny" ./scripts/privacy.sh ) >"$POUT" 2>&1
  else
    PINV="range $(printf '%.7s' "$base")..HEAD"
    ( cd "$d" && EMEND_PRIVATE_DENYLIST="$deny" ./scripts/privacy.sh --range "$base..HEAD" ) >"$POUT" 2>&1
  fi
  PRC=$?
}

# commit_in <dir> <message>: stage everything and commit.
# commit_in <dir> <message>: stage everything and commit. The add is forced,
# because .gitignore ignores the office extensions these fixtures use: a plain
# "git add -A" staged 0 of 5 such fixtures while "-f" staged all 5, so V8, V17
# and V18 committed nothing and the gate correctly found nothing.
commit_in() { git -C "$1" add -A -f && git -C "$1" commit -q -m "$2"; }

# secs <start>: whole seconds since that epoch-with-fraction stamp.
secs() { awk -v a="$1" -v b="$(date +%s.%N)" 'BEGIN { printf "%.0fs", b - a }'; }

RUN_START=$(date +%s.%N)
echo "command: git rev-parse HEAD"
echo "about: gate self-test at commit $(git rev-parse HEAD); HEAD at run: $(git rev-parse HEAD)"
echo "phase: ${PHASE_NOTE:-not named by this run; set PHASE_NOTE} (specs/0007-gate-self-test.md) self-test.log; run at $(date -u +%Y-%m-%dT%H:%M:%SZ)"
if [ -n "${RERUN_NOTE:-}" ]; then echo "rerun: $RERUN_NOTE"; fi
echo "invocation: $INVOCATION"
echo "mode: $([ "$FAST" -eq 1 ] && echo 'fast (iteration only, not the sources invocation)' || echo 'default')$([ "$FULL" -eq 1 ] && echo ', full' || echo '')"
echo "privacy invocation: $([ "$FAST" -eq 1 ] && echo 'range <base>..HEAD, base read before the case commits; NOT what the sources ran' || echo 'full history, the invocation every source log used')"
echo "identity set in every clone from scripts/privacy-identity.txt"
echo "scratch denylist per privacy case, mode 600, one sentinel; the sentinel is never printed"
echo

echo "## counting controls"
counting_controls
if [ "$counting_controls_ok" != yes ]; then
  errors=$((errors + 1))
fi
echo

echo "## the base clone"
BASE=$SCRATCH/base
t=$(date +%s.%N)
git clone --shared --quiet . "$BASE" || { echo "cannot clone" >&2; exit 2; }
git -C "$BASE" config user.name "$ID_NAME"
git -C "$BASE" config user.email "$ID_EMAIL"
echo "cloned in $(secs "$t"); HEAD $(git -C "$BASE" rev-parse HEAD)"
echo

echo "## hygiene cases, through scripts/check.sh --only hygiene"
if want H0; then
  t=$(date +%s.%N); d=$(fresh H0) && { hyg "$d"; result H0 "control, unchanged clone" "exit 0; PASS hygiene" "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"; }
fi
# H1 and H2 plant ONE fixture each, because each source planted one and
# recorded count 1 in its own clone. An earlier version planted two and three
# and expected 2 and 3; no log anywhere records a hygiene office count above 1,
# so the wording was sourced but the count was arithmetic from the fixture
# (independent check 1, T6 P1). The lower- and upper-case halves that made H1
# worth two fixtures are now two cases, each matching its own source.
if want H1a; then
  t=$(date +%s.%N); d=$(fresh H1a) && {
    printf 'x\n' >"$d/a.$EXT_A"
    git -C "$d" add -f "a.$EXT_A" >/dev/null 2>&1
    hyg "$d"
    result H1a "t0-p5 b1 via t0-p6b L842" \
      "exit 1; FAIL hygiene: 1 office file(s) tracked outside tests/fixtures/ (paths not printed)" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H1b; then
  t=$(date +%s.%N); d=$(fresh H1b) && {
    printf 'x\n' >"$d/B.$EXT_B"
    git -C "$d" add -f "B.$EXT_B" >/dev/null 2>&1
    hyg "$d"
    result H1b "t0-p5 b2 via t0-p6b L842" \
      "exit 1; FAIL hygiene: 1 office file(s) tracked outside tests/fixtures/ (paths not printed)" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
# H2 plants ONE fixture per sub-case, so each count is the count its source
# recorded, and all THREE forms specs/0007:112 names reach the hygiene step.
# Reducing H2 to the tilde form alone fixed check 1's gap 14 but dropped the
# lock and .bak forms from the hygiene step entirely, leaving them covered only through
# privacy.sh as V17a and V17c (independent check 2, T6 P1, gap 7). The count-only
# wording and the count 1 come from t0-p6c i L843; the three forms come from
# t0-p6c a1, a2 and a3, which planted them one at a time in their own clones.
if want H2a || want H2b || want H2c; then
  for sub in a b c; do
    case "$sub" in
      a) nm=".~lock.n.$EXT_E#"; src="t0-p6c a1 + i L843" ;;
      b) nm="n.$EXT_A~";        src="t0-p6c a2 + i L843" ;;
      c) nm="n.$EXT_A.bak";     src="t0-p6c a3 + i L843" ;;
    esac
    want "H2$sub" || continue
    t=$(date +%s.%N); d=$(fresh "H2$sub") && {
      printf 'x\n' >"$d/$nm"
      git -C "$d" add -f . >/dev/null 2>&1
      hyg "$d"
      result "H2$sub" "$src" \
        "exit 1; FAIL hygiene: 1 office file(s) tracked outside tests/fixtures/ (paths not printed)" \
        "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
    }
  done
fi
# H3's source is this phase's own negative log. specs/0007 names T0 P6c b, but
# that case recorded a privacy "FAIL paths" line, not a hygiene line, and no
# log anywhere records a csv or a tsv at the hygiene step; its only other authority would
# be OFFICE_RE itself, the thing under test. Planning's ruling (2026-09-16) was
# to establish the expectation in negative.log first and source this case to
# that log by case name. N7 planted the same two fixtures, staged 2 paths, and
# recorded the line below with exit 1 and exactly one FAIL line. The count 2 is
# a measurement from a case that ran, not arithmetic over a fixture.
if want H3; then
  t=$(date +%s.%N); d=$(fresh H3) && {
    printf 'x\n' >"$d/data.$EXT_C"; printf 'x\n' >"$d/data.$EXT_D"
    git -C "$d" add -f . >/dev/null 2>&1
    hyg "$d"
    result H3 "t6-p1 negative N7 (established this phase)" \
      "exit 1; FAIL hygiene: 2 office file(s) tracked outside tests/fixtures/ (paths not printed)" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H4a; then
  t=$(date +%s.%N); d=$(fresh H4a) && {
    # The base name must NOT be an office file: OFFICE_RE matches an office
    # extension followed by ":", and check.sh:416 runs the office check before
    # the Zone check at :424, so an office base name stops the run on the
    # office rule and never reaches ZONE_RE. That was this case's first
    # fixture, and the failure was the fixture, not the gate.
    printf 'x\n' >"$d/r.txt:$ZONE"
    git -C "$d" add -f . >/dev/null 2>&1
    hyg "$d"
    result H4a "t2-p0 g L837 (same wording as H4b, root path)" \
      "exit 1; FAIL hygiene: 1 tracked Zone.Identifier file(s) (paths not printed)" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H4b; then
  t=$(date +%s.%N); d=$(fresh H4b) && {
    mkdir -p "$d/tests/fixtures"; printf 'x\n' >"$d/tests/fixtures/r.$EXT_A:$ZONE"
    git -C "$d" add -f . >/dev/null 2>&1
    hyg "$d"
    result H4b "t2-p0 g L837" \
      "exit 1; FAIL hygiene: 1 tracked Zone.Identifier file(s) (paths not printed)" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H5; then
  t=$(date +%s.%N); d=$(fresh H5) && {
    mkdir -p "$d/$CORPUS"; printf 'x\n' >"$d/$CORPUS/x.txt"
    git -C "$d" add -f . >/dev/null 2>&1
    hyg "$d"
    result H5 "t0-p5b d via t0-p6 L706" \
      "exit 1; FAIL hygiene: 1 tracked path(s) containing $CORPUS (paths not printed)" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H6; then
  t=$(date +%s.%N); d=$(fresh H6) && {
    n=$(printf 'bad\001name.txt'); printf 'x\n' >"$d/$n"
    git -C "$d" add -f "$n" >/dev/null 2>&1
    hyg "$d"
    result H6 "t0-p5b b L222" \
      "exit 1; FAIL hygiene: 1 tracked path(s) contain control bytes (paths not printed)" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H7; then
  t=$(date +%s.%N); d=$(fresh H7) && {
    printf 'dash %s here\n' "$EMDASH" >"$d/notes.txt"
    git -C "$d" add notes.txt >/dev/null 2>&1
    rm -f "$d/notes.txt"
    hyg "$d"
    result H7 "t0-p5b a L204" "exit 1; FAIL hygiene: U+2014 in tracked file(s): notes.txt" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H8a; then
  t=$(date +%s.%N); d=$(fresh H8a) && {
    printf 'dash %s here\n' "$EMDASH" >"$SCRATCH/outside.txt"
    ln -s "$SCRATCH/outside.txt" "$d/link-outside"
    git -C "$d" add link-outside >/dev/null 2>&1
    hyg "$d"
    result H8a "t0-p5b c L240" "exit 0; PASS hygiene" "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H8b; then
  t=$(date +%s.%N); d=$(fresh H8b) && {
    ln -s "target-$EMDASH" "$d/link-dangling"
    git -C "$d" add link-dangling >/dev/null 2>&1
    hyg "$d"
    result H8b "t0-p5b h L349" "exit 1; FAIL hygiene: U+2014 in tracked file(s): link-dangling" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H9; then
  t=$(date +%s.%N); d=$(fresh H9) && {
    mkdir -p "$d/tools/x/tests/fixtures"; printf 'x\n' >"$d/tools/x/tests/fixtures/a.$EXT_A"
    git -C "$d" add -f . >/dev/null 2>&1
    hyg "$d"
    result H9 "t0-p6b c L512 (PASS either step)" "exit 0; PASS hygiene" \
      "$(outcome "$HOUT" "$HRC")" "$(secs "$t")"
  }
fi
if want H10; then
  t=$(date +%s.%N); d=$(fresh H10) && {
    o=$d/key.out
    ( cd "$d" && ANTHROPIC_API_KEY=x ./scripts/check.sh --only hygiene ) >"$o" 2>&1
    rc=$?
    n=$(count_lines "$o")
    result H10 "t0-p5b e L267,L274" "exit 1; FAIL hygiene: ANTHROPIC_API_KEY is set in the environment; lines 1" \
      "$(outcome "$o" "$rc"); lines $n" "$(secs "$t")"
  }
fi
echo

echo "## privacy cases, through scripts/privacy.sh with a scratch denylist"
if want V0; then
  t=$(date +%s.%N); d=$(fresh V0) && {
    b=$(git -C "$d" rev-parse HEAD); priv "$d" "$(deny_for "$d")" "$b"
    result V0 "control, unchanged clone" "exit 0; PASS privacy" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V1; then
  t=$(date +%s.%N); d=$(fresh V1) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x\n' >"$d/v1.txt"; git -C "$d" add -A >/dev/null 2>&1
    other="zq$(printf '%s' 1)@$(printf 'exam%s' 'ple.com')"
    git -C "$d" -c "user.email=$other" commit -q -m "v1"
    priv "$d" "$(deny_for "$d")" "$b"
    result V1 "t0-p6 a L490" "exit 1; FAIL identity: commit <sha> author email differs from scripts/privacy-identity.txt" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V2; then
  t=$(date +%s.%N); d=$(fresh V2) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x\n' >"$d/v2.txt"
    git -C "$d" add -A >/dev/null 2>&1
    git -C "$d" commit -q -m "$(printf 'v2\n\nClaude-Session: control\n')"
    priv "$d" "$(deny_for "$d")" "$b"
    result V2 "t0-p6 b L502" "exit 1; FAIL message: commit <sha> has a Claude-Session line" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V3; then
  t=$(date +%s.%N); d=$(fresh V3) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf '%s\n' "$SENTINEL" >"$d/c.txt"; commit_in "$d" "v3 add"
    first=$(git -C "$d" rev-parse HEAD)
    rm -f "$d/c.txt"; commit_in "$d" "v3 remove"
    priv "$d" "$(deny_for "$d")" "$b"
    # The FAIL lines are compared through all_fails(), not counted in the note.
    # result() prints the note without examining it, so "first commit named 1
    # time(s)" looked like an assertion and was not (independent check 2, gap 9).
    result V3 "t0-p6 c L514" "FAIL denylist: commit <sha> path c.txt denylist entry 1 line 1" \
      "$(all_fails "$POUT")" "$PINV, exit $PRC, $(secs "$t")"
  }
fi
if want V4; then
  t=$(date +%s.%N); d=$(fresh V4) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'first\nx %s%s%s y\n' 'gh' 'p_' "$(printf 'a%.0s' $(seq 1 24))" >"$d/d.txt"
    commit_in "$d" "v4"
    priv "$d" "$(deny_for "$d")" "$b"
    result V4 "t0-p6 d L528" "exit 1; FAIL patterns: commit <sha> path d.txt pattern 1 line 2" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V5; then
  t=$(date +%s.%N); d=$(fresh V5) && {
    b=$(git -C "$d" rev-parse HEAD)
    { printf 'x %s%s%s%s y\n' '/mnt/' 'c/' 'Users/' 'bob'
      printf 'x C%s%sUsers%sbob y\n' ':' '\' '\'
      printf 'x %s%s y\n' '.claude' '/projects/x'; } >"$d/h.txt"
    commit_in "$d" "v5"
    priv "$d" "$(deny_for "$d")" "$b"
    # All three FAIL lines are compared. outcome() takes only the first, because
    # of -m 1, so V5 asserted one line where its source recorded three and the
    # "FAIL lines 3" rode in the uncompared note: if the gate stopped reporting
    # patterns 10 and 11, V5 would still have read as-expected (independent
    # check 2, T6 P1, gap 9). t0-p6d a recorded all three at L352-354, as
    # patterns 8, 10 and 11 on lines 1, 2 and 3 of the fixture. The path differs
    # from the source's: t0-p6d a wrote real.txt and this case writes h.txt, so
    # the path in the expectation is this fixture's and the rest is the
    # source's, word for word.
    result V5 "t0-p6d a L352-354" \
      "FAIL patterns: commit <sha> path h.txt pattern 8 line 1 + FAIL patterns: commit <sha> path h.txt pattern 10 line 2 + FAIL patterns: commit <sha> path h.txt pattern 11 line 3" \
      "$(all_fails "$POUT")" "$PINV, exit $PRC, $(secs "$t")"
  }
fi
if want V6; then
  t=$(date +%s.%N); d=$(fresh V6) && {
    b=$(git -C "$d" rev-parse HEAD)
    { printf 'x %s%s%s y\n' '/mnt/' 'c/' 'Users/'
      printf 'x C%s%sUsers%s y\n' ':' '\' '\'
      printf 'x %s%s y\n' '.claude' '/projects/'; } >"$d/bare.txt"
    commit_in "$d" "v6"
    priv "$d" "$(deny_for "$d")" "$b"
    result V6 "t0-p6d b L361-365" "exit 0; PASS privacy" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V7a; then
  t=$(date +%s.%N); d=$(fresh V7a) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x %s%s%s y\n' 'zq12' '@exam' 'ple.com' >"$d/g1.txt"
    commit_in "$d" "v7a"
    priv "$d" "$(deny_for "$d")" "$b"
    result V7a "t0-p6b g L789" "exit 1; FAIL email: commit <sha> path g1.txt line 1" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V7b; then
  t=$(date +%s.%N); d=$(fresh V7b) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x\n' >"$d/g2.txt"; git -C "$d" add -A >/dev/null 2>&1
    git -C "$d" commit -q -m "$(printf 'v7b\n\nx %s%s%s y\n' 'zq12' '@exam' 'ple.com')"
    priv "$d" "$(deny_for "$d")" "$b"
    result V7b "t0-p6b g2 L800" "exit 1; FAIL message: commit <sha> email line 3" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V7c; then
  t=$(date +%s.%N); d=$(fresh V7c) && {
    b=$(git -C "$d" rev-parse HEAD)
    nm="e-$(printf '%s%s%s' 'zq12' '@exam' 'ple.com').txt"
    printf 'x\n' >"$d/$nm"
    commit_in "$d" "v7c"
    priv "$d" "$(deny_for "$d")" "$b"
    result V7c "t0-p6c e L651, ordinal normalised: it is the entry's position in this repository's root tree, not a property of the gate" "exit 1; FAIL paths: commit <sha> tree entry <n> email (path not printed)" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
# The extra-commit-header half of specs/0007's V7, which had no case. Built the
# way its source built it: the committer line is followed by one more header
# line holding an address, so the address lands on header line 5, which is what
# t0-p6c d recorded. The line number is the source's, not a number chosen to
# make the case pass.
if want V7d; then
  t=$(date +%s.%N); d=$(fresh V7d) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x\n' >"$d/v7d.txt"; commit_in "$d" "v7d"
    addr=$(printf '%s%s%s' 'zq12' '@exam' 'ple.com')
    br=$(git -C "$d" rev-parse --abbrev-ref HEAD)
    git -C "$d" cat-file commit HEAD |
      awk -v x="x-extra-header $addr" '{ print } /^committer / && !done { print x; done = 1 }' \
      >"$SCRATCH/v7d-raw"
    nh=$(git -C "$d" hash-object -t commit -w "$SCRATCH/v7d-raw")
    git -C "$d" update-ref "refs/heads/$br" "$nh"
    priv "$d" "$(deny_for "$d")" "$b"
    result V7d "t0-p6c d L639" "exit 1; FAIL header: commit <sha> email line 5" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V8; then
  t=$(date +%s.%N); d=$(fresh V8) && {
    b=$(git -C "$d" rev-parse HEAD)
    mkdir -p "$d/$CORPUS"
    printf 'x\n' | gzip -c >"$d/$CORPUS/a.$EXT_A"
    commit_in "$d" "v8 add"
    rm -rf "${d:?}/$CORPUS"; commit_in "$d" "v8 remove"
    priv "$d" "$(deny_for "$d")" "$b"
    # ONE log records this whole union: t0-p6c g-a ran the same fixture -- a
    # gzip-compressed corpus office blob, committed then deleted -- after the
    # archives class existed, and recorded all three FAIL lines together with
    # "FAIL lines: count=3 (expect 3)". An earlier version of this case claimed
    # no single log held the union and cited t0-p6b a, which predates the
    # archives class, treating it as a judgement call for planning. That
    # premise was wrong (independent check 1, T6 P1). Only the archives WORDING
    # moved after t0-p6c, at d97b9ef, so the current wording comes from t2-p0 b.
    result V8 "t0-p6c g-a L686-688 + t2-p0 b (archives wording)" \
      "FAIL paths: commit <sha> 1 path(s) containing $CORPUS (paths not printed) + FAIL paths: commit <sha> 1 office file path(s) outside tests/fixtures/ (paths not printed) + FAIL archives: commit <sha> 1 blob(s) with an archive or compound-file signature outside tests/fixtures/ (paths not printed)" \
      "$(all_fails "$POUT")" "$PINV, exit $PRC, $(secs "$t")"
  }
fi
if want V9a; then
  t=$(date +%s.%N); d=$(fresh V9a) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'PK\003\004rest\n' >"$d/notes.txt"
    commit_in "$d" "v9a"
    priv "$d" "$(deny_for "$d")" "$b"
    result V9a "t0-p6c c L590 via t2-p0 b1 L524 (wording)" \
      "exit 1; FAIL archives: commit <sha> 1 blob(s) with an archive or compound-file signature outside tests/fixtures/ (paths not printed)" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
# V9a's citation splits fixture from wording, as V8's does: t0-p6c c L590
# planted the zip signature and recorded the archives class, but in the
# SUPERSEDED wording ("with a zip, gzip or 7z signature"), which d97b9ef
# replaced. The current wording is t2-p0 b1 L524. Citing L590 alone would name a
# line that does not say what the case asserts.
#
# The compound-file half of specs/0007's V9, which had no case: t2-p0 case b1
# added exactly this signature in a file named x.bin and recorded one
# count-only archives line at L524.
if want V9b; then
  t=$(date +%s.%N); d=$(fresh V9b) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf '\320\317\021\340\241\261\032\341rest\n' >"$d/x.bin"
    commit_in "$d" "v9b"
    priv "$d" "$(deny_for "$d")" "$b"
    result V9b "t2-p0 b1 L524" \
      "exit 1; FAIL archives: commit <sha> 1 blob(s) with an archive or compound-file signature outside tests/fixtures/ (paths not printed)" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V10; then
  t=$(date +%s.%N); d=$(fresh V10) && {
    b=$(git -C "$d" rev-parse HEAD)
    mkdir -p "$d/tests/fixtures"; printf 'x\n' >"$d/tests/fixtures/a.$EXT_A"
    commit_in "$d" "v10"
    priv "$d" "$(deny_for "$d")" "$b"
    result V10 "t0-p6b c L512" "exit 0; PASS privacy" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V11missing || want V11mode644 || want V11empty || want V11space; then
  for sub in missing mode644 empty space; do
    want "V11$sub" || continue
    t=$(date +%s.%N); d=$(fresh "V11$sub") && {
      b=$(git -C "$d" rev-parse HEAD)
      f=$d/deny.txt
      case "$sub" in
        missing) rm -f "$f" ;;
        mode644) printf '%s\n' "$SENTINEL" >"$f"; chmod 644 "$f" ;;
        empty)   : >"$f"; chmod 600 "$f" ;;
        space)   printf '%s has space\n' "$SENTINEL" >"$f"; chmod 600 "$f" ;;
      esac
      # Each sub-case asserts its source's ERROR WORDING, not just the exit
      # status. Asserting exit 2 alone would pass against any reworded or even
      # unrelated error (independent check 1, T6 P1).
      case "$sub" in
        missing) exp_line="ERROR privacy: the private denylist is missing"; src="t0-p6 e1 L539" ;;
        mode644) exp_line="ERROR privacy: the private denylist must be mode 600"; src="t0-p6 e2 L548" ;;
        empty)   exp_line="ERROR privacy: the private denylist holds no entries"; src="t0-p6 e3 L557" ;;
        space)   exp_line="ERROR privacy: denylist entry 1 contains whitespace"; src="t0-p6b f L776" ;;
      esac
      POUT=$d/p.out
      # Full history: t0-p6 e1, e2 and e3 each ran "scripts/privacy.sh" with no
      # --range. An earlier version passed --range even in default mode, so
      # these cases did not reproduce their source's invocation while the log
      # header claimed the run did (independent check 2, T6 P1, gap 10).
      ( cd "$d" && EMEND_PRIVATE_DENYLIST="$f" ./scripts/privacy.sh ) >"$POUT" 2>&1
      PRC=$?
      result "V11$sub" "$src" "exit 2; $exp_line" \
        "$(outcome "$POUT" "$PRC")" "$(secs "$t")"
    }
  done
fi
if want V12remove || want V12add; then
  want_n=$(count_matches -E '.' scripts/privacy-patterns.txt)
  for sub in remove add; do
    want "V12$sub" || continue
    t=$(date +%s.%N); d=$(fresh "V12$sub") && {
      b=$(git -C "$d" rev-parse HEAD)
      if [ "$sub" = remove ]; then
        sed -i '1d' "$d/scripts/privacy-patterns.txt"; n=$((want_n - 1))
      else
        printf 'zzzz[q]zzzz\n' >>"$d/scripts/privacy-patterns.txt"; n=$((want_n + 1))
      fi
      POUT=$d/p.out
      # Full history, as t0-p6b e ran it (gap 10).
      ( cd "$d" && EMEND_PRIVATE_DENYLIST="$(deny_for "$d")" ./scripts/privacy.sh ) >"$POUT" 2>&1
      PRC=$?
      result "V12$sub" "t0-p6b e L589,L677 (count read at run time)" \
        "exit 2; ERROR privacy: scripts/privacy-patterns.txt holds $n pattern(s); expected $want_n" \
        "$(outcome "$POUT" "$PRC")" "$(secs "$t")"
    }
  done
fi
if want V13; then
  t=$(date +%s.%N); d=$(fresh V13) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x\n' >"$d/v13.txt"; commit_in "$d" "v13"
    blob=$(git -C "$d" rev-parse HEAD:v13.txt)
    p=$d/.git/objects/${blob:0:2}/${blob:2}
    # The source made the blob unreadable by REMOVING its object file, and the
    # message it recorded names that blob. This case used chmod 000, a
    # different mechanism; asserting the source's wording over a different
    # mechanism would not be the source's case (independent check 1, T6 P1).
    rm -f "$p"
    POUT=$d/p.out
    # Full history, as t0-p6 i ran it (gap 10).
    ( cd "$d" && EMEND_PRIVATE_DENYLIST="$(deny_for "$d")" ./scripts/privacy.sh ) >"$POUT" 2>&1
    PRC=$?
    result V13 "t0-p6 i L675" "exit 2; ERROR privacy: git failed: cat-file blob <sha>" \
      "$(outcome "$POUT" "$PRC")" "$(secs "$t")"
  }
fi
if want V14; then
  t=$(date +%s.%N); d=$SCRATCH/cV14; rm -rf "$d"
  if git clone --quiet --depth 1 "file://$BASE" "$d" 2>/dev/null; then
    git -C "$d" config user.name "$ID_NAME"; git -C "$d" config user.email "$ID_EMAIL"
    POUT=$d/p.out
    ( cd "$d" && EMEND_PRIVATE_DENYLIST="$(deny_for "$d")" ./scripts/privacy.sh ) >"$POUT" 2>&1
    PRC=$?
    result V14 "t0-p6 o L760" "exit 2; ERROR privacy: shallow repository or unreadable repository state; history cannot be checked" \
      "$(outcome "$POUT" "$PRC")" "full history, $(secs "$t")"
  else
    errors=$((errors + 1))
    echo "case V14: could not make a shallow clone; error"
  fi
fi
if want V15; then
  t=$(date +%s.%N); d=$(fresh V15) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x\n' >"$d/v15.txt"; git -C "$d" add -A >/dev/null 2>&1
    git -C "$d" commit -q -m "$(printf 'v15 caf\303\251\n')"
    priv "$d" "$(deny_for "$d")" "$b"
    result V15 "t0-p6 p L770" "exit 1; FAIL message: commit <sha> has non-ASCII bytes" \
      "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
  }
fi
if want V16; then
  t=$(date +%s.%N); d=$(fresh V16) && {
    POUT=$d/p.out
    ( cd "$d" && EMEND_PRIVATE_DENYLIST="$(deny_for "$d")" ./scripts/privacy.sh --range -x ) >"$POUT" 2>&1
    PRC=$?
    result V16 "t0-p6 q L779" "exit 2; usage: scripts/privacy.sh [--range <rev>]" \
      "$(outcome "$POUT" "$PRC")" "$(secs "$t")"
  }
fi
# One fixture per sub-case, each matching the source that planted exactly that
# one and recorded count 1 in its own clone. An earlier version planted all
# four together and expected 4, a count no log records (independent check 1).
if want V17a || want V17b || want V17c || want V17d; then
  for sub in a b c d; do
    case "$sub" in
      a) nm=".~lock.n.$EXT_E#"; src="t0-p6c a1 L516" ;;
      b) nm="n.$EXT_A~";        src="t0-p6c a2 L533" ;;
      c) nm="n.$EXT_A.bak";     src="t0-p6c a3 L550" ;;
      d) nm="data.$EXT_C";      src="t0-p6c b1 L564" ;;
    esac
    want "V17$sub" || continue
    t=$(date +%s.%N); d=$(fresh "V17$sub") && {
      b=$(git -C "$d" rev-parse HEAD)
      printf 'x\n' >"$d/$nm"
      commit_in "$d" "v17$sub"
      priv "$d" "$(deny_for "$d")" "$b"
      result "V17$sub" "$src" \
        "exit 1; FAIL paths: commit <sha> 1 office file path(s) outside tests/fixtures/ (paths not printed)" \
        "$(outcome "$POUT" "$PRC")" "$PINV, $(secs "$t")"
    }
  done
fi
if want V18; then
  t=$(date +%s.%N); d=$(fresh V18) && {
    b=$(git -C "$d" rev-parse HEAD)
    printf 'x\n' >"$d/r.$EXT_A:$ZONE"
    commit_in "$d" "v18"
    priv "$d" "$(deny_for "$d")" "$b"
    result V18 "t2-p0 a L487-488" \
      "FAIL paths: commit <sha> 1 office file path(s) outside tests/fixtures/ (paths not printed) + FAIL paths: commit <sha> 1 Zone.Identifier path(s) (paths not printed)" \
      "$(all_fails "$POUT")" "$PINV, exit $PRC, $(secs "$t")"
  }
fi
echo

echo "## hook cases"
if want K1; then
  # The source, t0-p6 g, ran the clean push with NO EMEND_PRIVATE_DENYLIST: the
  # hook uses the real denylist, and it reported PASS privacy over the whole
  # local sha. My first version exported the variable for both pushes, so the
  # clean push hit the hook's refusal of that variable at install-hooks.sh
  # line 9, which is the thing K2 exists to prove, and never reached the scan.
  # That was a harness fault, not a gate finding.
  #
  # THE BLOCKED HALF USES A PATTERN HIT, NOT THE REAL DENYLIST. An earlier
  # version read the first entry of $HOME/.config/emend/private-denylist.txt
  # into a variable and committed it into a scratch clone. It was never printed
  # and nothing leaked, but it put private data into a git object in a
  # mktemp -d tree disposed of only by a trap, in a committed tool that runs
  # many times (independent check 2, T6 P1). Planning's ruling, 2026-09-16: the
  # self-test never reads the private denylist, by any path. negative.log
  # greps this committed script for that path and expects 0.
  #
  # The source, t0-p6 g, blocked its push with a scratch denylist supplied
  # through EMEND_PRIVATE_DENYLIST. The hook has refused that variable since
  # P6b (install-hooks.sh line 9, which is what K2 proves), so that route is no
  # longer available to any push. The faithful equivalent is the other class
  # the hook blocks on: a pattern hit, assembled at run time exactly as V4
  # assembles one, whose wording t0-p6 d recorded at L528.
  t=$(date +%s.%N); d=$(fresh K1) && {
    bare=$SCRATCH/K1.git; rm -rf "$bare"; git init -q --bare "$bare"
    o=$d/k1.out
    ( cd "$d" && ./scripts/install-hooks.sh ) >"$o" 2>&1
    irc=$?
    git -C "$d" remote add scratch "$bare" 2>/dev/null
    git -C "$d" checkout -q -b clean
    printf 'x\n' >"$d/k1.txt"; commit_in "$d" "k1 clean"
    o2=$d/k1-push.out
    ( cd "$d" && git push scratch clean ) >"$o2" 2>&1
    prc=$?
    entry="$(printf '%s%s%s' 'gh' 'p_' "$(printf 'a%.0s' $(seq 1 24))")"
    git -C "$d" checkout -q -b dirty
    printf 'x %s y\n' "$entry" >"$d/c.txt"; commit_in "$d" "k1 blocked"
    o3=$d/k1-block.out
    ( cd "$d" && git push scratch dirty ) >"$o3" 2>&1
    brc=$?
    git -C "$bare" for-each-ref --format='%(refname)' >"$d/k1-refs" 2>/dev/null || : >"$d/k1-refs"
    refs=$(count_lines "$d/k1-refs")
    pass=$(count_matches -F 'PASS privacy' "$o2")
    deny=$(count_matches -E '^FAIL patterns: ' "$o3")
    leak=$(count_matches -F "$entry" "$o2")
    leak2=$(count_matches -F "$entry" "$o3")
    # NOT $((leak + leak2)). count_matches prints the word "error" when it
    # cannot read a file, and bash evaluates an unset name inside $(( )) as 0 --
    # which is exactly the expected value here. An unreadable push output would
    # therefore have reported the safe answer, and this case, the one guarding a
    # real denylist entry, could not have failed (independent check 1, T6 P1).
    # The counts are compared as strings, so "error" is a mismatch.
    result K1 "t0-p6 g L578-L630 structure + t0-p6 d L528 (pattern class; the source's denylist route is refused by the hook since P6b)" \
      "install 0; clean push 0 with PASS privacy 1; blocked push 1 with FAIL patterns 1; remote refs 1; fixture printed 0 and 0" \
      "install $irc; clean push $prc with PASS privacy $pass; blocked push $brc with FAIL patterns $deny; remote refs $refs; fixture printed $leak and $leak2" \
      "$(secs "$t")"
  }
fi
if want K2; then
  t=$(date +%s.%N); d=$(fresh K2) && {
    bare=$SCRATCH/K2.git; rm -rf "$bare"; git init -q --bare "$bare"
    ( cd "$d" && ./scripts/install-hooks.sh ) >/dev/null 2>&1
    git -C "$d" remote add scratch "$bare" 2>/dev/null
    git -C "$d" checkout -q -b k2
    printf 'x\n' >"$d/k2.txt"; commit_in "$d" "k2"
    o=$d/k2.out
    ( cd "$d" && EMEND_PRIVATE_DENYLIST="$(deny_for "$d")" git push scratch k2 ) >"$o" 2>&1
    rc=$?
    n=$(count_matches -F 'FAIL privacy: EMEND_PRIVATE_DENYLIST is set' "$o")
    result K2 "t0-p6b d3 L555" "exit 1; refusal line 1" "exit $rc; refusal line $n" "$(secs "$t")"
  }
fi
# K3's source is this phase's own negative log. specs/0007 names T0 P6b d, but
# that case recorded d1, d2 and d3 and holds no hooksPath case: no log in this
# project records this refusal, because T6 P1 N8 is the FIRST time it has ever
# been exercised. Planning's ruling (2026-09-16) was to establish it in
# negative.log first and source this case to that log by case name, rather than
# assert it against the script text under test, where it could not fail by
# construction. N8 recorded the line below with exit 1 and exactly one line.
if want K3; then
  t=$(date +%s.%N); d=$(fresh K3) && {
    git -C "$d" config core.hooksPath .githooks
    o=$d/k3.out
    ( cd "$d" && ./scripts/install-hooks.sh ) >"$o" 2>&1
    rc=$?
    n=$(count_lines "$o")
    result K3 "t6-p1 negative N8 (first exercise of this refusal)" \
      "exit 1; install-hooks: core.hooksPath is set to '.githooks'; git would not run .git/hooks/pre-push; lines 1" \
      "$(outcome "$o" "$rc"); lines $n" "$(secs "$t")"
  }
fi
echo

echo "## build-level cases"
if want B1; then
  t=$(date +%s.%N); d=$(fresh B1) && {
    f=$(git -C "$d" ls-files '*.rs' | head -n 1)
    if [ -n "$f" ]; then
      printf '\nfn  self_test_misformat( ) ->i32{1}\n' >>"$d/$f"
      o=$d/b1.out
      ( cd "$d" && ./scripts/check.sh --only fmt ) >"$o" 2>&1
      rc=$?
      # Asserts the source's wording, not just that some FAIL fmt line exists.
      # Strengthening this from "FAIL fmt line 1" immediately exposed a wording
      # drift that the weak assertion had been hiding: t0-p3 a recorded
      # 'cargo fmt --all --check', and the gate gained "-- --color never" after
      # it. The current wording IS recorded, in a later log, so principle 2
      # applies exactly as it does for V8: the latest log rules and the case
      # names both. t0-p3/negative.log holds only the older wording.
      result B1 "t0-p3 a L100 via t0-p4 L319 (colour off, added after)" \
        "exit 1; FAIL fmt: 'cargo fmt --all --check -- --color never' exited non-zero" \
        "$(outcome "$o" "$rc")" "$(secs "$t")"
    else
      errors=$((errors + 1)); echo "case B1: no rust file found; error"
    fi
  }
fi
echo

echo "## captured outputs"
# specs/0007: "captured outputs are counted for control bytes, non-ASCII bytes
# and the sentinel, and the counts printed." This was not implemented until
# independent check 1 asked for it. The sentinel count is the one that matters:
# it is what proves the scratch denylist value never reached a file this log
# could quote. The sweep takes every capture under the scratch directory, a
# superset of the per-case ones, which is the safe direction for a leak guard.
find "$SCRATCH" -type f -name '*.out' -print >"$SCRATCH/captures" 2>/dev/null ||
  : >"$SCRATCH/captures"
ncap=$(count_lines "$SCRATCH/captures")
echo "  captured output files: $ncap"
cap_ctl=error; cap_na=error; cap_sent=error
if [ "$ncap" != error ] && [ "$ncap" -gt 0 ]; then
  CAPS=()
  while IFS= read -r line; do CAPS+=("$line"); done <"$SCRATCH/captures"
  cap_ctl=$(count_bytes "$CTRL_CLASS" "${CAPS[@]}")
  cap_na=$(count_bytes "$NONASCII_CLASS" "${CAPS[@]}")
  cap_sent=$(count_matches -F "$SENTINEL" "${CAPS[@]}")
fi
echo "  control bytes $cap_ctl, non-ASCII bytes $cap_na, sentinel $cap_sent (expect 0, 0 and 0)"
if [ "$cap_ctl" != 0 ] || [ "$cap_na" != 0 ] || [ "$cap_sent" != 0 ]; then
  errors=$((errors + 1))
  echo "  UNEXPECTED: a captured output is not clean"
fi
echo

echo "## totals"
nfail=$(count_lines "$CLONE_FAILURES")
want_cases=$(expected_cases)
echo "  clone failures: $nfail (expect 0)"
echo "  cases performed: $cases; cases selected: $want_cases"
if [ "$nfail" != 0 ]; then
  errors=$((errors + 1))
  echo "  UNEXPECTED: $nfail case(s) could not be cloned"
fi
if [ "$cases" -ne "$want_cases" ]; then
  errors=$((errors + 1))
  echo "  UNEXPECTED: $((want_cases - cases)) selected case(s) did not run"
fi
echo "  wall time: $(secs "$RUN_START")"
echo "run ended at $(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf 'self-test: %s cases, %s unexpected, %s errors\n' "$cases" "$unexpected" "$errors"
if [ "$unexpected" -eq 0 ] && [ "$errors" -eq 0 ] && [ "$counting_controls_ok" = yes ]; then
  echo "RESULT: PASS"
  exit 0
fi
echo "RESULT: FAIL"
exit 1
