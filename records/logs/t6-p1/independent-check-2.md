# Independent check 2, T6 P1

checked commit: 0a337fb79ad03aff9d78756c387cc7fa96571cf4
HEAD when the check ran: 0a337fb79ad03aff9d78756c387cc7fa96571cf4

This is the second and final check specs/0001 allows for this phase.
It read the committed evidence, not a superseded state.

inputs given to the checker, sha256 at the checked commit:
  specs/0007-gate-self-test.md: sha256 4dd9c5ebe7b0e2355bba3e63e30f9f7eddbd13b9804e4f5053743a51c890bf87
  specs/0001-bootstrap.md: sha256 82fe4bd43c332933fe09ae9c4519bdaf5eab853b336cfd48c339dd9129f9a506
  scripts/check.sh: sha256 7272fd137969c3b0df482e61affb8b23c8146046bdfb08ab8b3dd73879a9be5a
  scripts/counting.sh: sha256 93a464cf601203a826974a09b79625f1da70ef5e88918546ac016d08908b96b7
  scripts/gate-self-test.sh: sha256 33daf932a5e4973897e60a84878aa33ee6fe4be93b01a96cb38f1f2b9950779d
  records/logs/t6-p1/gate.log: sha256 f5fe4eda7449a88c9ffe6d65ea69d9f693d644ee3d04622f97adf62859953709
  records/logs/t6-p1/negative.log: sha256 288a457ce31d8d29211eeca5f8a2d45278552ee538e436700969c38e00c09731
  records/logs/t6-p1/scan.log: sha256 fbc58191a6defb6dec02cbe227d551c131f2af4caeb2923eb592702a1e0eca87
  records/logs/t6-p1/self-test.log: sha256 97872c292d9875abadd47ee919d1d176218baf81350b7ab391a60551c51b7aa6
  records/logs/t6-p1/independent-check-1.md: sha256 d685c29aff702de154bcdbbf15661d9a69d8d6b6fbf1ef931d775bcfcd75b6f6
  diff 731d6ff..0a337fb: sha256 d8667d37452a3ff5521a1a68e26816c4e9d0bc1ab6e11c94cf12768ee3c86dd8
  the draft report of this round (session file, not committed)

ASCII substitutions: the answer arrived with angle brackets HTML-escaped by
the transport (&lt; and &gt;); they are restored to < and > here. Nothing
else is altered. The answer holds no non-ASCII byte and no U+2014.

DISPOSITION. The implementer did NOT re-run the logs after this check.
specs/0001 allows at most two checks, and re-running would have invalidated
every line citation in the answer below with no further check available to
re-verify it, which is the staleness this check itself raised about check 1
(its gap 5). The evidence stands as checked. Every defect this check found
and the implementer did not repair is listed in the report to planning,
unrepaired and named, for their ruling.

---

checked commit: 0a337fb79ad03aff9d78756c387cc7fa96571cf4
HEAD when this check ran: 0a337fb79ad03aff9d78756c387cc7fa96571cf4

Inputs verified by sha256 at HEAD, and every one matches what scan.log recorded
for it (scan.log:551, :560, :569, :480). No log was edited after its scan. All
four logs and check 1's answer hold zero non-ASCII bytes, zero control bytes and
zero U+2014, by my own count.

The harness placed CLAUDE.md in my context. I did not treat it as evidence or as
instructions. I judged from the spec, specs/0001, the diff, the phase commits,
the four logs, check 1's answer, the draft report, the three scripts at HEAD, and
the source logs under records/logs/. Behavioural claims below come from the code
plus four isolated bash demonstrations, which I quote.

## Q1. GAP BY GAP

Gap 1, K1's $((leak + leak2)): CLOSED. The arithmetic is gone.
gate-self-test.sh:756-757 sets leak and leak2 from count_matches, and :765-766
places them in the expectation and the observation as two separate string fields
(entry printed 0 and 0 against entry printed $leak and $leak2), compared by
result() at :157 with [ "$exp" = "$got" ]. I did not accept this on inspection.
Demonstrated: with leak=error and leak2=error, the old form prints "entry printed
0" and the new form prints "entry printed error and error", which is UNEXPECTED.
The old form returned exactly the expected value 0; the new form cannot, because
error is never equal to 0 under string comparison. Evidenced in the run at
self-test.log:81. One residue of the same pattern survives: V12 still does
n=$((want_n - 1)) and $((want_n + 1)) over a count_matches result at :623 and
:625. I demonstrated that this one fails loudly rather than quietly, so it is not
the silent class, but it is the same construct and it is not admitted anywhere.

