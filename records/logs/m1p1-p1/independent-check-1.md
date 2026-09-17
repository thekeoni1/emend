NOTE, added when this answer was committed; nothing below this block is
altered. This check read the code commit e14dbee5f4d535719ad08263dd53ee1faa6dcd81 and the
five logs of that state, and its input table names each by sha256. The
defect it found was repaired in commit cb2b4da2012415b001b3184d475ef22f107bc7b9, and the
logs committed beside this file are reruns at that commit, each header
naming by sha256 the log it replaces. The answer is otherwise verbatim,
disagreements included (CLAUDE.md, "Independent check before reporting";
specs/0008-zip-layer.md, the P1 rulings of 2026-09-17, item 7).

# Independent check 1: Milestone 1 part 1 P1 (specs/0008-zip-layer.md)

Checker: a fresh subagent with no access to the implementing session's
conversation. Inputs, each with its sha256 as read at check time:

| Input | sha256 |
|-------|--------|
| `<repo>/specs/0008-zip-layer.md` | `a9b96226e3e4d54afe2fff3499044b30552d0d0a34d7e718d505a8e58130179f` |
| `<repo>/specs/0001-bootstrap.md` | `34d5248bab6a647db66c551697d9dbf1355304d3e6b6dc955e93056216854a8b` |
| `<scratch>/m1p1-p1-diff.txt` (2674 lines) | `6fa8fc0a713152483b261171585f1f555a6d67fbbc4b60d59b24cff6d87f31f9` |
| `<scratch>/m1p1-p1-report-draft.md` | `a34f14d8d9ad9e1615e2bce9b6c19a36224b960f34c0f47ea62fcb8d003c9ee4` |
| `<repo>/records/logs/m1p1-p1/gate.log` | `b0b0790dd63413a21db7584209efbc9b0156c495fe85da4fd6d726857ed57d0c` |
| `<repo>/records/logs/m1p1-p1/test.log` | `cf878917992e46d98c5e0be11fc823e5473a6d849d06910a0414ca07a8049e16` |
| `<repo>/records/logs/m1p1-p1/negative.log` | `893ef75e00c91ff36e0f27b8d8e2c2d6bf81ba82257020e8914811e10cf02167` |
| `<repo>/records/logs/m1p1-p1/run.log` | `bb62ae62f8e7810777ed7a43f926f402741227550478571957faea4ee673aa3b` |
| `<repo>/records/logs/m1p1-p1/scan.log` | `51873e4017c7e85b259d14316085e02c163f6bff12cf0bedce3c74ca80b0ee57` |
| `<repo>/records/m1p1-p1-2026-09-17-e14dbee.md` | `3d9fbf439461b7762a2651e581703f960a941e8301e7f321122617ffefeb1338` |
| `<repo>/records/m1p1-p1-2026-09-17-e14dbee.txt` | `88bfaaed86de10a96b7d9743e6d210efec0fb76d670621faca1f34a90e3d407a` |

Line references are to these files. Diff lines are lines of the diff file
itself, not of the sources.

## Q1

Does the diff do what the spec says, and nothing else?

Checked point by point against `specs/0008` "Design", "Phases" and "P1".

The crate (`specs/0008:52-60`), diff 99-120 and 26-43:

- `crates/emend/Cargo.toml`: `publish = false` (diff 110),
  `edition.workspace = true` (108), `license.workspace = true` (109).
  The workspace sets `edition = "2024"` (`<repo>/Cargo.toml`), so the
  spec's "edition 2024" holds.
- `zip = { version = "=8.6.0", default-features = false, features =
  ["deflate-flate2-zlib-rs"] }` (diff 116), exactly as the spec pins it.
- No `[dev-dependencies]` section anywhere in the file (diff 104-120);
  `Cargo.lock` shows `emend` depending on `zip` only and `emend-copy` on
  `emend` only (diff 9-21).
- Added to workspace `members` (diff 34-40), which the spec calls an
  ordinary code-commit edit.

