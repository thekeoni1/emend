# Docs intro close-out logs

specs/0050-newcomer-intro.md, the close-out: one push of main, no tag,
on planning's word and the maintainer's authorisation typed in the
implementing session ("push main no tag").

| Log | What it holds | sha256 |
|---|---|---|
| pre.log | the pre checks before the push, written by close-pre.sh | 32f73309fc93c1777e4c567190e8ee7af68e854913ade803e69efb1e67d2ca65 |
| push.out | the push and the remote ref after it | f612c6709eadd3ef5aa6cb9378dac5bf08dfb44f7c6ffffe7d7aa9211b703849 |
| post.log | the post scan | 67c46f13a63f086db607d3fce0a4372bc961f81edbd7050b324e41e17ed7d663 |

close-pre.sh (phase scratch, not committed) has sha256
1b180dc8b01f77360a2e23495fba68c2e4b7566be75094a54b9835f33250d4ed.
pre.log's last line is "pre log checks all as expected: yes". An
earlier pre run (pre-1.log, kept in the phase scratch) was vacuous on
its U+2014 check under LC_ALL=C; its control caught it, the check now
counts the byte sequence with git grep -c -F, and pre.log is the run
after that fix.

## The push

- Once, 2026-11-14T00:41:20Z: main 635aaba..2da5eb9, 11 commits, exit 0.
  The pre-push hook ran scripts/privacy.sh over the range: 11 commits,
  3129 blobs, 0 findings. No tag was pushed.
- CI: run 43536026856, event push, head
  2da5eb94594ae4307766c641e6f35b1b25cb3d69, completed success (job
  check).

## The post scan

- Remote main is 2da5eb94594ae4307766c641e6f35b1b25cb3d69 (control:
  it is not 635aaba).
- Remote tags: v0.1.0 and v0.1.1 only.
- scripts/privacy.sh --range 635aaba..2da5eb9: 11 commits, 3129 blobs,
  0 findings, exit 0.

This record commit is local and goes with the next push.
