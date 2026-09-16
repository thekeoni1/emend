# Independent check 1, T6 P1

checked commit: 2f5fae9acd841379f5bbee53b56ebe5e67670e74
HEAD when the check ran: 2f5fae9acd841379f5bbee53b56ebe5e67670e74
phase commit now: 0a337fb79ad03aff9d78756c387cc7fa96571cf4 (the code commit was amended after this check, to carry its repairs)

NOTE. This check read the SUPERSEDED state: commit 2f5fae9 and the four
uncommitted logs of the first round, none of which is the evidence now
committed. Every log it names was replaced after it reported. Its input
hashes below are those superseded files, recorded before they were replaced.

inputs given to the checker:
  specs/0007-gate-self-test.md at 2f5fae9: sha256 4dd9c5ebe7b0e2355bba3e63e30f9f7eddbd13b9804e4f5053743a51c890bf87
  specs/0001-bootstrap.md at 2f5fae9: sha256 82fe4bd43c332933fe09ae9c4519bdaf5eab853b336cfd48c339dd9129f9a506
  scripts/check.sh at 2f5fae9: sha256 7272fd137969c3b0df482e61affb8b23c8146046bdfb08ab8b3dd73879a9be5a
  scripts/counting.sh at 2f5fae9: sha256 93a464cf601203a826974a09b79625f1da70ef5e88918546ac016d08908b96b7
  scripts/gate-self-test.sh at 2f5fae9: sha256 521c9a3cda7be4c86d6190af17f5966e321ec45a7d227e76fe60bc2b7fc54acb
  diff 731d6ff..2f5fae9: sha256 4441d4eaad2a671d7637087ab466ab5e547e1d95f08f24833481c47ebb8dc577
  records/logs/t6-p1/gate.log (superseded): sha256 0e132c350251e14523e1e14acfc32cfbda46cba9228060970fbb4fe5f09ff9db
  records/logs/t6-p1/scan.log (superseded): sha256 672c028fb8aa9afdcabb3db86ef552d2cf0196f4c6036df326ec0b013c77ab61
  records/logs/t6-p1/negative.log (superseded): sha256 18bdac3c2a35b69ae4ca49147eedf47be810d6c0a1264c8a7a85e6dc862509f1
  records/logs/t6-p1/self-test.log (superseded): sha256 5170cd870dfa15f7c3444066a2f7c60c21e313d883a86beceecbfe4ab370ed1a
  the draft report of the first round (session file, not committed)