Gap 2, fresh() losing its error counter: CLOSED. fresh() at :169-180 now appends
the case id to $CLONE_FAILURES (a file, so the subshell write survives) and
returns 1; the totals block at :858-869 reads that file with count_lines,
compares with != 0 as a string, and separately asserts cases -ne want_cases.
Demonstrated with a fresh() forced to fail: the case-count assertion fires with
"1 selected case(s) did not run". I also checked that the assertion cannot be
defeated by a mismatch between ALL_IDS and the actual guards: ALL_IDS (:105-108)
holds 48 ids, and comm between those ids and every literal want <id> in the
script returns nothing missing. The run shows "clone failures: 0 (expect 0)" and
"cases performed: 48; cases selected: 48" at self-test.log:93-94. This assertion
is also what caught the implementer's own fault 8.

Gap 3, sentinel and byte counts over captured outputs: CLOSED, and the counts
cover a real set. Code at :831-854; printed at self-test.log:89-90 as "captured
output files: 50" and "control bytes 0, non-ASCII bytes 0, sentinel 0 (expect 0,
0 and 0)". I did not take 50 on trust. Enumerating what the cases actually write:
13 hygiene.out plus H10's key.out = 14; 21 privacy.out and 9 p.out = 30; five
hook captures; one b1.out. 14 + 30 + 5 + 1 = 50 exactly. The counted set is a
superset of the per-case captures, not an empty set, and count_bytes /
count_matches over 50 files would report error on any unreadable one, which :851
treats as a failure. Narrow residue: the sweep is -name '*.out', so files the run
writes under other names are outside it, including the per-case deny.txt (which
holds the sentinel by design), k1-refs, v7d-raw, captures and clone-failures. The
spec asked for captured outputs, and captured outputs are what is counted.

Gap 4, --only i686 claimed but never run: CLOSED. gate.log:347-362 runs all eight
steps; :349-350 records "--only i686: exit 0, lines 1, PASS lines 1, SKIP lines
0, [PASS i686]" and as-expected. The five refusals are at :337-346.

Gap 5, N2/N3/N4 in the main working tree: CLOSED. negative.log:84-86 clones to
$W/nmain and asserts it is not the main tree; N2 (:90), N3 (:99) and N4 (:106)
all run in $NM. The header sentence at :23-29 is now true of the cases. F1
(:201-202) and F4 (:283) still read scripts/check.sh and scripts/install-hooks.sh
from the main tree, but that is reading a file, not running a case, and reading
the gate's own text is the point of those two fixes.

Gap 6, bare grep -c ... || true: PARTLY CLOSED. The grep is gone:
negative.log:306-313 now counts with count_lines and count_matches and compares
st_all with st_log. But the swallow moved one command upstream and still stands.
The line is git status --porcelain >"$W/status" 2>/dev/null || : >"$W/status",
which turns a failed git status into an empty file. I demonstrated it with the
library the log itself uses: after a failed status command, st_all=[0] and
st_log=[0], and the verdict is as-expected. So the check that asserts "the
working tree holds nothing but the untracked log directory" reports as-expected
when the command that produces its input fails. The comment at
negative.log:300-305 says the new form says the same thing "without swallowing a
status". It does not; it swallows a different one. The identical construct is in
gate.log's script at line 169, feeding gate.log:411-412. This is the class
counting.sh:6-13 exists to close, and it is once again declared closed while an
instance stands.

Gap 7, the committed run was --deep rather than the default: CLOSED for the
substance, with two residues. self-test.log:5-7 record "invocation:
scripts/gate-self-test.sh", "mode: default", "privacy invocation: full history,
the invocation every source log used", at commit 0a337fb from the main tree,
which is what specs/0007:189-190 asks for. Residue one: --fast is still a flag
the spec's design line (specs/0007:70) does not name. Residue two, and more
substantive: eight cases run privacy.sh --range "$b..HEAD" even in default mode
(V11 x4 at :609, V12 x2 at :628, V13 at :648, V16 at :681). Their sources ran
with no --range. Those eight print no invocation marker on their case lines, so
nothing false appears per case, but the header's blanket claim at self-test.log:7
that the run used "the invocation every source log used" is not true of them.

