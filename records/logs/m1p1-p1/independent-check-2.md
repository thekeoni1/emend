NOTE, added when this answer was committed; nothing below this block is
altered. This check read the code commit e14dbee5f4d535719ad08263dd53ee1faa6dcd81 and the
five logs of that state, and its input table names each by sha256. The
defect it found was repaired in commit cb2b4da2012415b001b3184d475ef22f107bc7b9, and the
logs committed beside this file are reruns at that commit, each header
naming by sha256 the log it replaces. The answer is otherwise verbatim,
disagreements included (CLAUDE.md, "Independent check before reporting";
specs/0008-zip-layer.md, the P1 rulings of 2026-09-17, item 7).

# Milestone 1 part 1 P1: independent check 2

Phase: Milestone 1 part 1 P1, `specs/0008-zip-layer.md`, "P1: open, index,
verbatim write".
Phase commit: e14dbee5f4d535719ad08263dd53ee1faa6dcd81; base
6f4764deb60fd9b3a4159e0107023cacdd62691f.

Inputs, each with its sha256 as read:

| Input | sha256 |
|-------|--------|
| `specs/0008-zip-layer.md` | `a9b96226e3e4d54afe2fff3499044b30552d0d0a34d7e718d505a8e58130179f` |
| `specs/0001-bootstrap.md` | `34d5248bab6a647db66c551697d9dbf1355304d3e6b6dc955e93056216854a8b` |
| the diff under check (`<scratch>/m1p1-p1-diff.txt`, 2674 lines) | `6fa8fc0a713152483b261171585f1f555a6d67fbbc4b60d59b24cff6d87f31f9` |
| the draft report (`<scratch>/m1p1-p1-report-draft.md`) | `a34f14d8d9ad9e1615e2bce9b6c19a36224b960f34c0f47ea62fcb8d003c9ee4` |
| `records/logs/m1p1-p1/gate.log` | `b0b0790dd63413a21db7584209efbc9b0156c495fe85da4fd6d726857ed57d0c` |
| `records/logs/m1p1-p1/test.log` | `cf878917992e46d98c5e0be11fc823e5473a6d849d06910a0414ca07a8049e16` |
| `records/logs/m1p1-p1/negative.log` | `893ef75e00c91ff36e0f27b8d8e2c2d6bf81ba82257020e8914811e10cf02167` |
| `records/logs/m1p1-p1/run.log` | `bb62ae62f8e7810777ed7a43f926f402741227550478571957faea4ee673aa3b` |
| `records/logs/m1p1-p1/scan.log` | `51873e4017c7e85b259d14316085e02c163f6bff12cf0bedce3c74ca80b0ee57` |
| `records/m1p1-p1-2026-09-17-e14dbee.md` | `3d9fbf439461b7762a2651e581703f960a941e8301e7f321122617ffefeb1338` |
| `records/m1p1-p1-2026-09-17-e14dbee.txt` | `88bfaaed86de10a96b7d9743e6d210efec0fb76d670621faca1f34a90e3d407a` |

The diff file was regenerated independently as `git diff <base>..HEAD` and
is byte for byte the file supplied (same sha256 as the row above).

Two findings below were not read out of the code but MEASURED, by running
the phase's own `target/debug/emend-copy` (sha256
`a9245431e3b3520a65bf01571431371b647e347dbd5ca434eff0e3128ee93bac`, the
binary `gate.log`, `negative.log` and `run.log` all cite) over archives
this check generated in `<scratch>`. No corpus file was opened or read.

## Q1

Does the diff do what the spec says, and nothing else?

Structurally yes. Against `specs/0008`, "Module `archive`" and "Crate":
`crates/emend` carries `archive::{Archive, Entry, Edits, Error}`; every
accessor the spec lists exists with the type it names (`archive.rs`
lines 165-207, 415-449); every offset, size and count in the index and
the writer is `u64` (`archive.rs` 148-159, 496-520) and each narrowing to
`usize` goes through `usize_of`, which refuses rather than truncates
(`archive.rs` 966-969); copies go through a fixed buffer and never hold
an entry (`archive.rs` 951-961); `write_to` refuses a non-empty `Edits`
(`archive.rs` 459-464); `tools/emend-copy` has no `--replace`
(`main.rs` 22, 58-63); `crates/emend/Cargo.toml` pins `=8.6.0`,
`default-features = false`, `deflate-flate2-zlib-rs`, `publish = false`,
workspace edition and licence, and no dev-dependencies; the workspace
`members` list gains the two new crates and nothing else. The runner
change is the two hunks the ruling allows and no other
(`scripts/passthrough-run.sh` 304 and 320 at HEAD, both now
`"$dir/.out.$LABEL.$ext."*.tmp`).

Two qualifications, one of them serious.

