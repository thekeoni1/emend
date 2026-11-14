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

## Addendum, 2026-11-14 (planning ruling H)

A code review found three post.log lines and one README name that no
logged command printed. Ruling H keeps the logs as committed; nothing
is rerun or edited. The commands below are taken from the implementing
session's transcript of the close-out.

- (a) post.log's two tag-name lines (refs/tags/v0.1.0 and
  refs/tags/v0.1.1) were not printed by the `wc -l` command shown above
  them. The script ran a second command right after it, with no `$`
  line of its own:

  ```
  git ls-remote --tags origin | cut -f2
  ```

- (b) The line after `$ gh run view 43536026856` is not that command's
  output. The script ran this command under that `$` line:

  ```
  gh run view 43536026856 --json status,conclusion,headSha,event --jq '"\(.event) \(.headSha) \(.status) \(.conclusion)"'
  ```

- (c) The job name "check" is in no log. It was read at 00:43Z, after
  `gh run watch 43536026856` exited 0, with this command, whose output
  named one job, "check", conclusion success:

  ```
  gh run view 43536026856 --json status,conclusion,headSha,jobs --jq '{status,conclusion,headSha,jobs:[.jobs[]|{name,conclusion}]}'
  ```

- (d) Known gaps of close-pre.sh, accepted by ruling H:
  - Point 1: it writes msg-control.txt into the phase scratch without
    creating the directory or checking the write, so a failed write
    would leave the Claude-Session control passing without testing
    anything.
  - Point 2: its control drops the ASCII clause of the message check,
    so the non-ASCII test has no control of its own.
  - Point 6: it hard-codes the repository and cache paths under the
    home directory, while its usage line in pre.log says "run from
    anywhere".
  - Point 7: its U+2014 test covers the tree at HEAD only, as its
    heading says ("over the range's tree at HEAD"), not each commit of
    the range, so a U+2014 added and then removed inside the range
    would not be seen.