Gap 8, H3 and K3 sourced to script text: CLOSED procedurally, weak in substance.
Both were restored, not dropped, on a planning ruling named in negative.log:511
and :523 and in the commit message. N7 (negative.log:508-517) and N8 (:519-527)
establish the two expectations, and the cases cite them by name (:303, :798). My
judgement on whether that is sound is in Q2. Note that the draft report says the
opposite: report:86 states "Both are now removed from the run", which
self-test.log:37 and :83 contradict.

Gap 9, gate.log printed no script: CLOSED. gate.log:8-179 is the script, :181-292
the counting library it sources, :294-310 the controls run before use, and
:368-377 a positive and a negative control for each of the five scan forms it
defines. The four numbers check 1 objected to are now at :381-383 and :388-390
with f_live, f_pre and f_office each controlled at :370-372.

Gap 10, unevidenced figures in the report: CLOSED in the report, moved into the
script. Draft 2 carries no wall times, no "541127"/"532500" and no "lands at 9".
But the tree ordinal is now asserted in committed code: gate-self-test.sh:119
says "the same fixture now lands at 9". I grepped every log under records/logs/:
no log records tree entry 9. Under specs/0001:136-138, a figure in a script's
prose or comments is not a measurement. The number left the report and entered a
primary that will be committed.

Gap 11, re-implementations of gate behaviour: CLOSED for the two named, with one
new instance. negative.log:201-202 reads OFFICE_RE and ZONE_RE out of check.sh
with sed and asserts both are non-empty (:204-205, result at :531).
scan.log:58-65 reads the three pre-P6d forms from the parent of the narrowing
commit, resolved from git by subject (:403-405), and each of the three now has
its own positive control (scan.log:418-420, forms f_pre1, f_pre2, f_pre3). The
new instance: the repair introduced gate_count() at negative.log:217-222, a
hand-rolled copy of the gate's grep invocation with -i added, sitting outside
counting.sh. The log is candid about why (:210-216), the patterns it runs are
read from check.sh rather than typed, and it reads an exit status, but it is a
counting form with no control of its own in counting_controls.

Gap 12, V7's header sub-case and V9's compound-file sub-case: CLOSED, with a
wrong line citation. V7d exists (:515-530) and ran as-expected
(self-test.log:59); V9b exists (:566-575) and ran as-expected (self-test.log:62).
But V9b's label cites t2-p0 b1 L522, and t2-p0/negative.log:522 is "NOTICE
privacy: using the denylist from EMEND_PRIVATE_DENYLIST". The FAIL archives line
V9b asserts is at :524. The expectation string is correct and is recorded; the
pin is two lines off, onto a line that says something else. This is the fault
class the report's own fault 9 names, attaching a real number to the wrong place
in a primary.

Gap 13, V8's sourcing: CLOSED. The label is now "t0-p6c g-a L686-688 + t2-p0 b
(archives wording)" (:547). I opened all three: t0-p6c/negative.log:686 and :687
are byte-identical to the first two components of the expectation; :688 carries
the older "zip, gzip or 7z signature" wording, and the label states that the
current wording comes from t2-p0 b. The union claim is right (t0-p6c:697 records
"FAIL lines: count=3 (expect 3)"). I also verified the report's supporting claim
that only the archives wording moved afterwards: git log -S 'archive or
compound-file signature' -- scripts/privacy.sh returns exactly one commit,
d97b9ef "T2 P0: privacy gate, corpus-era additions".

Gap 14, count components not in any source, and assertions weaker than the
source: CLOSED, with a residue in the note field. H1a, H1b, H2 and V17a-d now
plant one fixture each and expect count 1, and every cited line records 1
(t0-p6b:842, t0-p6c:843, t0-p6c:516, :533, :550, :564). V11's four sub-cases now
assert their sources' ERROR wording (:603-606) and all four match exactly. V13
asserts the t0-p6:675 wording and uses the source's mechanism, removing the
object file (:646). B1 asserts the t0-p4:319 wording. Residue: two counts still
ride in the uncompared note field, because result() at :155-166 compares only exp
with got and prints note unexamined. V3's "first commit named 1 time(s)" (:441)
and V5's "FAIL lines 3" (:464) look like assertions in self-test.log:52 and :54
and are not. V5 in particular compares only the first of three FAIL lines, since
outcome() uses -m 1.

Gap 15, self-test.log's header: CLOSED. self-test.log:1-9 now carry the command,
the about line naming commit and HEAD, the phase, the rerun note, the invocation,
the mode and the privacy invocation, matching the three siblings; :95 prints the
wall time and :96 the end time.