REVERSIBLE REPLACEMENT (planning's ruling, 2026-09-16). This answer quoted a
bare shape of privacy pattern 8 while describing scan.log's control for it.
The live patterns are blind to a bare shape by design, so the gate scored 0 on
this file, but the pre-P6d bare-shape control fires on it, and specs/0001
line 126 allows a documented, reversible replacement as the only exception.
specs/0001 line 55 defines the mechanism for a non-ASCII CODE POINT; planning
extended it to an ASCII shape for this case. The original bytes are given as
hex, not literally, because writing them here would reintroduce the shape.

  position:        line 357, column 39 of the answer; byte offset 22490 in the
                   whole file as committed at 4d2ece2 (equivalently, offset
                   20479 within the extracted answer)
  original bytes:  hex 2f6d6e742f632f55736572732f (13 bytes)
  replacement:     hex 3c62617265207061746820736861706520313e (19 bytes)
  extracted answer before: 25392 bytes, sha256 3da5c8792fc60bf64c10dab7c4177fc51b2d2fcb707c7a6532da8a7df4d5e204
  extracted answer after:  25398 bytes, sha256 6704066d8a7b8b24101830d0aa821993625dafb53ba6107ec102a0b757442bcd
  whole file before:       27403 bytes, sha256 d685c29aff702de154bcdbbf15661d9a69d8d6b6fbf1ef931d775bcfcd75b6f6
To reverse: replace the replacement bytes at that position with the original
bytes. Nothing else in the answer is altered.

ASCII substitutions: the answer arrived with angle brackets HTML-escaped by
the transport (&lt; and &gt;); they are restored to < and > here. Nothing
else is altered. The answer holds no non-ASCII byte and no U+2014.

---

Note: the harness placed CLAUDE.md in my context. I did not use it as evidence.
I judged only from the spec, the diff, the draft report, the four t6-p1 logs,
the three scripts at HEAD, and the source logs under records/logs/. I did not
re-run gate-self-test.sh; behavioural claims below come from the code plus two
isolated bash demonstrations, which I name.

## Q1. Does the diff do what the spec says for P1, and nothing else?

Delivered, and present:

- check.sh --only <step>: scripts/check.sh:12-60 (parse with shift, refusal on
  bad name, on missing value, on combination with --quick or --ci), wanted() at
  :53-57, eight "if wanted <step>" wrappers at :111, :115, :132, :135, :143,
  :175, :278, :441. The ANTHROPIC_API_KEY check stays ahead of every step.
  Evidenced at gate.log:28-37.
- scripts/counting.sh: the four forms at :30, :40, :51, :63 and
  counting_controls at :81-111, fifteen controls, run before use at
  gate-self-test.sh:207, negative.log:50, scan.log:114.
- scripts/gate-self-test.sh with H0 to H10, V0 to V16, K1 to K3 and B1: all
  present; self-test.log:31-78 shows 42 case lines.
- records/logs/t6-p1/self-test.log: present.

In the diff but not called for by the spec:

1. --deep (gate-self-test.sh:21-31, :45, :176-185). The spec's design line is
   scripts/gate-self-test.sh [--full] [--case <id>...] (specs/0007:70). --deep
   is a new flag, and worse, it changes what the default is: without it every
   privacy case runs privacy.sh --range <base>..HEAD, which gate-self-test.sh:201
   itself admits "differs from the sources". The spec has no default other than
   the sources' invocation.
2. V17 and V18 (gate-self-test.sh:586-608). The catalogue is V0 to V16
   (specs/0007:126-154). Two extra cases, not listed in the report.
3. H4/H8 split into H4a/H4b and H8a/H8b, V11 into four subcases, V12 into two.
   Benign, and arguably what the spec's wording implies.

Called for by the spec and missing or unevidenced:

4. records/logs/t6-p1/self-test.log is not the run the spec names.
   specs/0007:189-190 requires "a full default run from the main tree at the
   phase commit". self-test.log:4 reads "mode: deep". The committed evidence run
   is therefore not the run P1 specifies, and no default run is committed at all.
5. The sentinel / control-byte / non-ASCII counts over captured outputs are not
   implemented. specs/0007:91-93: "captured outputs are counted for control
   bytes, non-ASCII bytes and the sentinel, and the counts printed."
   gate-self-test.sh never calls count_bytes and never counts the sentinel;
   self-test.log has no such line. Every T0 source log printed exactly this
   guard (for example t0-p6/negative.log:492, t0-p6b/negative.log:484). This is
   the one check that would prove the self-test never leaked the sentinel into a
   committed log. I found no sentinel-shaped string in self-test.log by
   inspection, and scan.log:538-546 scores f_deny=0 over it, but f_deny matches
   the real denylist, not the scratch sentinel, so the property the spec asked to
   be measured is unproven rather than violated.
6. V7's "extra commit header" sub-case is dropped. specs/0007:140-141 names
   T0 P6c d as well as e. t0-p6c/negative.log:639 records that as a distinct
   class, "FAIL header: commit <sha> email line 5", with its own disable test at
   t0-p6c/negative.log:778. V7a/V7b/V7c cover content, message body and path
   only. The header detection class has no case.
7. V9's compound-file half is dropped. specs/0007:145: "archive and
   compound-file signatures in files named x.bin (T0 P6c c, T2 P0 b)".
   gate-self-test.sh:480 plants only PK\003\004 in notes.txt. The OLE2
   signature, which is precisely what t2-p0 case b1 added
   (t2-p0/negative.log:518), is never exercised.
8. gate.log does not print its script. "Rules for every phase" requires "Print
   the exact script into the log." negative.log:7-209 and scan.log:9-239 comply;
   gate.log has no script section at all.

Everything else in the diff is the doc commit 39b2249 (ROADMAP.md, specs/0000,
0001, 0005, 0007), which the phase rules require as the first commit.
git show --stat 39b2249 gives 5 files, 272 insertions, matching report:9.

## Q2. THE CENTRAL QUESTION: does every case take its expectation from its named source?

I opened every cited source line. Result by case.

Verified exact against the named source line (expectation string equals what the
source log recorded, allowing only the declared <sha> normalisation):

| Case | Source line | Verdict |
|---|---|---|
| H4b | t2-p0/negative.log:837 | exact |
| H5 | t0-p6/negative.log:706 | exact |
| H6 | t0-p5b/negative.log:222 | exact |
| H7 | t0-p5b/negative.log:204 | exact |
| H8a | t0-p5b/negative.log:240 | exact (PASS, exit 0) |
| H8b | t0-p5b/negative.log:349 | exact |
| H10 | t0-p5b/negative.log:267 and :274 | exact, including "lines 1" from :274 |
| V1 | t0-p6/negative.log:490 | exact |
| V2 | t0-p6/negative.log:502 | exact |
| V3 | t0-p6/negative.log:514 | exact |
| V4 | t0-p6/negative.log:528 | exact, and the fixture writes a leading "first\n" so the token lands on line 2 as the source recorded |
| V5 | t0-p6d/negative.log:352-354 | exact; the note "FAIL lines 3" matches the three lines |
| V6 | t0-p6d/negative.log:361-365 | exact |
| V7a | t0-p6b/negative.log:789 | exact, fixture even reuses the name g1.txt |
| V7b | t0-p6b/negative.log:800 | exact |
| V10 | t0-p6b/negative.log:512 | exact |
| V12remove/add | t0-p6b/negative.log:589 and :677 | exact, and the count is re-read at run time (gate-self-test.sh:520) rather than typed |
| V14 | t0-p6/negative.log:760 | exact |
| V15 | t0-p6/negative.log:770 | exact |
| V16 | t0-p6/negative.log:779 | exact |
| V18 | t2-p0/negative.log:487-488 | exact, both lines in the recorded order |
| K1 | t0-p6/negative.log:578, :589, :592, :599, :601, :606, :608 | every component traces to the source |
| K2 | t0-p6b/negative.log:555, :557 | exact |

That is a real and substantial majority, and several of them (V4, V5, V12, V18,
K1) show positive evidence of having been read off the log rather than recalled.

Cases where the expectation is not fully in any source. Listing every one:

- K3 (worst). Source is given as scripts/install-hooks.sh:11-13, first exercised
  in T6 P1. I grepped every log under records/logs/ for hooksPath: the only hits
  are state lines such as t0-p6b/hook.log:115 "core.hooksPath in any git config:
  unset". No log anywhere records this refusal. The spec named T0 P6b d as K3's
  source (specs/0007:161-162); t0-p6b/negative.log:521-560 shows d1, d2, d3 and
  contains no hooksPath case. So K3's expectation is read off the current script
  text and cannot diverge from it by construction. The spec's own rule
  (specs/0007:102-104) says a case whose source expectation is not clear "is
  left out and listed in the report for a planning ruling". K3 was neither. The
  label is honest; the rule was not followed.
- H3. Source given as "check.sh OFFICE_RE + its control" - no log cited. The
  spec named T0 P6c b (specs/0007:113). t0-p6c/negative.log:564 does record the
  csv fixture, but as a FAIL paths privacy line, not a hygiene line, so there is
  genuinely no source log for csv/tsv at step 7. The consequence is that H3's
  authority is OFFICE_RE itself (check.sh:289): if OFFICE_RE were wrongly
  widened, H3 would still pass. Partially anchored by the in-gate control at
  check.sh:368-371, which pins e.Csv.bak and .~lock.d.ods# - but that control is
  part of the gate under test.
- H4a. Cites check.sh ZONE_RE, yet the identical wording IS recorded, at
  t2-p0/negative.log:837, the very line H4b cites. Under-citation, not fitting;
  trivially fixable.
- H2. Cites t0-p6c i, which at t0-p6c/negative.log:843 recorded count 1 for a
  single name.xlsx~. H2 plants three forms and expects 3. The lock/tilde/.bak
  triple comes from t0-p6c a1/a2/a3 (t0-p6c/negative.log:516, :533, :550), which
  the label does not name, and each of those recorded 1, in its own clone, as a
  privacy line.
- H1. Cites t0-p5 b1,b2 "via t0-p6b L842". t0-p5/negative.log:241 and :257
  recorded the older path-printing wording with one file each;
  t0-p6b/negative.log:842 recorded the count-only wording with count 1. H1
  expects count 2. The wording is sourced; the count 2 appears in no log. I
  confirmed by grep that no log anywhere records a hygiene office line with count
  2 or 3 - the only hits are the t6-p1 self-test lines themselves.
- V17. Same shape: the four cited source lines (t0-p6c/negative.log:516, :533,
  :550, :564) each recorded 1; V17 expects 4.

For H1, H2, H3 and V17 the count component is arithmetic from the fixture, not
from a source. That is defensible and I would not call it dishonest, but it is
not what specs/0007:101-102 says ("takes its expected result from there"), and
the log does not say which half of each expectation is sourced.

- Assertions weaker than the source recorded (so a reworded gate message would
  pass): V11's four subcases assert only "exit 2" (gate-self-test.sh:512-513)
  while the ERROR wording rides in an uncompared note; likewise V13 (:549);
  likewise B1, which asserts "FAIL fmt line 1" (:689) where
  t0-p3/negative.log:100 records the full wording. Not fitted, but these cases
  cannot detect the change their sources would have detected.
- H9 asserts "PASS hygiene" where the source line t0-p6b/negative.log:512 is
  "PASS privacy". The label says "(PASS either step)", which is honest, and the
  path differs too (tools/x/tests/fixtures/ vs tests/fixtures/).

Signs an expectation was fitted to current behaviour. K3 unambiguously is: there
is no recorded outcome for it to differ from. H3 effectively is, since OFFICE_RE
is both the subject and the authority. H4a reads as fitted but is not - the
wording exists at t2-p0:837. I found no case where a source log's recorded
string was contradicted by the committed expectation. The commit message's rule
broadening ("a source is a primary: the log where the case ran, or the gate
script's own text where no log holds it") is the implementer's own extension of
the spec, disclosed in the commit message and at gate-self-test.sh:8-15, but
never put to planning as the spec requires.

V7c, judged specifically: honest. The source ordinal is real
(t0-p6c/negative.log:651, "tree entry 8"). normalise() at
gate-self-test.sh:90-100 explains the reason, the reason is printed in the log
line itself (self-test.log:55), and the normalisation is applied symmetrically to
expectation and observation, so it can hide only the ordinal, not the detection
class. I accept the handling and I accept that the log says so. Two caveats: it
would also mask the gate printing a nonsensical ordinal; and the report's claim
at report:74 that the fixture "now lands at 9" appears in no primary, because the
log only ever prints <n>.

V8, judged specifically: honest in the log, overstated in the report. The
expectation at gate-self-test.sh:473 is correct, and the comment at :466-471
accurately says the archives class post-dates t0-p6b. But the premise that no
single log records this union is wrong. t0-p6c/negative.log:680-697 is the same
fixture - "a gzip-compressed emend-testing/corpus/private.xlsx, committed then
deleted" - re-run after the archives class existed, and it records all three FAIL
lines together (:686, :687, :688) plus "FAIL lines: count=3 (expect 3)" at :697.
Only the archives wording changed again at T2 P0 (t2-p0/negative.log:183, :524).
So the honest sourcing for V8 is "t0-p6c g-a for the union, t2-p0 b for the
current archives wording", and t0-p6c g-a - the closest and most complete
primary - is cited nowhere. The report at report:78-81 presents this as a
judgement call resolved by a planning ruling on principle 2, built on a premise
that t0-p6c:686-688 contradicts.

## Q3. Logs against "Rules for every phase", and the report's citations

First line names the commit and HEAD. gate.log:1-2, negative.log:1-2 and
scan.log:1-2 all use "command: git rev-parse HEAD" then an "about:" line naming
the phase commit and the HEAD at run, consistent with prior phases. self-test.log
is weaker than its three siblings: line 1 is a title, the commit appears only at
line 3, there is no phase name, no run timestamp, no end timestamp and no printed
invocation.

The exact script printed. negative.log:7-209 and scan.log:9-239 comply. gate.log
does not print its script at all, so the four numbers at gate.log:40-43 have no
printed command form and no control anywhere - and those are exactly the four
numbers the report cites at report:47-48. self-test.log prints no script either,
mitigated by the script being committed at the phase commit.

Counting forms that read the exit status, with positive controls run before use.
counting.sh is sound and its controls run first in all three logs that use it
(self-test.log:10-25, negative.log:325-340, scan.log:355-370). But there are four
sites that defeat or bypass it, and all four are unadmitted:

1. gate-self-test.sh:648, "entry printed $((leak + leak2))". leak and leak2 come
   from count_matches at :644-645, which returns the string "error" on a read
   failure. Bash evaluates "error" inside $(( )) as an unset variable, so it
   becomes 0 - which is exactly the expected value written at :647. I
   demonstrated this: leak=error; leak2=error; echo "entry printed $((leak +
   leak2))" prints "entry printed 0". So if either push output became unreadable,
   K1's leak check would silently report the safe answer and the case would still
   read as-expected. This is precisely the class counting.sh:6-13 was written to
   close, sitting in the one check that guards a real private denylist entry.
