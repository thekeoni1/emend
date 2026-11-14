#!/usr/bin/env bash
# emend's minimum Rust version check (specs/0051-msrv.md, "The change
# (P2)"). Reads the crate's rust-version from its manifest and, with that
# toolchain, runs the eight commands of the trial that found it: cargo build
# and cargo test, with no feature and with every feature, on the host and on
# i686-unknown-linux-musl. Prints one "msrv: <command>: exit <n>" line per
# command (with the tests run for a test command), then "PASS msrv <version>"
# or "FAIL msrv: <reason>", and exits non-zero on a failure.
#
# It never installs anything: a toolchain or target that is not installed is
# refused with a message and exit 1. Runs from any directory.
#
# Usage: scripts/msrv.sh [<crate manifest>]
#   <crate manifest> defaults to crates/emend/Cargo.toml; another path is for
#   the check's own controls (a copy naming another version).
set -euo pipefail

readonly USAGE="usage: scripts/msrv.sh [<crate manifest>]"
[ "$#" -le 1 ] || { echo "$USAGE" >&2; exit 2; }
MANIFEST=${1:-}
if [ -n "$MANIFEST" ]; then
  [ -f "$MANIFEST" ] || { echo "FAIL msrv: no manifest at $MANIFEST"; exit 1; }
  MANIFEST=$(cd "$(dirname "$MANIFEST")" && pwd)/$(basename "$MANIFEST")
fi
cd "$(dirname "${BASH_SOURCE[0]}")/.."
MANIFEST=${MANIFEST:-$PWD/crates/emend/Cargo.toml}

# A missing toolchain or target must fail, never download (specs/0001).
export RUSTUP_AUTO_INSTALL=0
export CARGO_TERM_COLOR=never
readonly I686=i686-unknown-linux-musl

fail() {
  echo "FAIL msrv: $1"
  exit 1
}

[ -f "$MANIFEST" ] || fail "no manifest at $MANIFEST"
# The rust-version of the [package] table, which must be written once as a
# quoted version (not inherited from the workspace).
F=$(awk '/^\[/ { inpkg = ($0 == "[package]") }
  inpkg && /^rust-version = "[0-9][0-9.]*"$/ { n++; v = $3 }
  END { gsub(/"/, "", v); print (n == 1 ? v : "") }' "$MANIFEST")
[ -n "$F" ] || fail "the manifest's [package] table has no single quoted rust-version"
echo "msrv: rust-version $F, read from the crate manifest"

if ! rustc "+$F" --version >/dev/null 2>&1; then
  echo "msrv: the toolchain $F is not installed; this script never installs one" >&2
  echo "msrv: install it with: rustup toolchain install $F --profile minimal --target $I686 --no-self-update" >&2
  fail "toolchain $F is not installed"
fi
# Read into a variable first: under pipefail, grep -q leaving at its first
# match could fail a pipeline and report an installed target as missing.
targets=$(rustup target list --installed --toolchain "$F" 2>/dev/null) || targets=""
if ! grep -q -x -F "$I686" <<<"$targets"; then
  echo "msrv: the target $I686 is not installed for $F; this script never installs one" >&2
  fail "target $I686 is not installed for $F"
fi
echo "msrv: $(rustc "+$F" --version)"

# executed_tests <cargo test output>: tests run (passed plus failed over every
# "test result:" line), counted as scripts/check.sh counts them.
executed_tests() {
  { LC_ALL=C /usr/bin/grep -E '^test result: ' "$1" || true; } |
    awk '{ for (i = 1; i < NF; i++) if ($(i + 1) ~ /^(passed|failed);$/) n += $i } END { print n + 0 }'
}

OUT=$(mktemp -d)
trap 'rm -r -- "${OUT:?}"' EXIT
failed=0
# The trial's order: on each target, build then test, each with no feature
# and with every feature.
for tgt in host "$I686"; do
  for cmd in build test; do
    for feat in none all; do
      args=("$cmd" --locked -p emend)
      [ "$feat" = all ] && args+=(--all-features)
      [ "$tgt" = host ] || args+=(--target "$tgt")
      rc=0
      cargo "+$F" "${args[@]}" >"$OUT/cmd.out" 2>&1 || rc=$?
      cat "$OUT/cmd.out"
      line="msrv: cargo +$F ${args[*]}: exit $rc"
      if [ "$cmd" = test ]; then
        n=$(executed_tests "$OUT/cmd.out")
        line="$line, tests run $n"
        if [ "$n" -eq 0 ] && [ "$rc" -eq 0 ]; then rc=1; fi
      fi
      echo "$line"
      [ "$rc" -eq 0 ] || failed=$((failed + 1))
    done
  done
done
[ "$failed" -eq 0 ] || fail "$failed of the eight commands failed or ran no test"
echo "PASS msrv $F"