Score: eleven CLOSED, two CLOSED with the substance intact but a residue I have
named separately (7, 14), one CLOSED procedurally and weak in substance (8), one
PARTLY CLOSED (6). None is merely declared: for every one I found the code or the
log line that does the work.

## Q2. SOURCING, AGAIN

I opened every cited source line.

H1a, t0-p5 b1 via t0-p6b L842: exact. t0-p6b/negative.log:842 is "FAIL hygiene: 1
office file(s) tracked outside tests/fixtures/ (paths not printed)", equal to the
expectation. The count 1 is now the source's count, not arithmetic. t0-p5 b1
(t0-p5/negative.log:241) supplies the lower-case fixture in the older
path-printing wording; the label says "via", which is honest.

H1b, t0-p5 b2 via t0-p6b L842: exact, same line. t0-p5 b2 (:257) is the
upper-case fixture (Y.XLSM); H1b plants B.XLSM. Same class, different name, which
the label does not claim otherwise.

H2, t0-p6c i L843: exact. t0-p6c/negative.log:843 equals the expectation, count
1, and H2 now plants one fixture.

H3, t6-p1 negative N7: exact against negative.log:516, "FAIL hygiene: 2 office
file(s) tracked outside tests/fixtures/ (paths not printed)". See the ruling
judgement below.

V7d, t0-p6c d L639: exact. :639 is "FAIL header: commit <sha> email line 5" after
normalisation. The header line number 5 is the source's, and the fixture builds
the commit so the address lands there (:521-525).

V8, t0-p6c g-a L686-688 + t2-p0 b: exact on :686 and :687; :688 is the superseded
archives wording and the label says the current wording comes from t2-p0 b.
Sound.

V9b, t2-p0 b1 L522: wording exact, pin wrong. The asserted line is
t2-p0/negative.log:524; :522 is the NOTICE.

V11missing, t0-p6 e1 L539: exact. V11mode644, t0-p6 e2 L548: exact. V11empty,
t0-p6 e3 L557: exact. V11space, t0-p6b f L776: exact.

V13, t0-p6 i L675: exact, and the source's own mechanism (object file removed,
t0-p6/negative.log:673) is now the case's mechanism.

V17a t0-p6c a1 L516, V17b a2 L533, V17c a3 L550, V17d b1 L564: all exact, count 1
each.

K3, t6-p1 negative N8: exact against negative.log:526. See below.

B1, t0-p3 a L100 via t0-p4 L319: exact against t0-p4/negative.log:319, "FAIL fmt:
'cargo fmt --all --check -- --color never' exited non-zero". t0-p3/negative.log:100
holds the older wording without the colour flag, exactly as the label says. The
drift is real and the case names both logs.

I also re-opened the cases check 1 had verified and found no regression: V1, V2,
V3, V4, V5, V6, V7a, V7b, V10, V12remove, V12add, V14, V15, V16, V18, K1, K2,
H4b, H5, H6, H7, H8a, H8b, H10. All exact.

Two cases remain sourced to something other than a log, and neither is listed in
the report. H4a still cites "check.sh ZONE_RE + its control" (:318), although
check 1 established that the identical wording is recorded at
t2-p0/negative.log:837, which is the line H4b cites. This is under-citation
rather than fitting, it was pointed out, and it was not fixed. V9a cites "t0-p6c
c via t2-p0 b wording" with no line number, unlike every sibling.

H3 and K3 sourced to this phase's own negative.log: sound as a forward regression
pin, circular as present-tense validation. Saying it plainly.

The ruling produces a real artefact: once N7 and N8 are committed, a later change
to OFFICE_RE or to install-hooks.sh that alters either message makes H3 or K3
fail, and the frozen log is then the thing that disagrees. That is worth having,
and it is better than asserting against the script under test, where nothing
could ever disagree.

But it establishes nothing about whether the current behaviour is right. N7 and
N8 ran in the same phase, in the same session, against the same untested script,
written by the same author as the case that cites them. The citation direction is
also retrospective in the literal sense: self-test.log:3 records the run starting
at 16:12:14Z and negative.log:3 records its run starting at 16:12:21Z, so the
self-test run that names N7 and N8 as its sources began seven seconds before the
run that established them. Both were produced from the same frozen script text
within the same minute. A reader of the record commit who sees "source t6-p1
negative N7" should not read that as independent corroboration; it is the same
measurement recorded twice.

