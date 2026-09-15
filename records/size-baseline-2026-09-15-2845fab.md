# Size baseline 2026-09-15-2845fab

The stripped `i686-unknown-linux-musl` sizes of the two size probes
(`specs/0006-passthrough.md`, "Size baseline"), measured by
`records/logs/t3-p2/size.log`. Milestone 1 measures emend's core as its
increase over `probe-zip-xml` built the same way (maintainer ruling 4,
`specs/0000-step-zero-plan.md`).

- Commit: 2845fab9ba5ebbea759313de66c7eda2699b84e6
- Date: 2026-09-15 (UTC)
- Toolchain: rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05)
- Target: i686-unknown-linux-musl
- Profile: `size` in the workspace `Cargo.toml` (inherits release,
  opt-level "z", lto "fat", codegen-units 1, panic "abort", strip true,
  debug false)
- `zip` 8.6.0, `default-features = false`, features
  `deflate-flate2-zlib-rs` (as `xlsx-diff`)
- `quick-xml` 0.42.0, `default-features = false`, no features, behind
  the `size-probe` feature `xml`, which only `probe-zip-xml` requires

| Binary | What it does | Bytes |
| --- | --- | --- |
| `probe-zip` | opens an archive with `zip`, prints the entry count | 473636 |
| `probe-zip-xml` | the same, then parses one entry with `quick-xml` in streaming mode and prints the element count | 539172 |
| difference | what `quick-xml` and the parse add | 65536 |

The dependency trees and features are listed in size.log; the licence of
every package in the tree, `quick-xml` included, passes the gate's
licences step (`records/logs/t3-p2/gate.log`).