2. gate-self-test.sh:144-155, fresh(). It does errors=$((errors + 1)) inside a
   function invoked as d=$(fresh H1) - a command-substitution subshell - so the
   increment is discarded in the parent. I demonstrated this: the parent's errors
   stays 0. On a clone failure the case is silently skipped (&& short-circuits),
   cases is not incremented either, and nothing in the script asserts an expected
   case count. The summary at :697 would read "41 cases, 0 unexpected, 0 errors"
   and :699 would print "RESULT: PASS". A whole case can vanish from a passing
   run.
3. negative.log:205, STATUS=$(git status --porcelain | LC_ALL=C "$G" -a -c -v -e
   'records/logs/t6-p1' || true), then [ "$STATUS" -eq 0 ] at :206. A bare grep
   -c outside the library with || true swallowing the exit status; if git status
   failed or grep errored, STATUS is 0 and :418 prints as-expected. The same
   script's own header at negative.log:25 states "Every count goes through
   scripts/counting.sh, whose controls run first" - untrue at line 205.
4. gate-self-test.sh:525 and :527, n=$((want_n - 1)) / $((want_n + 1)) over a
   count_matches result. These fail loudly rather than silently (the case goes
   UNEXPECTED), but it is the same arithmetic-on-"error" pattern.

Control bytes and non-ASCII. scan.log:520-546 counts all three sibling logs at 0;
scan.log cannot scan itself and declares it at :202 and :210, per the accepted P1
ruling. negative.log:416 counts its own captures 0/0. I found no non-ASCII or
control bytes in what I read.

