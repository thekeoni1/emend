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

# Step 5: no C build tooling in the dependency tree, and no -sys crate
# outside the allowlist. Adding to the allowlist needs a planning ruling.
[ -f "$SYS_ALLOWLIST" ] || fail native "$SYS_ALLOWLIST is missing"
cargo tree --workspace --target all -e normal,build --prefix none \
  >"$OUT/tree" 2>"$OUT/tree-err" ||
  fail native "'cargo tree' exited non-zero" "$OUT/tree-err"
names=$(awk 'NF { print $1 }' "$OUT/tree" | sort -u)
banned=$(printf '%s\n' "$names" | awk '$0 == "cc" || $0 == "cmake" || $0 == "pkg-config"' | tr '\n' ' ')
if [ -n "$banned" ]; then
  fail native "C build tooling in the dependency tree: ${banned% }" "$OUT/tree"
fi
unlisted=""
for crate in $(printf '%s\n' "$names" | awk '/-sys$/'); do
  if ! awk -v c="$crate" '$0 !~ /^[[:space:]]*(#|$)/ && $1 == c { found = 1 } END { exit !found }' "$SYS_ALLOWLIST"; then
    unlisted="$unlisted $crate"
  fi
done
if [ -n "$unlisted" ]; then
  fail native "-sys crate not in $SYS_ALLOWLIST:$unlisted" "$OUT/tree"
fi
echo "PASS native"

# Step 6: every package's licence expression, workspace members included,
# passes against the allowed identifiers. Do not widen the list without a
# planning ruling.
cargo metadata --format-version 1 >"$OUT/metadata.json" 2>"$OUT/metadata-err" ||
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
