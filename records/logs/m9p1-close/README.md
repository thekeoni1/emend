# M9 part 1 close-out logs

specs/0051-msrv.md, "Close-out", and planning's ruling L: one push of
main, no tag, on planning's word and the maintainer's authorisation
typed in the implementing session ("push main no tag"); the ci and
msrv runs of the pushed head read by gh run view with --json; the post
scan. The first post run failed (a finding, below); the second, with
the script fixed under ruling L, is all as expected.

| File | What it holds | sha256 |
|---|---|---|
| pre.log | the pre mode, 2026-11-14, before the push | 791134396787a21176be78e01fc28702fc802574658405805d2569239f9579b2 |
| post-run1.log | the first post mode run, 2026-11-23 (the finding) | f272a1fe64dff8f1a7bbe9c18d28cadd92d38895432ebdb3670c399a56b127cb |
| post-run2.log | the second post mode run, 2026-11-23/24 (ruling L) | 3fcf54b4ec148e7afeb87f714edf766de64e84615efa57d1f162d0cef7503fc0 |
| close-run1.sh | the script that wrote pre.log and post-run1.log | 90c9eaa4a465390a0f1d10c86f4dd67213bb57c6cd9c2d158c22146582d6c650 |
| close.sh | the final script, which wrote post-run2.log | 892cc1c3bc077a3b8a2860cf96ab0f72a68622c37ebe788f270ee74f228ec81d |
| close-run1-to-run2.diff | diff -u close-run1.sh close.sh | abe7330e4b044baa6e17d7c49d2b6b982875e4988572bcbff1a32fefb80962dd |

Both script versions are committed, with the diff between them. In the
phase scratch the logs are pre/pre.log, post/post.log and
post2/post.log; here post.log of each run is renamed post-run1.log and
post-run2.log. Each log prints the script it ran, in full, and its
sha256: pre.log and post-run1.log print
90c9eaa4a465390a0f1d10c86f4dd67213bb57c6cd9c2d158c22146582d6c650,
post-run2.log prints
892cc1c3bc077a3b8a2860cf96ab0f72a68622c37ebe788f270ee74f228ec81d.

## The pre mode (pre.log)

Run once, 2026-11-14T18:43:17Z, over 2da5eb9..87b4745 (9 commits). Last
line before the exit status: "pre log checks all as expected: yes".
Privacy: "privacy: 9 commit(s), 3140 blob(s) scanned, 0 finding(s)".

## The push

Once, main 2da5eb9..87b4745, 9 commits, no tag. The push's own output
was not saved to a file; its result is in both post logs: origin's main
is 87b4745d43883f1e39597c1b6edd9ac38478a450 and the remote's tags are
v0.1.0 and v0.1.1 alone.

## The first post run (post-run1.log): a finding

Run once, 2026-11-23T23:51:43Z: "checks and controls not as expected:
4", "post log checks all as expected: no", "script exit status: 1".
Cause: jq not installed on the host. The four lines not as expected
are "it lists one run: no" and "a ci run id was read: no", and the
same two for msrv; gh run view never ran. The log shows
"<phase>/scripts/close.sh: line 241: jq: command not found" and the
same for line 242.

Defect of the first script, fixed in the second: two controls passed
untested, because the reader printed nothing and each control only
asked for "not success" or "not 0":

- "control: the conclusion test refuses a planted failure"
- "control: the job test refuses the same fixture's failed job"

The remote checks had the same weakness in two controls, which ran in
pre.log and post-run1.log; both are fixed in the second script:

- "control: the same comparison refuses an all-zero id"
- "control: the same comparison refuses the list with a third tag added"

Not rerun under the same script. Ruling L: no jq install; the script
reads JSON with python3 -I -B from the saved gh output; a reader that
exits non-zero is a check not as expected; a tool guard refuses to
start when a tool the mode calls is missing; the controls compare exact
values; the second run writes its own directory, post2. See
close-run1-to-run2.diff.

## The second post run (post-run2.log)

Run once, 2026-11-23T23:59:58Z, "bash <phase>/scripts/close.sh post
87b4745d43883f1e39597c1b6edd9ac38478a450 post2":

- origin's main is 87b4745d43883f1e39597c1b6edd9ac38478a450; the
  remote's tags are v0.1.0 and v0.1.1 alone.
- the range 2da5eb9..87b4745: 9 commits, a straight line; privacy
  "privacy: 9 commit(s), 3140 blob(s) scanned, 0 finding(s)".
- ci run 45256045379: conclusion success; jobs "check: completed
  success".
- msrv run 45256045539: conclusion success; jobs "msrv: completed
  success".
- "checks and controls not as expected: 0", "FINDING lines: 0", "post
  log checks all as expected: yes", "script exit status: 0".

## Known weak controls of the pre mode

Listed under ruling L 4, accepted, not fixed (the pre mode is not
rerun). Each passes on an empty or missing input:

- "control: the same test refuses one status line": "the fixture
  ctl-status was written as meant" ran before it.
- "<commit> message: no U+2014" and "<commit> message: no non-ASCII
  byte": they pass on an empty or missing message file. They have no
  fixture; the message each one tests is printed in pre.log under the
  "$ git log -1 --format=%B <commit>" line above it.
- "control: the trailer test refuses a message with a Claude-Session
  line" and "control: the trailer test refuses a message with two
  Co-Authored-By lines": "the fixture ctl-session was written as meant"
  and "the fixture ctl-twoco was written as meant" ran before them.

Rule for the next close script: no control may pass on an empty or
missing input.

This record commit is local and goes with the next push.