1. The writer does not do what the spec's headline sentence says for one
   input shape, and silently: `specs/0008` line 105-106 and
   `archive.rs` 456-458 both claim "a no-edit round trip is
   byte-identical to the input, prefix and comment included". For a
   prefixed archive whose recorded offsets COUNT the prefix, the prefix
   bytes are dropped with no error and exit 0. Measured (GAP 1). A
   second prefixed shape is refused outright (GAP 2). Neither is covered
   by a test, and the test that looks as if it covers the second one
   builds the layer's own convention into the fixture.

2. "Nothing else" is not quite true of the RANGE, though it may be
   intended. Besides the code commit the range carries planning doc
   edits: `specs/0007-gate-self-test.md` (220 lines changed),
   `specs/0001-bootstrap.md`, `specs/0006-passthrough.md` and
   `ROADMAP.md`. `specs/0008` scopes none of these; they arrive through
   the two doc commits whose earlier parent is the declared base, so
   they are in the range by construction. Planning should confirm the
   0007 rewrite is its own text, since this check cannot tell planning's
   prose from the implementer's.

I found no unrequested behaviour in the layer itself. `Edits`,
`Replacement`, `remove` and `add` exist but are inert, as the spec
directs.

## Q2

Do the logs contain every line the spec requires?

The acceptance list of `specs/0008` "P1" is present and evaluated, not
merely printed: every entry row `identical` with an empty flags column
(`run.log` 512-514 host, 590-592 i686), `files_identical` `yes`,
`diff_exit` 0 (`run.log` 500 and 578), `lo` and `lo_in` `ok`
(`run.log` 501 and 579), per workbook on both targets (`run.log` 515-542
and 593-620), the gate eight PASS with exit 0 and empty stderr
(`gate.log` 573-583), `test.log` on both targets (`test.log` 363 and
504) with the same test names on each (`test.log` 699), and
`negative.log` covering the tool's exit codes and refusals on generated
files only (`negative.log` 604-614, 648-696). The phase rules of
`specs/0001` that bite here are met: the exact script is printed into
each log; every commit id is read from git by a printed command
(`gate.log` 472-477, `scan.log` 529-532); the range is written
`<base>..HEAD` (`scan.log` 535); control bytes are counted over the
sibling logs with a positive control (`scan.log` 498-526, 715-750);
`<scratch>`, `<repo>` and `<home>` substitutions are applied
(`run.log` 54, `negative.log` 49).

Five shortfalls, in descending order:

- `gate.log` 103-105 prints a quotation attributed to `specs/0008` that
  `specs/0008` does not contain, and the report repeats it (GAP 3).
- the scan's pre-P6d bare-shape control, which `specs/0001` requires over
  the phase's files, omits three of the files this phase adds (GAP 5).
- no log records the numbers the report's environment finding calls
  measured (GAP 6).
- `run.log` carries the corpus directory's name in its own printed script
  while asserting the record pair carries none (GAP 7).
- `specs/0001` says the FIRST line of every log names the commit the log
  is about and the HEAD at the time it ran. In all five logs line 1 is
  `command: git rev-parse HEAD` and line 2 is the `about:` line. The
  T6 precedent for check files (`specs/0001`, "Planning rulings on P1",
  question 3) accepts a line-2 placement; whether it extends to logs is
  planning's to say. Not counted as a gap below.

On the spec's P1 test list I checked each item against `tests.rs` by
hand: stored, deflated, descriptor with and without signature, zip64
markers, prefix, archive and entry comments, unknown extra fields in
both headers, file order differing from central order, padding, a
non-ASCII name, a directory entry, an empty archive, and the five
refusals. Every item has at least one test. No item is missing. The one
ignored test belongs to the harness and is named as such
(`test.log` 676-679), which is correct: the phase does not touch
`tools/xlsx-diff`.

## Q3

Does the report's every number and quoted string appear in a primary?

Reproduced from primaries, independently: the three commit statistics
(4 files 440/52, 2 files 47/6, 9 files 1898/4) from `git show --stat`;
the five log byte counts and sha256 values from `wc -c` and `sha256sum`;
the record pair's 5619 and 198280 bytes and both hashes
(`run.log` 632-633); the whole run table, with the pins the report gives
landing exactly on `run.log` 499-514 and 577-592; the four binary hashes
(`gate.log` 587-590, `negative.log` 596-599, `run.log` 459-462); 98
passed, 0 failed, 1 ignored, 28 layer tests (`test.log` 499, 640, 674);
E1 to E8 with their exact stderr lines (`negative.log` 652-692); the
usage string read from the source at run time (`negative.log` 650); the
scan totals and `RESULT: PASS` (`scan.log` 676, 709, 789-793); and the
T3 P4 comparison at `records/logs/t3-p4/run.log` 136 and 142-143.

Four statements do not survive:

