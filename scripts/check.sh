#!/usr/bin/env bash
# emend gate. Prints one "PASS <step>" or "FAIL <step>: <reason>" line per
# step and exits non-zero at the first failure. Runs from any directory.
# --quick skips the i686 and clippy steps, for local iteration only; logs
# handed to planning come from a full run.
set -euo pipefail

QUICK=0
for arg in "$@"; do
  case "$arg" in
    --quick) QUICK=1 ;;
    *)
      echo "usage: scripts/check.sh [--quick]" >&2
      exit 2
      ;;
  esac
done

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# A missing toolchain or target must fail, never download (specs/0001).
export RUSTUP_AUTO_INSTALL=0
# Logs carry no colour or other control bytes (specs/0001).
export CARGO_TERM_COLOR=never

# ANTHROPIC_API_KEY is checked before any step, so a key never reaches a
# build script or test. The value is never printed.
if [ -n "${ANTHROPIC_API_KEY+set}" ]; then
  echo "FAIL hygiene: ANTHROPIC_API_KEY is set in the environment"
  exit 1
fi

readonly I686=i686-unknown-linux-musl
readonly SYS_ALLOWLIST=scripts/sys-allowlist.txt
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT

# fail <step> <reason> [output file]: print the FAIL line, show the tail of
# the command output on stderr, and stop.
fail() {
  echo "FAIL $1: $2"
  if [ -n "${3:-}" ] && [ -s "$3" ]; then
    tail -n 60 "$3" | sed 's/^/    /' >&2
  fi
  exit 1
}

# quiet <step> <output file> <command...>: run the command with its output
# captured, and fail the step if it exits non-zero.
quiet() {
  local step=$1 out=$2
  shift 2
  "$@" >"$out" 2>&1 || fail "$step" "'$*' exited non-zero" "$out"
}

# executed_tests <cargo test output>: print how many tests ran (passed plus
# failed, summed over every "test result:" line). Exits 2 on a grep error.
executed_tests() {
  local lines rc=0
  lines=$(LC_ALL=C /usr/bin/grep -E '^test result: ' "$1") || rc=$?
  if [ "$rc" -ge 2 ]; then
    return 2
  fi
  printf '%s\n' "$lines" |
    awk '{ for (i = 1; i < NF; i++) if ($(i + 1) ~ /^(passed|failed);$/) n += $i } END { print n + 0 }'
}

quiet host "$OUT/host-build" cargo build --workspace --all-targets
quiet host "$OUT/host-test" cargo test --workspace
echo "PASS host"

if [ "$QUICK" -eq 1 ]; then
  echo "SKIP i686: --quick"
else
  quiet i686 "$OUT/i686-build" cargo build --workspace --all-targets --target "$I686"
  quiet i686 "$OUT/i686-test" cargo test --workspace --target "$I686"
  if ! ran=$(executed_tests "$OUT/i686-test"); then
    fail i686 "could not read the test output" "$OUT/i686-test"
  fi
  if [ "$ran" -eq 0 ]; then
    fail i686 "zero tests executed on $I686" "$OUT/i686-test"
  fi
  echo "PASS i686"
fi

quiet fmt "$OUT/fmt" cargo fmt --all --check -- --color never
echo "PASS fmt"

if [ "$QUICK" -eq 1 ]; then
  echo "SKIP clippy: --quick"
else
  quiet clippy "$OUT/clippy" cargo clippy --workspace --all-targets -- -D warnings
  echo "PASS clippy"
fi

# Step 5: no C build tooling in the dependency tree, and no -sys or _sys
# crate outside the allowlist. Adding to the allowlist needs a planning
# ruling. --all-features shows crates behind features nothing enables.
[ -f "$SYS_ALLOWLIST" ] || fail native "$SYS_ALLOWLIST is missing"
cargo tree --workspace --target all -e normal,build --all-features --prefix none \
  >"$OUT/tree" 2>"$OUT/tree-err" ||
  fail native "'cargo tree' exited non-zero" "$OUT/tree-err"