Module `archive` against `specs/0008:62-84`: every named item is present
and its signature matches. `Archive::open` (diff 426), `entries` (541),
`index_of` (548), `comment` (553), `prefix_len` (558), `read(index) ->
Result<impl Read + '_, Error>` (563), `write_to(&mut self, out: impl
Write, edits: Edits)` (585). `Entry::name/method/crc32/compressed_size/
uncompressed_size/local_offset/has_descriptor/is_dir/is_encrypted` (diff
291-333). `Edits::replace/remove/add` and `Replacement = Box<dyn FnOnce
(&mut dyn Write) -> io::Result<()>>` (diff 346-391). `Error` with `Io`,
`NotZip`, `Truncated`, `Unsupported`, `DuplicateName`, `NoSuchEntry`
(diff 214-233), messages carrying at most an index (diff 224-229); the
spec permits but does not require an escaped name, so this is inside it.

`write_to` implements the empty-`Edits` case only: it returns
`Error::Unsupported` for a non-empty `Edits` before doing anything (diff
585-590), pinned by `a_non_empty_edits_is_refused_in_p1` (diff 1818-1827,
test.log:652). The writer follows "What the writer preserves": prefix
first (diff 614-616), spans in file order (618-630), central directory in
the original order with only the local-header offset rewritten, including
the zip64 extra-field case (diff 636-640, 960-1013), end records re-emitted
with only counts, sizes and offsets rewritten (diff 641-653, 1018-1057).

Unit tests: all 28 are byte-built with no library (diff 1138-1150) and
every round-trip test compares output bytes with input bytes (diff
1432-1455). Every case the P1 list names is present: stored (1473),
deflated (1482), descriptor with (1491) and without (1500) signature,
zip64 markers (1509), prefix (1551), archive and entry comments (1558),
unknown extra fields in both headers (1567), file order differing from
central order (1577), padding (1588), non-ASCII name (1595), directory
entry (1603), empty archive (1611), and the refusals: truncated (1705,
reaching `Error::Truncated`), not a zip (1693), duplicate names (1720),
encrypted read (1738), multi-disk (1730).

`tools/emend-copy` without `--replace` (diff 2554-2675): two arguments
only (2619-2622), exit 2 on any error (2584, 2627), `error: ...` on
stderr (2626), temporary `.<name>.emend-copy-<pid>.tmp` (2668-2674),
matching `specs/0008:174-176`.