Negative tests in scratch clones, never the main working tree. N1 clones
(negative.log:55), F2 clones (:144), F3 clones (:165). But N2 (:67) runs ( cd
"$MAIN" && ... ./scripts/gate-self-test.sh --case H0 ), N3 (:76) runs
./scripts/check.sh $a, and N4 (:83) runs ./scripts/check.sh --only hygiene - all
three in the main working tree (cwd is set to the repo root at :27). They are
read-only and N2 exits at the guard, so nothing was written. But specs/0001 says
every negative test runs in a scratch copy and "A departure from this needs
planning's word before the run", and no departure is declared. Worse,
negative.log:23-24 states "Every case runs in a scratch clone under <scratch>,
never in the main working tree, which specs/0001 now requires as the rule rather
than a departure." That sentence is false of N2, N3 and N4, and it is committed
in a primary.

Commit ids read from git. Satisfied, and done well: gate.log:9 prints the
command; scan.log:3-4, :40-43, :100-101 resolve the base from the doc commit's
subject rather than typing it.

Report citations, sampled by opening the cited lines. Correct: report:9 (git show
--stat 39b2249 gives 5 files, 272 insertions); report:36-37 (I counted 13
hygiene, 25 privacy, 3 hook, 1 build at self-test.log:31-78, summing to the 42 at
:80); report:43 (self-test.log:80-81, :3); report:46 (gate.log:49-57); report:50
(I counted exactly 16 results at negative.log:348, :349, :353, :354, :358, :360,
:362, :364, :368, :375, :384, :393, :400, :411, :417, :418, with :420 "all as
expected: yes"); report:54-58 (scan.log:565, :561-563, :405, :3-4, :494, :498,
:508, :547, :559); report:94 (negative.log:390-392); report:97
(negative.log:399).