- "each asserting the OUTPUT BYTES: yes, 28 tests" (GAP 4).
- "run over every file this phase adds" (GAP 5).
- the environment finding's 19229, 22015 and 478 MB (GAP 6).
- the rule quoted as `specs/0008`'s (GAP 3).

One smaller inaccuracy: the report cites
`records/logs/t3-p4/run.log:136,142-143` for a sentence that also
asserts "no file was byte-identical"; that half is at line 141 of that
log, outside the cited range. The claim is true; the citation is short
by one line.

## GAPS

1. SILENT PREFIX LOSS on a prefixed archive whose recorded offsets count
   the prefix. `archive.rs` 353-355 derives `prefix_len` as
   `central_start - central_offset`; `archive.rs` 489-504 copies
   `[0, prefix_len)` and then jumps to each entry's `span_start`, so any
   bytes between `prefix_len` and the first local header are never
   copied. A self-extracting archive whose producer writes file-absolute
   offsets (a common convention) yields `prefix_len` 0, and the stub is
   dropped. MEASURED with the phase binary: a 218-byte archive came back
   202 bytes, exit 0, empty stderr, output a valid zip. 16 bytes lost
   with no error and no flag. The only prefix test builds the other
   convention (`tests.rs` 152-156, `offsets[index] = out.len() - base`,
   and `tests.rs` 414-418), so nothing in the suite can fail on this.
   This is the input shape the phase's own acceptance sentence claims to
   handle. A caller of the library gets no signal at all; only a
   byte-compare against the input (which the harness does, and which the
   corpus happens not to exercise) would show it.

2. ZIP64 LOCATOR OFFSET TREATED AS ABSOLUTE, and the test cannot catch
   it. `archive.rs` 632 reads the locator's 8-byte field and
   `archive.rs` 641 seeks to it as a file position; `archive.rs` 520
   writes `prefix_len + central_offset + central_size` back into it. A
   zip64 archive behind a prefix whose locator records the offset
   relative to the archive proper is refused. MEASURED: exit 2,
   `error: unsupported zip feature: a zip64 locator without its record`;
   the same archive with an absolute locator round trips byte for byte.
   `tests.rs` 399-411 is named as the case that stops the zip64 and
   prefix corrections cancelling, but `tests.rs` 256 sets
   `record_position = out.len() as u64` AFTER the prefix is already in
   the buffer, so the fixture encodes the implementation's own
   convention. Report lines 184-186 overstate what that test pins. A
   refusal, not corruption, so less severe than GAP 1.

3. A QUOTATION ATTRIBUTED TO A SPEC THAT DOES NOT CONTAIN IT.
   `gate.log` 103-105 prints, as a quotation from `specs/0008`: "If P1
   touches any gate file (.gitignore included), run
   scripts/gate-self-test.sh detached and commit self-test.log with the
   other logs." `specs/0008` (sha256 `a9b96226...`) contains no
   occurrence of "self-test", "gate file" or "gitignore". The rule is a
   PLANNED T6 P2 item (`ROADMAP.md` 118). Report lines 105-108 repeat
   the attribution. The assertion's OUTCOME is sound (0 gate files in
   the diff, `gate.log` 504-505), but a committed log quotes a primary
   that does not say it, which is the failure mode `specs/0001` names
   when it requires a source to be a primary.

4. "EACH ASSERTING THE OUTPUT BYTES" IS FALSE OF 12 OF THE 28 TESTS.
   Report line 253. The nine refusal tests (`tests.rs` 555-617 and
   680-690) assert an error, the index test (`tests.rs` 487-513) and the
   read test (`tests.rs` 515-530) assert the crate's own reading, and
   `a_zip64_archive_has_no_prefix` (`tests.rs` 380-396) asserts
   `prefix_len` and two field values. All are legitimate tests; the
   report's blanket sentence is what is wrong. Separately,
   `tests.rs` 524, 528, 544 and 669 discard the `read_to_end` result
   with `.ok()`, so a read error inside those tests is invisible; the
   value assertion still holds, so this is a weakness, not a failure.

5. THE BARE-SHAPE CONTROL MISSES THREE ADDED FILES. `scan.log` 266-273
   and 776-786 run the three pre-P6d forms over four added sources, the
   modified runner, the four sibling logs and the record pair. The phase
   also ADDS `crates/emend/Cargo.toml`, `tools/emend-copy/Cargo.toml`
   and `specs/0008-zip-layer.md`, none of which is in that list. Report
   line 166 says the control ran "over every file this phase adds". The
   three are covered by the narrowed live patterns (`scan.log` 577-585,
   649-657, 658-666), which `specs/0001` says are blind to a bare shape
   by design, so the gap is exactly the one the control exists to close.