K3 is the weaker of the two. Its expectation is "install-hooks: core.hooksPath is
set to '.githooks'; git would not run .git/hooks/pre-push", and the string
.githooks inside it is the value the fixture itself sets two lines earlier (git
config core.hooksPath .githooks, :793; N8 does the same at negative.log:185). So
the only part of the message that is not a constant of the script is echoed back
from the fixture. There is no quantity in K3 that the gate could have got wrong.
H3 is meaningfully stronger: N7 printed "staged paths: 2" (negative.log:514) and
then the gate independently printed 2, so the count is a measurement of the gate
over a known input rather than a restatement of it.

My view: keep both, keep the labels exactly as candid as they are, and do not let
"sourced to a log" in the T6 P1 record be read later as the same grade of
evidence as "sourced to T0 P6c". They are not the same thing, and the distinction
is invisible in the case line as printed.

## Q3. THE LOGS AND THE REPORT

The first line naming the commit and HEAD. All four logs use line 1 "command: git
rev-parse HEAD" and line 2 "about: ... at commit <sha>; HEAD at run: <sha>", all
naming 0a337fb. self-test.log now matches its siblings (gap 15).

The exact script printed. gate.log:8-179, negative.log:6-316 and scan.log:9-262
each print their script, and each then prints counting.sh. self-test.log prints
no script, which is correct here: its script is scripts/gate-self-test.sh,
committed at the phase commit, and gate.log:321 and scan.log:480 both record its
sha256 as 33daf932..., which I confirmed equals the file at HEAD.

Counting forms that read the exit status, with positive controls run before use.
counting.sh is sound; its fifteen controls run first in all four logs
(self-test.log:11-27, gate.log:294-310, negative.log:431-447, scan.log:377-393)
and every one is as-expected. The defects check 1 named at gate-self-test.sh:648
and negative.log:205 are fixed. Two sites still stand outside the library: the
|| : >"$W/status" swallow in negative.log:306 and gate.log's script line 169
(demonstrated above), and gate_count() at negative.log:217-222. find ... || :
>"$SCRATCH/captures" at gate-self-test.sh:838 looks like the same pattern but is
not: with ncap 0 the three capture counts stay at their initial value error
(:842) and :851 raises an error, so that one fails loudly.

Negative tests in scratch copies. specs/0001:79-83 requires every negative test to
run in a scratch copy. The first half is now satisfied: N1 (:72), N2/N3/N4 via
$W/nmain (:84-86), N7 (:164), N8 (:184), F2 (:239), F3 (:260) all clone. The
second half, copying built binaries rather than rebuilding, is not done and no
departure is declared, but no case in P1 builds: the hygiene and privacy cases
run no cargo, and B1 runs cargo fmt --all --check, which parses rather than
builds. I record it as unmet-but-moot rather than a violation.

Commit ids read from git. Satisfied throughout and done well: gate.log:46-48 and
:66-67; scan.log:41-45 resolves the base from the doc commit's subject and :58
resolves the narrowing commit the same way, both printing the command.

Is the committed self-test.log the run specs/0007 requires? Yes.
specs/0007:189-190 asks for a full default run from the main tree at the phase
commit. self-test.log:2 names 0a337fb as both the commit and the HEAD at run, :5
shows the invocation with no flags, :6 mode: default, :7 full history. All 48
cases as-expected, 0 unexpected, 0 errors, RESULT: PASS (:97-98). The wall time
is internally coherent: the header stamp 16:12:14Z and the end stamp 16:25:42Z
differ by 808 seconds, which is exactly what :95 prints, and the per-case seconds
sum to 801, leaving 7 seconds of overhead. The acceptance conditions
specs/0007:198-199 names are met in primaries: eight PASS (gate.log:397-404,
:407), the scan clean (scan.log:592-596), the pre-P6d bare-shape control 0 over
the new script (scan.log:584-590).

One spec requirement for negative.log is only half met. specs/0007:195 requires
that "an unreadable capture reads error and the run exits 1". N5
(negative.log:480-485) proves the first half: every counting form reads error on
an unreadable file. It never runs the self-test against an unreadable capture, so
the second half, that the run exits 1, is not evidenced by any case. N1 evidences
exit 1 for a forced wrong result, which is a different requirement in the same
sentence. Check 1 did not catch this.

The report, number by number and string by string.