Incorrect or unevidenced:

- report:26-29 is false as a gate.log citation. "Verified in gate.log: five
  refusals ... each cheap step prints exactly one line, and --only i686 prints
  PASS i686 with no SKIP line, which the self-test depends on." gate.log:33-37
  tests five steps: fmt, native, licences, hygiene, privacy. There is no --only
  i686 line in gate.log, nor --only host, nor --only clippy; I grepped every log
  under records/logs/ for --only and the only files that contain it are the four
  t6-p1 logs. The same claim is asserted in the commit message of 2f5fae9. It is
  also the one --only behaviour with real risk, because the i686 step body
  contains the --quick SKIP branch (check.sh:115-130). "Each cheap step" covers 5
  of 8.
- report:38 and the "Cost, measured" heading at report:60. Neither "3.2 minutes"
  nor "10.5 minutes" is printed in any log. self-test.log prints per-case seconds
  only, with no total and no timestamps. The 10.5 figure is recoverable by
  summing self-test.log's per-case values to roughly 623 s, but the log does not
  print it; the 3.2 figure corresponds to no committed run at all, since no
  default run was logged. This runs against "Nothing is called measured unless
  the log prints the measurement".
- report:74, "now lands at 9": the 8 is real (t0-p6c:651); the 9 is in no
  primary.
