# M9 part 2 close-out logs

specs/0052-launch-prep.md, "Close-out": one push of main, no tag, on
planning's word and the maintainer's authorisation typed in the
implementing session ("push main no tag"); the ci and msrv runs of the
pushed head read by gh run view with --json; the post scan.

| File | What it holds | sha256 |
|---|---|---|
| pre.log | the pre mode, before the push | a905e05f82e65041ab168e85f9af14568a999f740cff6d7af7713dcf8fb345d3 |
| post.log | the post mode, after the push and both runs | 8910c0124e01af09997ca6e435e7ef746ea8759ae79c96df9304fdd903934791 |
| close.sh | the script that wrote both logs | 8fe07425301c06d318c9a42da759dc9af812ac020f5b5e9b816d3bfe0b51eca0 |

Both logs print the script they ran, in full, with its sha256
8fe07425301c06d318c9a42da759dc9af812ac020f5b5e9b816d3bfe0b51eca0. The
script is built from part 1's final close script
(records/logs/m9p1-close/close.sh). The changes: the range and its
commit list; the tool guard refuses an empty tool list, with a control
in each mode; and the pre mode's status, message and trailer controls
fail on an empty or missing input (the three kinds part 1's record
lists).

## The pre mode (pre.log)

Run once, 2026-11-24T01:01:55Z to 01:03:26Z, on HEAD 7cf1b10. "the
range holds 5 commits: a031c83 e37b276 3adfeb1 b4f38ea and HEAD";
"privacy: 5 commit(s), 3146 blob(s) scanned, 0 finding(s)"; "PASS
hygiene"; origin's main 87b4745d43883f1e39597c1b6edd9ac38478a450; the
tree clean. "checks and controls not as expected: 0", "FINDING lines:
0", "pre log checks all as expected: yes".

## The push

Once, main 87b4745..7cf1b10, 5 commits, no tag. The push's own output
was not saved to a file; its result is in post.log.

## The post mode (post.log)

Run once, 2026-11-24T01:18:14Z to 01:19:49Z, after both runs had
completed:

- origin's main is 7cf1b10bce628297cc1ae8a51827e06a22df5bc8; the
  remote's tags are v0.1.0 and v0.1.1 alone.
- the range 87b4745..7cf1b10: "privacy: 5 commit(s), 3146 blob(s)
  scanned, 0 finding(s)".
- ci run 45303751433: conclusion success; jobs "check: completed
  success".
- msrv run 45303751470: conclusion success; jobs "msrv: completed
  success".
- "checks and controls not as expected: 0", "FINDING lines: 0", "post
  log checks all as expected: yes", "script exit status: 0".

This record commit is local and goes with the next push.