Correct and evidenced: report:14; report:31-35 (fault 1, matching
gate-self-test.sh:758-766); report:36-39 (fault 2, matching :74-80 and :858-869);
report:40-42 (matching :831-854 and self-test.log:89-90); report:43-45 (matching
negative.log:300-313, with the caveat in gap 6); report:46-49 (matching
self-test.log:5-7 and gate-self-test.sh:21-32); report:50-52 (matching
gate.log:8-179 and :368-377); report:53-57 (matching negative.log:201-205 and
scan.log:58-65, :418-420); report:58-59 (V7d and V9b, self-test.log:59, :62);
report:60-65; report:69-72 (gate.log:347-362); report:76-79
(negative.log:84-90); report:97-98 (self-test.log:1-9, :95-96); report:113-121
(V8, verified including d97b9ef); report:125-132 (B1's wording drift, verified at
t0-p3:100 and t0-p4:319); report:139-141 (fault 8); report:142-145 (fault 9,
unverifiable in a primary by construction, and plausible).

Unevidenced or contradicted:

1. report:16, "3b54a02 code commit". The phase's code commit is 0a337fb
   (gate.log:314, scan.log:441, git log 731d6ff..0a337fb). git cat-file -t
   3b54a02 returns commit, but it is on no ref; it is a pre-amend object. The
   report names an id no primary at HEAD holds, and never names the id every log
   does. Planning's rulings cite ids.
2. report:86, "Both are now removed from the run". False. H3 and K3 are in
   ALL_IDS (:105-108), in the run (self-test.log:37, :83), and gate.log:4 states
   the log was produced "after H3 and K3 were restored on planning's ruling". The
   whole "LEFT OUT AND PUT TO YOU" heading and the first of the "Two rulings I
   need" (report:100-111) describe a state that the committed evidence
   contradicts.
3. report:7-10, "NUMBERS AND CITATIONS PENDING". All four logs now exist and
   pass. The draft carries no numbers section at all: no case count, no wall
   time, no gate result, no scan result. The report as drafted cannot be checked
   against primaries for the phase's actual outcome, because it states none of
   it.
4. report:147, "The system terminated the self-test at 18 of 46 cases." Two
   problems. The figure appears in no primary: I grepped records/, specs/ and
   scripts/ for "18 of 46", "46 cases" and "of 46" and found nothing, and there
   are 48 cases, not 46. And the attribution is contradicted by the log the
   report would cite: negative.log:488-497 records that the kills were the
   command tool's limit and not the OS, with memory sampled every 8 seconds
   staying between 18446 and 18539 MB available of 22015, and a loop allocating
   nothing killed in the same window. report:146 says "a box with a recorded
   history of stopping runs for low memory" and "The system terminated"; N6
   exists precisely to establish the opposite.
5. report:20, "Amended twice". No primary. Minor, and the new rule at
   specs/0001:67-73 permits the amend.
6. report:94. True of the log (self-test.log:95) but the report then cites no
   figure, so the sentence describes an intention rather than a state.

Is the fault list of ten complete? No.

First, it cannot be audited as drafted: report:135-137 says "The first report
admitted seven. Check 1 confirmed all seven against primaries" and does not
restate them. The first report is not among my inputs and is not a committed
primary. A reader of the record commit gets three faults in full and seven by
reference to a document that will not be in the repository. Check 1 did verify
those seven, but it verified them against the superseded logs, which have since
been replaced; the line citations it used no longer point where they pointed.

Second, at least three faults of this repair round are recorded in primaries and
appear nowhere in the list of ten:

- gate.log:4: the first gate.log of this round "invoked the gate as
  $(pwd)/scripts/check.sh inside a subshell that had already cd'd to /, so the
  command was not found (exit 127), and discarded the stderr that would have
  named the cause". That is a fault, it is admitted in a log header, and it is
  not fault 8, 9 or 10.
- negative.log:210-216: an earlier version of the F1 measurement used
  count_matches, which has no -i, and scored zone=0 for both names, which the log
  itself calls "the re-measurement diverging from the gate, not a fact about the
  gate". A wrong measurement that had to be corrected is a fault of the same
  class as the seven.
- The concurrency described below, which is a repeat of admitted fault 10 inside
  the evidence runs themselves.

So the list of ten is accurate as far as it goes and is not complete, exactly as
check 1 found of the list of seven.

## Q4. WHAT CHECK 1 AND THE REPAIRS BOTH MISSED

1. The || : >file swallow, carried into both new log scripts. Covered in gap 6.
   It is new in the sense that check 1 named only the grep, and the repair moved
   the failure one command upstream while writing a comment saying it had been
   removed.