6. THE ENVIRONMENT FINDING'S NUMBERS ARE IN NO PRIMARY. Report lines
   235-246 give 19229 MB available of 22015 and a largest resident
   process of 478 MB, and says "Measured at that moment". A grep of all
   five logs and the record `.md` finds none of those numbers and no
   low-memory note. `specs/0001`, "Rules for every phase": "Nothing is
   called measured unless the log prints the measurement". The related
   rule about a run stopped by the environment is satisfied elsewhere
   (`run.log` 5-10 records the detached start), so only the figures are
   unsupported.

7. `run.log` CARRIES THE CORPUS DIRECTORY NAME. `run.log` 45 prints,
   inside its own script, the shell assignment that defaults the corpus
   directory to a named folder under the home directory, and
   `run.log` 246 prints that same folder name again inside a pattern.
   (Neither is quoted here, for the reason this answer exists.) The
   script asserts that the RECORD PAIR holds no such name
   (`run.log` 242-249) but applies no equivalent test to the log it is
   writing. The scan's denylist form reports 0 on this log
   (`scan.log` 742-750), so the name is not a denylist entry today;
   whether the directory name is private at all is the maintainer's
   call. The asymmetry between what the log demands of the record pair
   and what it demands of itself is the finding.

8. UNTESTED ERROR PATHS IN THE TOOL. `main.rs` 78-82 opens the temporary
   with `create_new(true)`, so a pre-existing temporary of the same name
   fails the run rather than being overwritten, and the cleanup block
   (`main.rs` 86-91) runs only after a successful create, so a file this
   process did not make is never removed. That is right, and no case
   reaches it: `negative.log` exercises the `Create` path once (E8) and
   never reaches `Finish` (`main.rs` 46) or `Rename` (`main.rs` 47), and
   nothing covers input and output naming the same path. `specs/0008`
   does not require these, so this is coverage, not a breach.

9. A 64 KIB BUFFER ON THE STACK, on the 32-bit target that matters.
   `archive.rs` 952, `let mut buffer = [0u8; COPY_BUFFER];` with
   `COPY_BUFFER` 64 KiB (`archive.rs` 81). `copy_exact` is called once
   per entry from `write_all_parts`. A musl thread's default stack is
   far smaller than the main thread's, so a consumer that calls the
   layer from a spawned thread has little margin. Every test runs on a
   test-harness thread and none is deep, so nothing here measures it. A
   heap buffer, or a smaller frame, would remove the question.

10. AN ERROR MESSAGE THAT STATES A FALSE INDEX. `archive.rs` 1002 maps
    the crate's `FileNotFound` to `Error::NoSuchEntry(usize::MAX)`,
    which `archive.rs` 119 renders as "no entry with index
    4294967295" on the 32-bit target. `specs/0008` allows a message to
    carry an index; it does not allow one to invent it. Unreachable
    through `read`, which bounds-checks first (`archive.rs` 438-440), so
    cosmetic today and a trap for part 2.

11. OPEN IS GATED ON THE `zip` CRATE'S ACCEPTANCE. `archive.rs` 400
    constructs `zip::ZipArchive` during `open`, although the module
    header (`archive.rs` 5-7) says the crate is used for one thing only,
    decompressing in `read`. An archive this layer parses but the crate
    rejects cannot even be byte-copied, and `emend-copy` would report
    "a zip feature the zip crate refused". Nothing tests a divergence
    between the two parsers. Deferring the construction to the first
    `read` would make the layer's acceptance its own.

12. SMALLER PARSER OBSERVATIONS, each one line:
    - `archive.rs` 554 requires the end record to finish exactly at end
      of file, so one trailing byte makes a sound archive `NotZip`.
      MEASURED: exit 2, "no end-of-central-directory record: not a zip
      archive". Defensible, but no test or negative case records it.
    - `archive.rs` 894-899 skips rewriting an entry count that already
      holds 0xFFFF, so an archive of exactly 65535 entries keeps a
      marker with no zip64 record behind it. Untested.
    - `archive.rs` 826-827 reports a name or extra field running past
      the end as `Io` ("unexpected end of file"), not `Truncated`,
      unlike the fixed-part check at `archive.rs` 370-372.
    - `archive.rs` 805-814 compares names with `Vec::contains`, which is
      quadratic; harmless at 870 entries, not at 100000.

NOT A FINDING, since the brief asks: the widened runner glob cannot match
anything it should not. Everything the runner writes into the run
directory (`scripts/passthrough-run.sh` 284-290, 313, 358-382) is
`out.`, `time.`, `diff.`, `diff-stderr.`, `stdout.`, `stderr.`,
`lo-reason.`, `lo-in-reason.` or a directory; none begins with a dot and
ends `.tmp`, so neither the tool's own output nor any runner artefact can
be swept. `negative.log` L1 to L4 exercise the new form, the old form,
the empty case and a planted stale file, which is the right set.
