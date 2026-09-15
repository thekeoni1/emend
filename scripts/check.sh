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

readonly I686=i686-unknown-linux-musl
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

quiet fmt "$OUT/fmt" cargo fmt --all --check
echo "PASS fmt"

if [ "$QUICK" -eq 1 ]; then
  echo "SKIP clippy: --quick"
else
  quiet clippy "$OUT/clippy" cargo clippy --workspace --all-targets -- -D warnings
  echo "PASS clippy"
fi