- report:106-107, "541127 control bytes, 532500 non-ASCII bytes": these appear
  only inside the script's own comment at negative.log:18, which is prose in a
  script, not a printed measurement of a run, and the superseded log was not
  committed. Per "a figure in a script's prose or comments is not one (T3 P4
  check 1)", these two numbers lack a primary. The fault they describe is real
  and the log is candid about it.
- report:11-14, "Amended once": no primary evidences it; the reflog is not a
  committed primary. Minor, and the new rule permits the amend.

Are the seven admitted faults accurate and complete? Accurate: I checked each
against a primary and all seven hold - fault 1 against self-test.log:3 and :28
with gate-self-test.sh:216; fault 2 against negative.log:378-385 and
gate-self-test.sh:256-269; fault 3 against negative.log:387-394 and
gate-self-test.sh:104-119; fault 4 against negative.log:396-401 and
gate-self-test.sh:188-192; fault 5 against negative.log:403-412 and
gate-self-test.sh:612-622; fault 6 against negative.log:4 and :115-121; fault 7
against gate.log:4.

Complete: no. The report's own claimed pattern at report:109-111 - "when I
re-implement a gate behaviour in a harness instead of observing the gate, my copy
diverges" - still stands in three places:

- negative.log:109-110 hand-types the gate's regexes. OFF is a full hand copy of
  check.sh:289's OFFICE_RE alternation and ZON of check.sh:292's ZONE_RE. I
  compared them: they are equivalent today (only [.] versus \.). But F1, the
  fault the report says it closed, WAS a re-implementation of the gate's
  matching, and the fix at negative.log:124 repaired the flags while leaving the
  patterns a hand copy that nothing reads from check.sh and nothing compares to
  it. The report's "F1 now measures with the gate's own invocation form"
  (report:111) therefore overstates: it uses the gate's invocation form over a
  hand-typed copy of the gate's pattern.
- scan.log:51-56 hand-types the three pre-P6d patterns into PRE. I checked them
  against git show 721e863^:scripts/privacy-patterns.txt lines 8, 10 and 11: they
  match exactly today. But only one of the three has a positive control -
  scan.log:140 builds c-bare with the <bare path shape 1> shape only, and :155 controls
  f_pre against it. The pre-P6d forms of patterns 10 and 11 have no control, so a
  typo in either would score 0 hits and read as clean. That matters because
  scan.log:549-559 is the P1 acceptance condition the spec names. The contrast is
  instructive: scan.log:128-129 does the right thing for the live patterns,
  reading them from the file and cmp-ing them against the committed file at $C.
  The same technique was available for PRE and was not used.
