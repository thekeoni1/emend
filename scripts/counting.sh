# emend shared counting forms (specs/0007-gate-self-test.md, "scripts/counting.sh").
#
# Sourced by scripts/gate-self-test.sh and by the log scripts of later phases,
# which use these instead of carrying their own copy.
#
# WHY THIS FILE EXISTS. In T3 P4 four counting sites compared a bare `grep -c`
# with 0 without reading the exit status, so a read error printed 0 and the
# check passed. Two independent checks found them, and the class was declared
# closed three further times while instances still stood: a helper whose first
# repair read grep's status while `cat` was the failing command, and both
# halves of one function in T2 P7. Planning's remedy is that every counting
# form, including a throwaway one, is a named function that reads the exit
# status and has a positive control, from the start.
#
# Every function prints either a number or the word "error". None of them ever
# prints a matched line, a value, or a path. A caller that compares a result
# with 0 therefore cannot mistake a failed read for an absence.
#
# counting_controls runs every form against a match, a miss and an unreadable
# file, prints one line per control, and sets counting_controls_ok. Callers run
# it before using any form.

G=/usr/bin/grep

# count_matches <-F|-E> <pattern> <file>...: total matching lines across the
# files, or "error" if any file could not be read. The files go to grep
# directly: with `cat file | grep -c`, an unreadable file makes CAT fail while
# grep counts 0 over the empty stream and exits 1, so the read error reads as
# "no hits".
count_matches() {
  local mode=$1 pattern=$2 out rc
  shift 2
  out=$(LC_ALL=C "$G" -a -H -c "$mode" -e "$pattern" -- "$@" 2>/dev/null); rc=$?
  if [ "$rc" -ge 2 ]; then printf 'error'; return; fi
  printf '%s\n' "$out" | awk -F: '{ n += $NF } END { print n + 0 }'
}

# count_from_file <-F|-E> <pattern file> <file>...: the same, with the patterns
# read from a file (grep -f), so a pattern is never printed.
count_from_file() {
  local mode=$1 patterns=$2 out rc
  shift 2
  out=$(LC_ALL=C "$G" -a -H -c "$mode" -f "$patterns" -- "$@" 2>/dev/null); rc=$?
  if [ "$rc" -ge 2 ]; then printf 'error'; return; fi
  printf '%s\n' "$out" | awk -F: '{ n += $NF } END { print n + 0 }'
}

# count_bytes <tr class> <file>...: total bytes of that class across the files,
# or "error" if any file could not be read. Each file is read on its own rather
# than through `cat "$@" | tr`, for the same reason as above.
count_bytes() {
  local class=$1 total=0 f n
  shift
  for f in "$@"; do
    [ -r "$f" ] || { printf 'error'; return; }
    n=$(LC_ALL=C tr -cd "$class" <"$f" | wc -c) || { printf 'error'; return; }
    total=$((total + n))
  done
  printf '%s' "$total"
}

# count_lines <file>...: total lines, or "error" if any file could not be read.
count_lines() {
  local total=0 f n
  for f in "$@"; do
    [ -r "$f" ] || { printf 'error'; return; }
    n=$(wc -l <"$f") || { printf 'error'; return; }
    total=$((total + n))
  done
  printf '%s' "$total"
}

CTRL_CLASS='\000-\010\013\014\016-\037\177'
NONASCII_CLASS='\200-\377'

# counting_controls <label prefix>: run every counting form above against a
# fixture that must match and one that must not, and against an unreadable
# file, printing one line per result. Sets counting_controls_ok to yes or no.
# This is the positive control planning requires for every form, and it also
# proves each form reports "error" rather than 0 when it cannot read.
counting_controls() {
  local d ok=yes r
  d=$(mktemp -d) || { counting_controls_ok=no; return; }
  printf 'alpha\nbeta needle gamma\nneedle\n' >"$d/hit"
  printf 'nothing of interest\n' >"$d/miss"
  printf 'x \033[31mred\033[m\017 y\ncaf\303\251\n' >"$d/bytes"
  printf 'needle\n' >"$d/patterns"
  printf 'unreadable\n' >"$d/locked"
  chmod 000 "$d/locked"

  r=$(count_matches -F needle "$d/hit");        echo "  control count_matches on a file with two matching lines: $r (expect 2)"; [ "$r" = 2 ] || ok=no
  r=$(count_matches -F needle "$d/miss");       echo "  control count_matches on a file with none: $r (expect 0)"; [ "$r" = 0 ] || ok=no
  r=$(count_matches -F needle "$d/hit" "$d/miss"); echo "  control count_matches summed over two files: $r (expect 2)"; [ "$r" = 2 ] || ok=no
  r=$(count_matches -E 'ne+dle' "$d/hit");      echo "  control count_matches with a regular expression: $r (expect 2)"; [ "$r" = 2 ] || ok=no
  r=$(count_matches -F needle "$d/locked");     echo "  control count_matches on an unreadable file: $r (expect error)"; [ "$r" = error ] || ok=no
  r=$(count_matches -F needle "$d/hit" "$d/locked"); echo "  control count_matches on readable plus unreadable: $r (expect error)"; [ "$r" = error ] || ok=no
  r=$(count_from_file -F "$d/patterns" "$d/hit"); echo "  control count_from_file on the same file: $r (expect 2)"; [ "$r" = 2 ] || ok=no
  r=$(count_from_file -F "$d/patterns" "$d/miss"); echo "  control count_from_file on a file with none: $r (expect 0)"; [ "$r" = 0 ] || ok=no
  r=$(count_from_file -F "$d/patterns" "$d/locked"); echo "  control count_from_file on an unreadable file: $r (expect error)"; [ "$r" = error ] || ok=no
  r=$(count_bytes "$CTRL_CLASS" "$d/bytes");    echo "  control count_bytes, control bytes in a file with ESC, ESC and SI: $r (expect 3)"; [ "$r" = 3 ] || ok=no
  r=$(count_bytes "$NONASCII_CLASS" "$d/bytes"); echo "  control count_bytes, non-ASCII bytes in a file with one two-byte character: $r (expect 2)"; [ "$r" = 2 ] || ok=no
  r=$(count_bytes "$CTRL_CLASS" "$d/miss");     echo "  control count_bytes on a clean file: $r (expect 0)"; [ "$r" = 0 ] || ok=no
  r=$(count_bytes "$CTRL_CLASS" "$d/locked");   echo "  control count_bytes on an unreadable file: $r (expect error)"; [ "$r" = error ] || ok=no
  r=$(count_lines "$d/hit");                    echo "  control count_lines on a three-line file: $r (expect 3)"; [ "$r" = 3 ] || ok=no
  r=$(count_lines "$d/locked");                 echo "  control count_lines on an unreadable file: $r (expect error)"; [ "$r" = error ] || ok=no

  chmod 644 "$d/locked"
  rm -rf "$d"
  counting_controls_ok=$ok
  echo "  counting controls: $([ "$ok" = yes ] && echo "all as expected" || echo FAILED)"
}