`scripts/passthrough-run.sh`: exactly two hunks (diff 1862 and 1881),
both globs changed from `.passthrough-`*.tmp` to
`"$dir/.out.$LABEL.$ext."*.tmp` (diff 1877, 1890), comments added, no
other change. This matches the ruling text committed in
`specs/0006` (diff 1962-1983).

Nothing in the diff goes beyond the spec, with these observations:

- The diff also carries planning's doc changes to `ROADMAP.md` (diff
  44-98), `specs/0001` (1894-1925), `specs/0006` (1926-1986) and
  `specs/0007` (1987-2290), plus `specs/0008` itself (2292-2535). These
  are the two doc commits, allowed by `specs/0001` ("From P2 on,
  planning's doc updates waiting in the working tree are committed
  unedited as the first commit of the next phase"). They are not the
  implementer's work and are outside what `specs/0008` calls for, which
  the report states.
- Four tests exist beyond the P1 list: `a_zip64_archive_has_no_prefix`
  (diff 1518), `a_zip64_archive_with_a_prefix_round_trips` (1536),
  `a_write_after_a_read_still_round_trips` (1670),
  `a_failed_write_leaves_a_usable_archive` (1778). They serve the spec's
  acceptance; the last cites "Planning's pin (2026-09-16)" (diff 1780),
  which appears in none of my inputs (gap 11).
- One Design sentence has no counterpart in the code: "a reader is
  wrapped with `io::copy`" (`specs/0008:81`). No such helper exists in
  the diff. It is P2 material (nothing in P1 consumes a `Replacement`),
  but it is listed as gap 12.
- `tools/size-probe` is untouched, correct for P1 (`probe-emend-copy` is
  P3, `specs/0008:219-226`).

## Q2

Do the logs contain every line the spec requires?

The log set `specs/0008:178-184` names is complete under
`records/logs/m1p1-p1/`: `gate.log`, `test.log` (both targets),
`negative.log`, `scan.log`, `run.log`.

P1's ACCEPTANCE clauses, one by one, against `run.log`:

1. "every entry `identical` with an empty flags column": host
   `run.log:512` ("entry rows: 870; status identical: 870; rows with a
   non-empty flags column: 0") with verdicts at 513-514; i686
   `run.log:590-592`. The clause is asserted on the ENTRY rows, not only
   on the summary, which is what T3 P4 shows to be necessary.
2. "`files_identical` `yes`": `run.log:500` and 578, with per-workbook
   lines 516-542 and 594-620.
3. "diff exit 0": `run.log:500`, 578 ("diff_exit 0: 27").
4. "`lo` and `lo_in` `ok`": `run.log:501`, 579.
5. "on every corpus file on both targets": manifest rows 27
   (`run.log:464`), summary rows 27 = manifest rows (499, 577), both
   labels run (466, 544).
6. "the gate eight PASS": `gate.log:573-580` lists eight PASS lines,
   `gate.log:583` verdict, exit 0 and empty stderr at 581-584.
7. "`negative.log` covers the tool's exit codes and refusals on
   generated files": `negative.log:604-614` (every fixture generated),
   `negative.log:648-692` (E1 to E8).
8. "recorded as `records/m1p1-p1-<utc-date>-<shortsha>.md` and `.txt`":
   `run.log:629-634`.

Rules for every phase (`specs/0001:90-158`):

- Control bytes: counted with a positive control in `scan.log:468-473`
  and 499-500, and over every scanned file (f_ctrl = 0 throughout,
  e.g. 552, 717, 755). `test.log:502`, 643, 688, 695 count control bytes
  in the cargo and clippy output. Exception: `scan.log` cannot count its
  own (gap 8).
- Colour off: `gate.log:230`, 233; `test.log:42`; `negative.log:41`;
  `run.log:44`.
- `/usr/bin/grep` explicitly: `G=/usr/bin/grep` in the counting library
  printed into every log (`gate.log:304`, `test.log:196`,
  `negative.log:425`, `run.log:287`, `scan.log:318`), and used as `"$G"`
  at every call site, including the gate-file assertion
  (`gate.log:126`). I searched all five logs for an executable bare
  `grep`: every hit is inside a comment or is the version line
  `scan.log:487`. None remains.
- A positive control for every scan form: `gate.log:516-525` (five
  forms, each with a match control and a clean-file control);
  `scan.log:498-526` (f_nonascii, f_ctrl, f_emdash, f_session, f_p6,
  f_pre, f_pre1, f_pre2, f_pre3, f_host, f_deny, f_email, each with a
  match and a clean-file control); counting controls in all five logs.
- Pre-P6d bare-shape control: present in both `gate.log` (read from the
  narrowing commit's parent, 514-515; results 530, 537, 544, 551, 564)
  and `scan.log` (507-512, 776-787). Its coverage in `scan.log` is
  narrower than the report claims (gap 5).
- Commit ids read from git at run time by a printed command:
  `gate.log:46` and 472-477; `scan.log:3-5`, 529-532; `run.log:1-2`;
  `test.log:1-2`; `negative.log:1-2`. None is typed.
- The exact script printed into the log: `gate.log:54-55`,
  `test.log:72-73`, `negative.log:60-61`, `run.log:103-104`,
  `scan.log:127-128`, each with `sed 's/^/    /' "$0"` on the script
  file itself.
- Absolute machine paths, hostname, session ids: no hostname is printed
  (the scan computes it into a control fixture only, `scan.log:166`,
  and f_host = 0 on every scanned file); f_session = 0 everywhere
  (`scan.log:554`, 684, 694, 704, 719). One path shape is printed in a
  log: `run.log:45` (gap 7).
- Repo-relative paths, `<scratch>` for scratch locations:
  `negative.log:49`, 93; `scan.log:9`; `run.log:54`.
- Range scan written `<base>..HEAD`: `scan.log:2` and 529 use the
  explicit base..HEAD form.

One assertion in `gate.log` cites a rule that does not exist (gap 1).

## Q3

Does the report's every number and quoted string appear in a primary?

Found in a primary:

- Base `6f4764de...`: `gate.log:476`, `scan.log:2`, `scan.log:5`.
- `acc36fb70...`, `791d0d7d...`, `e14dbee5...`: `gate.log:481-483`,
  `scan.log:530-532`; the phase commit also at `run.log:2`,
  `test.log:2`, `negative.log:2`.
- "three commits in the range": `gate.log:484`, `scan.log:709`.
- Runner glob `"$dir/.out.$LABEL.$ext."*.tmp`: diff 1877 and 1890.
  "the diff is two hunks": diff 1862, 1881. "`gate.log` records `bash
  -n` parsing it": `gate.log:510`.
- The corpus table: summary rows 27 of 27 manifest rows
  (`run.log:464`, 499, 577); outcome ok 27, diff_exit 0 27,
  files_identical yes 27 (500, 578); lo/lo_in 27/27 and leftover no 27
  (501, 579); entry rows 870, identical 870, flagged 0 (502, 512, 580,
  590); recompressed/changed/dropped/added all 0 (502, 580); wall
  00:17:51Z to 00:20:11Z (468) and 00:20:11Z to 00:22:32Z (546).
- The pins `run.log:499-514` and `run.log:577-592` are accurate.
- T3 P4 comparison: `records/logs/t3-p4/run.log:136` (870 identical per
  label) and 142-143 (attrs 870, extra 578, gpflags, descriptor and
  local 292 each; files not byte-identical). Cited line numbers check
  out.
- Log sha256s for `gate.log`, `test.log`, `negative.log`, `run.log`:
  `scan.log:715`, 724, 733, 742.
- Verdict strings: "gate log checks all as expected: yes"
  `gate.log:597`; "test log ... yes" `test.log:706`; "negative log ...
  yes" `negative.log:702`; "run log ... yes" `run.log:646`; "RESULT:
  PASS" `scan.log:793`.
- Record pair: 5619 bytes and its sha256 `run.log:632`; 198280 bytes and
  its sha256 `run.log:633`; "no home, mount or corpus path"
  `run.log:637-640`.
- Four binary hashes: `gate.log:587-590`, repeated at `run.log:459-462`
  and `negative.log:596-599` (cited, not recomputed, as the report
  says).
- Base uniqueness assertion: `gate.log:78` and 473-474; "scan.log
  asserts the same" `scan.log:10` and 125.
- "ten gate files are named, none is in the phase diff":
  `gate.log:504-505`.
- test.log claims: 98 passed, 0 failed, 1 ignored on each
  (`test.log:499`, 640); same test names (`test.log:699`); 28 layer
  tests listed by name (`test.log:646-674`); the ignored test named
  `zip64_entry_over_4_gib` and 0 ignored in the layer (`test.log:677-
  679`); size-probe builds and passes clippy with the `xml` feature on
  both targets (`test.log:684-695`).
- negative.log claims: L1 to L4 (`negative.log:619-646`); E1 to E8 with
  the exact stderr lines (652-692) - every quoted message in the
  report's table appears verbatim there (659, 664, 671, 676, 681, 691);
  the usage text read from the source at run time (649-651); no
  emend-copy temporary survived (694-696).
- scan.log claims: 0 hits, 0 errors, 0 structural failures, PASS
  (789-793); 14 files scanned and read (676); 3 messages, each with the
  Co-Authored-By line last and f_session 0 (679-709); sibling logs and
  record pair scanned (715-770).
- Fault 1's value 76: diff 464 (archive.rs comment) and diff 1522
  (tests.rs comment); the three pinning tests: diff 1509, 1518, 1536.
- Fault 2's measurement, 5 at the base, 0 in the added lines, 5 at HEAD:
  `gate.log:566`.
- Fault 8, `mkdir -p` for the runs directory: `run.log:58-62`.
- "The scan runs twice": `scan.log:7`.
- "The size probe waits for P3: yes, untouched": no `tools/size-probe`
  path appears in the diff's file list (`scan.log:536-549` lists all 14
  changed files).

Not found in any primary, or found in a weaker form than the report
states: gaps 1 to 4, 5, 6, 9, 10, 11 below. I re-derived several of
these independently and they are FACTUALLY correct; the finding is that
the inputs given to a checker do not contain them.

## GAPS

1. `gate.log:103-105` quotes `specs/0008` for a rule the spec does not
   contain: "specs/0008: 'If P1 touches any gate file (.gitignore
   included), run scripts/gate-self-test.sh detached and commit
   self-test.log with the other logs.'" `specs/0008-zip-layer.md`
   contains no occurrence of "self-test", "gate file" or "gitignore".
   No file under `specs/` contains that sentence or its paraphrase.
   `specs/0007` (diff 2283-2288) says the rule is to be ADDED to
   `specs/0001` in T6 P2, which has not run, so the rule does not yet
   exist anywhere. The draft report repeats the misattribution at report
   lines 106-108 ("`specs/0008`'s rule that a phase touching a gate file
   must run the self-test"). The assertion's RESULT is sound (0 of 10
   gate files in the diff, `gate.log:504`), and running the check anyway
   is right; the citation is not.

2. Report lines 70-76: the five log byte counts (33280, 42260, 37464,
   38051, 39925) appear in no log and are not in the diff. I measured
   them myself and all five are correct. Separately, the sha256 the
   report gives for `scan.log` (report line 76) appears in no primary,
   since a log cannot hash itself; the other four are at
   `scan.log:715`, 724, 733, 742.

3. Report lines 15-17: the per-commit counts "4 files, 440 insertions,
   52 deletions", "2 files, 47 insertions, 6 deletions" and "9 files,
   1898 insertions, 4 deletions" appear in no log (no log contains the
   word "insertion") and cannot be read off the diff, which is the
   combined 14-file range (`gate.log:501`, `scan.log:676`). I confirmed
   all three from `git show --stat`, and the per-commit file lists the
   report gives are also correct; they are simply unsourced in the
   checker's inputs.

4. Report lines 237-246, "An environment finding": "19229 MB available
   of 22015", "no OOM lines in the kernel log", "the largest resident
   process was the session itself at 478 MB", "the third such kill in
   this session". None of these figures is in any log or in the diff,
   and nothing in the inputs lets them be checked. `specs/0001:141-143`
   is explicit that nothing is measured unless a log prints the
   measurement. Also in that paragraph: the report says the run was
   "started detached with `setsid nohup ... disown`"; `run.log:5` and
   90-95 record setsid and nohup only, with no mention of disown. The
   completion time 00:22:32Z is sound (`run.log:645`).

5. `scan.log:772-787`, the pre-P6d bare-shape control, covers 11 files:
   the four new sources, `scripts/passthrough-run.sh`, the four sibling
   logs and the record pair. It does NOT cover three files this phase
   ADDS (`crates/emend/Cargo.toml`, `tools/emend-copy/Cargo.toml`,
   `specs/0008-zip-layer.md`), the six other modified files in the diff,
   or the three commit messages, all of which the narrowed live patterns
   did scan (`scan.log:550-675`, 679-708). `specs/0001:128-132` asks
   every scan script to run the pre-P6d forms over its files. Report
   lines 165-166 say the forms were "run over every file this phase
   adds", which the log does not show. (`gate.log:530`, 537, 544, 551
   does cover the four new sources, so the four biggest new files are
   double-covered; the three added manifests and the spec are covered by
   neither pre-P6d scan.)

6. Report lines 139-141: "L4 CLOSES THE BACKLOG ITEM 'stale-temporary-
   file removal has no negative case' on `ROADMAP.md`". `ROADMAP.md:205-
   206` reads "the runner's `|| die` guards and the stale-temporary-file
   removal have no negative case": one compound item covering two
   things, of which L4 addresses one. The quoted string is not verbatim
   in the primary, and the item as written is not wholly closed. The
   same quotation appears inside the negative script's comment
   (`negative.log:284-285`), which is prose, not a measurement.

7. `run.log:45` prints, inside the committed script, the line that sets
   `CORPUS`, whose default value spells out the home directory variable
   followed by the corpus directory's own name (not quoted here). The
   log therefore names the corpus directory, while its own header
   (`run.log:16`, 32-33) says the corpus directory is written `<corpus>`
   and `run.log:246-248` asserts exactly that absence for the RECORD
   PAIR only, not for the log. `hide()` (`run.log:54`) rewrites captured
   output but not the printed script. `scan.log:742-750` shows f_deny =
   0 over `run.log`, so the string is not a denylist entry, and f_p6 = 0
   because the narrowed patterns do not cover a home-directory prefix.
   Planning should rule whether this is a defect under
   `specs/0001:148-158`. Note that `run.log:246` also prints the
   corpus-name fragment as part of a search pattern.

8. `scan.log` is the only scan in the phase and cannot scan itself
   (`scan.log:246`), so no primary records its control-byte, non-ASCII,
   em dash, session-line, denylist or email counts. The report states
   this at lines 223-226; recorded here so the phase's clean scan is not
   read as covering `scan.log` itself. The same holds for the record
   commit, which is not yet made.

9. Report line 158: "No output file survives any refusal". The log
   asserts this for E2, E3a, E3b and E4 only (`negative.log:661`, 666,
   673, 678). E5 to E7 are usage errors with no output path, and E8
   (`negative.log:689-692`) has no such assertion. The sentence
   generalizes four checks to eight cases. The companion claim about
   temporaries is backed (`negative.log:694-696`), though that find runs
   at `maxdepth 1` of `<scratch>` only.

10. Report lines 199-214, faults 3 to 7: their evidence is the prose
    comments printed inside the committed scripts
    (`negative.log:126-131`, 304-310, 328-336; `gate.log:122-125`,
    192-208), which describe code that no longer exists. Nothing in the
    inputs shows the pre-fix state, and `specs/0001:141-143` says a
    figure in a script's comments is not a measurement. Fault 2 is the
    exception and is properly measured (`gate.log:566`). This is not a
    reason to doubt the account; it is a statement of what the primaries
    can and cannot establish.

11. diff 1780: `a_failed_write_leaves_a_usable_archive` is justified by
    "Planning's pin (2026-09-16): after a write that fails part-way,
    every later call returns Err or works, never panics". No such pin
    appears in `specs/0008`, `specs/0001` or any log in my inputs. The
    test is good and its claim is verified by the test itself; the
    citation cannot be checked from the inputs.

12. `specs/0008:81` says of `Replacement` that "a reader is wrapped with
    `io::copy`". No such wrapper or helper exists in the diff
    (`crates/emend/src/archive.rs`, diff 343-397). P1 consumes no
    `Replacement`, so this is most likely P2 work, but it is a Design
    element the diff does not deliver and the report's acceptance list
    (report lines 250-263) does not mention it.

13. Minor, `gate.log:136-139`: the parse check is written
    `bash -n "$s" 2>/dev/null` followed by
    `note "$s parses" "$(is [ $? -eq 0 ])"`. It evaluates correctly here
    (the command substitution inherits `bash -n`'s status), but it reads
    `$?` across an argument list rather than inside one function, which
    is the shape `specs/0001:111-113` warns about. A future edit that
    inserts any command between the two lines would silently change what
    is being tested.

Nothing else was found. On Q1 the diff matches the spec item for item
with the exceptions listed as gaps 11 to 13; on Q2 every log the spec
names is present and every P1 acceptance clause is asserted in a primary,
with gaps 1, 5, 7 and 8 against the rules; on Q3 every figure in the
report that a primary could carry is carried, and gaps 2, 3, 4, 6, 9 and
10 name what is not.