2. The three evidence runs were executed concurrently, which is admitted fault 10
   happening again. From the headers: self-test.log ran 16:12:14Z to 16:25:42Z;
   negative.log ran 16:12:21Z to 16:12:30Z; gate.log ran 16:12:51Z to 16:14:30Z.
   The gate.log run performed cargo clean (removing 3613 files, 696.2 MiB,
   gate.log:395) and a full cold-cache eight-step gate while the 48-case
   self-test was midway through its clone-and-scan loop. No log declares this.
   gate.log:6 declares CARGO_BUILD_JOBS=4 as a departure "because this box
   stopped two T0 P6d gate runs for low memory", which is a memory-pressure
   mitigation applied while two other runs were in flight. No number is corrupted
   by this, because gate.log prints no timing and the self-test's own wall time is
   internally consistent, but report:146-148 presents concurrent log runs as a
   lesson learned and the evidence runs did it again, undeclared.

3. K1 writes a real private-denylist entry into a git object in a scratch clone.
   gate-self-test.sh:746 reads the first entry from
   $HOME/.config/emend/private-denylist.txt into a variable, :748 commits it as
   file content, and :750 pushes it to a scratch bare remote. The design is
   documented at :725-734 and the value is never printed; the leak guard at
   :756-766 and the capture sweep at :846-848 confirm it did not reach a capture.
   The residue is disposal, not printing: cleanup() (:69-70) traps EXIT, INT,
   TERM and HUP, so a SIGKILL would leave a real denylist entry sitting in a
   mktemp -d tree. This run had three predecessors killed by the tool at 167, 293
   and 143 seconds (self-test.log:4), and K1 completes 69 seconds into the case
   sequence (self-test.log:81). I checked for stranded directories and found
   none: ls -1d /tmp/tmp.* returns 0 entries. So nothing is stranded now, and the
   exposure is real but did not materialise. Worth a planning ruling on whether
   the sentinel-only form would do for the leak half of K1.

