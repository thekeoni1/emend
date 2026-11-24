# Contributing to emend

## Reporting a bug

Open an issue with the emend version, the Rust version, the target, the
call you made and what came back: the error text, or what the output
holds that you did not expect.

Never attach a private workbook. Reduce the problem to a workbook that
can be public, or describe it by entry names, byte counts and the error
text.

## What the directories are

`specs/` holds a numbered spec for every change: the decision behind it
and how it is accepted. `records/` holds the measurements the README
cites, with the logs that produced them.

## Building and testing

The README's "Building and testing" section has the pinned toolchain,
the two targets and the commands. `scripts/check.sh` runs every step of
the gate.

## Pull requests

Bug reports are welcome. A pull request follows an issue in which the
change was agreed. It passes `scripts/check.sh` and adds no dependency
to the core.

## Licence

A contribution is offered under the crate's two licences, MIT or
Apache-2.0, at the user's option.