- gate.log:42-43's "11 live patterns" and "three pre-P6d bare forms" come from a
  script that is never printed, so whatever it re-implemented cannot be reviewed
  at all.

## GAPS

1. gate-self-test.sh:648 - K1's "entry printed" check cannot fail. $((leak +
   leak2)) over a count_matches result evaluates "error" as 0, the expected value
   at :647. Demonstrated. This is the silent-zero class the project has declared
   closed four times, in the one check that guards a real denylist entry. Route
   both counts through an explicit string comparison.
2. gate-self-test.sh:144-155 - fresh() loses errors in a command-substitution
   subshell. Demonstrated. A failed clone silently drops the case; cases is not
   incremented and no expected-case-count is asserted, so the run still prints
   RESULT: PASS. Return the error through the exit status and assert the expected
   number of cases at :697.
3. specs/0007:91-93 not implemented. No counting of control bytes, non-ASCII
   bytes or the sentinel over captured outputs, and no such counts in
   self-test.log. This is the guard that would prove the self-test never leaked
   the sentinel; every T0 source log printed it.
4. report:26-29 cites gate.log for --only i686 printing PASS i686 with no SKIP
   line; it is in no log. gate.log:33-37 covers five of eight steps. The same
   claim is in the commit message of 2f5fae9. Either run it and log it, or strike
   the claim.
5. negative.log:23-24 states every case runs in a scratch clone; N2 (:67), N3
   (:76) and N4 (:83) run in the main working tree. Read-only, but an undeclared
   departure from specs/0001 plus a false statement committed in a primary.
6. negative.log:205 uses a bare grep -c ... || true and compares with 0,
   contradicting its own header at :25. Same silent-zero class.
7. The committed self-test.log is a --deep run; specs/0007:189-190 requires a
   full default run. --deep is itself absent from the spec's design
   (specs/0007:70) and changes the default privacy invocation away from the
   sources.
8. K3 and H3 take their expectations from current script text with no log
   anywhere. specs/0007:102-104 requires such cases to be left out and listed for
   a planning ruling; neither is listed in the report. The commit message's
   broadening of "source" to include the gate's own text was never put to
   planning.
9. gate.log prints no script, contrary to "Print the exact script into the log",
   and its four scan numbers at :40-43 have no positive control - yet
   report:47-48 cites them.
10. Unevidenced figures in the report: "3.2 minutes" and "10.5 minutes" under a
    heading reading "Cost, measured" (report:38, :60) appear in no log; "now
    lands at 9" (report:74) appears in no primary; "541127" and "532500"
    (report:106) appear only in a script comment at negative.log:18.
11. Remaining re-implementations of gate behaviour: negative.log:109-110
    hand-copies OFFICE_RE and ZONE_RE (correct today, unchecked against
    check.sh), and scan.log:51-56 hand-types the three pre-P6d patterns with a
    positive control for only one of them, while scan.log:128-129 shows the
    correct read-and-compare technique for the live patterns. The report's claim
    that this pattern was closed by fault 6's fix is too strong.
12. Two detection classes named by the spec have no case: V7's FAIL header
    sub-case (t0-p6c/negative.log:639, with its own disable test at :778) and
    V9's compound-file signature (t2-p0/negative.log:518). Neither omission is
    listed in the report.
13. V8's sourcing is presented as more strained than it is.
    t0-p6c/negative.log:680-697 records the same fixture with all three FAIL
    lines and "FAIL lines: count=3"; it is cited nowhere. The expectation is
    right; the report's judgement-call framing at report:78-81 rests on a premise
    that line contradicts.
14. Count components not in any source: H1 expects 2, H2 expects 3, V17 expects
    4; no log records a hygiene office count above 1. The wording is sourced, the
    count is not, and the log does not distinguish the two halves. Separately,
    V11, V13 and B1 assert less than their sources recorded, so a reworded gate
    message would pass.
15. self-test.log's header is weaker than its three siblings: the commit is on
    line 3, and there is no phase name, no run timestamp, no end timestamp, no
    total wall time and no printed invocation.