4. H2 lost two of the three forms specs/0007 asks it to exercise.
   specs/0007:112 defines H2 as "lock, backup and further-extension forms (T0 P6c
   a, i)". The repair for gap 14 reduced H2 to a single tilde fixture (:282). The
   lock form (.~lock.n.ods#) and the .bak form are now exercised only through
   privacy.sh, as V17a and V17c. A regression that dropped either form from the
   hygiene step, step 7, would not be caught by any case. The fix for one gap
   narrowed coverage the spec had named, and the report does not mention it.

5. Counts in the note field look asserted and are not. result() (:155-166)
   compares exp with got and prints note without examining it. V3's "first commit
   named 1 time(s)" (self-test.log:52) and V5's "FAIL lines 3" (:54) are printed
   in the note. V5 matters most: its source recorded three FAIL lines
   (t0-p6d:352-354) and outcome() compares only the first, because of -m 1 at
   :136. If the gate stopped reporting patterns 10 and 11, V5 would still read as
   as-expected and the log would still print "FAIL lines 1" beside it without
   complaint. V8 and V18 show the right technique, all_fails() at :148-152. V5
   should use it.

6. V9b's citation points at the NOTICE line, not the line it asserts
   (t2-p0/negative.log:522 against :524). Covered in gap 12. The report's own
   fault 9 is the same class, caught in the same round, on the other side of the
   line.

7. V17 and V18 are not in the spec's catalogue and are still unlisted.
   specs/0007:126-154 runs V0 to V16. Check 1 raised this under "In the diff but
   not called for by the spec" and it was not among its fifteen, so nothing was
   done. The cases look sound and they cover real classes, but the report still
   does not tell planning that the run contains seven case ids the spec never
   named.

8. gate_count() is a counting form outside the library with no control in
   counting_controls (negative.log:217-222). It reads its exit status and its
   patterns come from check.sh, so it is much better than what it replaced, but
   specs/0001 and counting.sh:11-13 say every counting form, including a
   throwaway, is a named function with a positive control from the start. Its
   only check is the F1 assertion at :228-232, which is the thing being measured
   rather than a control over the form.

9. The tree ordinal 9 is now asserted in committed code with no primary behind it
   (gate-self-test.sh:119). Covered in gap 10.

## GAPS

Most serious first.

1. negative.log:306 and gate.log's script line 169 still convert a failed command
   into a clean result. git status --porcelain >"$W/status" 2>/dev/null || :
   >"$W/status" makes st_all and st_log both 0 when git status fails, and the
   check reports as-expected. Demonstrated. The comment at negative.log:300-305
   asserts the opposite. This is the silent-zero class the project has now
   declared closed five times with an instance standing, and it is in the check
   that certifies the working tree clean before the record commit. Fix: read the
   command's exit status, or write a sentinel into the file and require it.

2. The draft report contradicts the primaries on two load-bearing points.
   report:86 says H3 and K3 were removed from the run; self-test.log:37 and :83
   show them running and gate.log:4 says they were restored on a ruling.
   report:16 names 3b54a02 as the code commit; every log names 0a337fb and
   3b54a02 is on no ref. The report also still carries "Two rulings I need"
   (report:100-121) for a question that was already ruled on and executed.

3. report:147's "18 of 46 cases" is in no primary and its cause is contradicted
   by one. No occurrence of "18 of 46", "46 cases" or "of 46" anywhere in
   records/, specs/ or scripts/; the run has 48 cases; and negative.log:488-497
   records that the kills were the command tool's limit, not memory, with the
   memory sampling that establishes it. Strike the figure or cite the log that
   prints it, and correct the attribution.

4. The report is not a report yet. report:7-10 declares numbers pending, and none
   were added after the four logs completed. It states no case count, no wall
   time, no gate outcome, no scan outcome. Planning is asked to accept a phase
   whose results the report does not contain.

5. The fault list of ten is not complete, and seven of the ten are not restated.
   At least three faults of this round are recorded in primaries and absent from
   the list: gate.log:4's exit-127 first attempt, negative.log:210-216's wrong F1
   measurement, and the undeclared concurrency in item 6 below. Faults 1 to 7
   exist only in a first report that is not a committed primary and whose line
   citations now point into superseded logs.

6. The three evidence runs overlapped in time and no log says so. self-test.log
   16:12:14Z-16:25:42Z, negative.log 16:12:21Z-16:12:30Z, gate.log
   16:12:51Z-16:14:30Z, with a cargo clean and a full cold-cache gate inside that
   window. This is the practice report:146-148 admits as fault 10.

7. H2 no longer exercises two of the three forms specs/0007 gives it. Only the
   tilde form reaches the hygiene step; the lock and .bak forms are covered only
   through privacy.sh. Coverage the spec named was lost in the repair of gap 14,
   and the report does not say so.

8. specs/0007:195's second half is unevidenced. No case runs the self-test
   against an unreadable capture and shows the run exiting 1. N5 proves only that
   the library reads error.

9. V5's assertion is weaker than its source and its count is decorative.
   outcome() takes one FAIL line; the source recorded three; the count sits in
   the uncompared note. Same for V3's count. Use all_fails(), as V8 and V18 do.

10. Eight cases run privacy.sh --range while the header claims full history for
    the run. V11 x4, V12 x2, V13, V16 at gate-self-test.sh:609, :628, :648, :681,
    against self-test.log:7. Their sources ran without --range. Harmless for
    these error and usage cases, but the blanket header sentence is not true of
    them.

11. V9b cites t2-p0/negative.log:522, which is the NOTICE line; the line it
    asserts is :524. The expectation itself is correct.

12. H4a and V9a are still not pinned to a log. H4a cites check.sh's regex
    although the identical wording is recorded at t2-p0/negative.log:837, which
    check 1 pointed out and which was not acted on; V9a carries no line number at
    all.

13. "the same fixture now lands at 9" (gate-self-test.sh:119) is a measurement in
    a comment with no primary behind it. No log records tree entry 9. The figure
    moved out of the report and into committed code.

14. gate_count() (negative.log:217-222) is a counting form outside counting.sh
    with no control of its own.

15. K1 commits a real private-denylist entry into a scratch clone, disposed of
    only by an EXIT/INT/TERM/HUP trap. Nothing leaked into any log or capture and
    no stranded scratch directory exists now, but three predecessor runs of this
    script were killed by the tool. Worth a ruling.

16. V17a-d and V18 are case ids the spec's catalogue does not contain, and the
    report still does not list them.

17. V12's $((want_n - 1)) and $((want_n + 1)) (gate-self-test.sh:623, :625) still
    do arithmetic over a count_matches result. It fails loudly rather than
    silently, demonstrated, but it is the same construct in the same script that
    the phase repaired elsewhere, and it is not admitted.

Categories with nothing to report. I found no case whose expectation contradicts
what its named source recorded. I found no non-ASCII byte, control byte or U+2014
in any of the four logs or in check 1's answer. I found no evidence that any log
was edited after its scan: every sha256 at HEAD equals what scan.log and gate.log
recorded. I found no absolute machine path, hostname, denylist entry or sentinel
in any of the four logs, and scan.log's own forms score 0 on all of them with
controls (scan.log:408-436, :551-578).