names=$(awk 'NF { print $1 }' "$OUT/tree" | sort -u)
banned=$(printf '%s\n' "$names" |
  awk '$0 == "cc" || $0 == "cmake" || $0 == "pkg-config" || $0 == "pkg_config"' | tr '\n' ' ')
if [ -n "$banned" ]; then
  fail native "C build tooling in the dependency tree: ${banned% }" "$OUT/tree"
fi
allowlisted=""
unlisted=""
for crate in $(printf '%s\n' "$names" | awk '/[-_]sys$/'); do
  if awk -v c="$crate" '$0 !~ /^[[:space:]]*(#|$)/ && $1 == c { found = 1 } END { exit !found }' "$SYS_ALLOWLIST"; then
    allowlisted="$allowlisted $crate"
  else
    unlisted="$unlisted $crate"
  fi
done
if [ -n "$unlisted" ]; then
  fail native "-sys or _sys crate not in $SYS_ALLOWLIST:$unlisted" "$OUT/tree"
fi
allowlisted=${allowlisted# }
echo "PASS native (allowlisted -sys: ${allowlisted:-none})"

# Step 6: every package's licence expression, workspace members included,
# passes against the allowed identifiers. Do not widen the list without a
# planning ruling.
cargo metadata --format-version 1 --all-features >"$OUT/metadata.json" 2>"$OUT/metadata-err" ||
  fail licences "'cargo metadata' exited non-zero" "$OUT/metadata-err"
if ! python3 - "$OUT/metadata.json" >"$OUT/licences" 2>&1 <<'PY'; then
import json
import re
import sys

ALLOWED = {
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "Zlib",
    "Unicode-3.0",
    "Unicode-DFS-2016",
}
OPERATORS = {"(", ")", "AND", "OR", "WITH"}


def passes(expr):
    """Evaluate an SPDX expression; raise ValueError on bad syntax."""
    toks = re.findall(r"\(|\)|[^\s()]+", expr)
    pos = 0

    def peek():
        return toks[pos] if pos < len(toks) else None

    def take():
        nonlocal pos
        if pos >= len(toks):
            raise ValueError("unexpected end")
        pos += 1
        return toks[pos - 1]

    def any_of():
        ok = all_of()
        while peek() == "OR":
            take()
            right = all_of()
            ok = ok or right
        return ok

    def all_of():
        ok = term()
        while peek() == "AND":
            take()
            right = term()
            ok = ok and right
        return ok

    def term():
        tok = take()
        if tok == "(":
            ok = any_of()
            if take() != ")":
                raise ValueError("missing )")
            return ok
        if tok in OPERATORS:
            raise ValueError("unexpected " + tok)
        if peek() == "WITH":
            take()
            exception = take()
            if exception in OPERATORS:
                raise ValueError("bad exception")
            return tok + " WITH " + exception in ALLOWED
        return tok in ALLOWED

    result = any_of()
    if pos != len(toks):
        raise ValueError("unexpected " + toks[pos])
    return result


packages = json.load(open(sys.argv[1]))["packages"]
failures = []
for pkg in sorted(packages, key=lambda p: (p["name"], p["version"])):
    label = pkg["name"] + " " + pkg["version"]
    expr = pkg.get("license")
    if not expr:
        verdict, reason = "NOT allowed", label + " (no licence)"
    else:
        try:
            ok = passes(expr)
            verdict, reason = ("allowed", None) if ok else ("NOT allowed", label + " (" + expr + ")")
        except ValueError as err:
            verdict, reason = "NOT allowed", label + " (" + expr + ": " + str(err) + ")"
    print(label + ": " + (expr or "<none>") + ": " + verdict)
    if reason:
        failures.append(reason)
if failures:
    print("; ".join(failures))
    sys.exit(1)
PY
  fail licences "$(tail -n 1 "$OUT/licences")" "$OUT/licences"
fi
echo "PASS licences"

# Step 7: repository hygiene. It reads what would be committed (the index),
# never follows a symlink, and runs a positive control for every grep form
# before using it. Exit 2 or more from any grep is an error, never zero hits.
# ANTHROPIC_API_KEY is checked before step 1.
EMDASH=$(printf '\342\200\224')
readonly OFFICE_RE='\.(xlsx|xlsm|xltx|xltm|xlsb|xls|ods)$'
readonly FIXTURES_RE='(^|/)tests/fixtures/'
: >"$OUT/hygiene-err"
rc=0

# hgrep <output file> <grep args...>: /usr/bin/grep under LC_ALL=C; sets rc.
hgrep() {
  local out=$1
  shift
  rc=0
  LC_ALL=C /usr/bin/grep "$@" >"$out" 2>>"$OUT/hygiene-err" || rc=$?
}

# index_dash <repo dir> <output file>: NUL-separated paths in the index whose
# content holds the U+2014 bytes. Text files come from git grep --cached -I,
# so binary files are skipped. git grep skips symlinks, so each symlink's
# blob (its target string) is checked here; the link is never followed.
# Sets rc to 0 (found), 1 (none) or 2 (error).
index_dash() {
  local repo=$1 out=$2 entry meta path sha lrc
  rc=0
  (cd "$repo" && LC_ALL=C git grep --cached --no-color -I -l -z -F -e "$EMDASH") \
    >"$out" 2>>"$OUT/hygiene-err" || rc=$?
  if [ "$rc" -ge 2 ]; then
    return 0
  fi
  if ! git -C "$repo" ls-files -s -z >"$OUT/stage.z" 2>>"$OUT/hygiene-err"; then
    rc=2
    return 0
  fi
  while IFS= read -r -d '' entry; do
    meta=${entry%%$'\t'*}
    path=${entry#*$'\t'}
    if [ "${meta%% *}" != 120000 ]; then
      continue
    fi
    sha=$(printf '%s\n' "$meta" | awk '{ print $2 }')
    if ! git -C "$repo" cat-file blob "$sha" >"$OUT/link-target" 2>>"$OUT/hygiene-err"; then
      rc=2
      return 0
    fi
    lrc=0
    LC_ALL=C /usr/bin/grep -q -F -e "$EMDASH" "$OUT/link-target" 2>>"$OUT/hygiene-err" || lrc=$?
    if [ "$lrc" -ge 2 ]; then
      rc=2
      return 0
    fi
    if [ "$lrc" -eq 0 ]; then
      printf '%s\0' "$path" >>"$out"
    fi
  done <"$OUT/stage.z"
  if [ -s "$out" ]; then rc=0; else rc=1; fi
}

# quoted <NUL-separated path file>: the paths as git ls-files prints them
# without -z, so non-ASCII bytes appear quoted; space-separated.
quoted() {
  local p list=""
  while IFS= read -r -d '' p; do
    list="$list $(git -c core.quotepath=true ls-files -- ":(literal)$p")"
  done <"$1"
  printf '%s' "${list# }"
}

# Positive controls, one per grep form, before any real check.
CTL=$OUT/controls
mkdir -p "$CTL"
printf 'plain\000bad\001path\000' >"$CTL/ctrl.z"
hgrep "$CTL/o-ctrl" -z -c -P '[\x01-\x1f\x7f]' "$CTL/ctrl.z"
[ "$rc" -eq 0 ] || fail hygiene "positive control failed: control byte in a path (grep exit $rc)" "$OUT/hygiene-err"
printf 'x/b.XLSX\000c.txt\000' >"$CTL/office.z"
hgrep "$CTL/o-office" -z -i -E "$OFFICE_RE" "$CTL/office.z"
[ "$rc" -eq 0 ] || fail hygiene "positive control failed: office extension (grep exit $rc)" "$OUT/hygiene-err"
printf 'tools/tests/fixtures/a.xlsx\000b.xlsx\000' >"$CTL/fixtures.z"
hgrep "$CTL/o-fixtures" -z -v -E "$FIXTURES_RE" "$CTL/fixtures.z"
if [ "$rc" -ne 0 ] || [ "$(tr '\000' '\n' <"$CTL/o-fixtures")" != b.xlsx ]; then
  fail hygiene "positive control failed: tests/fixtures/ exception (grep exit $rc)" "$OUT/hygiene-err"
fi
printf 'docs/Emend-Testing/x.txt\000' >"$CTL/corpus.z"
hgrep "$CTL/o-corpus" -z -i -F -e emend-testing "$CTL/corpus.z"
[ "$rc" -eq 0 ] || fail hygiene "positive control failed: emend-testing in any case (grep exit $rc)" "$OUT/hygiene-err"
git init -q "$CTL/repo" 2>>"$OUT/hygiene-err" ||
  fail hygiene "could not create the control repository" "$OUT/hygiene-err"
printf 'dash %s here\n' "$EMDASH" >"$CTL/repo/text.txt"
ln -s "target-$EMDASH" "$CTL/repo/link"
git -C "$CTL/repo" add text.txt link 2>>"$OUT/hygiene-err" ||
  fail hygiene "could not stage the control repository" "$OUT/hygiene-err"
rm "$CTL/repo/text.txt"
index_dash "$CTL/repo" "$CTL/o-index"
if [ "$rc" -ne 0 ] || [ "$(tr '\000' ' ' <"$CTL/o-index")" != "text.txt link " ]; then
  fail hygiene "positive control failed: U+2014 in the index and in a symlink target (exit $rc)" "$OUT/hygiene-err"
fi

git ls-files -z >"$OUT/tracked.z" 2>>"$OUT/hygiene-err" ||
  fail hygiene "'git ls-files' exited non-zero" "$OUT/hygiene-err"

# Tracked paths holding a control byte, newline included, fail before any
# other path check. Only their number is reported.
hgrep "$OUT/ctrl-count" -z -c -P '[\x01-\x1f\x7f]' "$OUT/tracked.z"
[ "$rc" -lt 2 ] || fail hygiene "grep error while checking tracked paths for control bytes (exit $rc)" "$OUT/hygiene-err"
if [ "$rc" -eq 0 ]; then
  fail hygiene "$(tr -d '\000\n' <"$OUT/ctrl-count") tracked path(s) contain control bytes (paths not printed)"
fi

hgrep "$OUT/office-all.z" -z -i -E "$OFFICE_RE" "$OUT/tracked.z"
[ "$rc" -lt 2 ] || fail hygiene "grep error while matching office extensions (exit $rc)" "$OUT/hygiene-err"
hgrep "$OUT/office.z" -z -v -E "$FIXTURES_RE" "$OUT/office-all.z"
[ "$rc" -lt 2 ] || fail hygiene "grep error while applying the tests/fixtures/ exception (exit $rc)" "$OUT/hygiene-err"
if [ -s "$OUT/office.z" ]; then
  fail hygiene "office file(s) tracked outside tests/fixtures/: $(quoted "$OUT/office.z")"
fi

hgrep "$OUT/corpus.z" -z -i -F -e emend-testing "$OUT/tracked.z"
[ "$rc" -lt 2 ] || fail hygiene "grep error while matching tracked paths (exit $rc)" "$OUT/hygiene-err"
if [ "$rc" -eq 0 ]; then
  fail hygiene "tracked path(s) containing emend-testing: $(quoted "$OUT/corpus.z")"
fi

index_dash . "$OUT/dash.z"
[ "$rc" -lt 2 ] || fail hygiene "error while scanning the index for U+2014" "$OUT/hygiene-err"
if [ "$rc" -eq 0 ]; then
  fail hygiene "U+2014 in tracked file(s): $(quoted "$OUT/dash.z")"
fi
echo "PASS hygiene"
