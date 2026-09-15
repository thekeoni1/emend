# Toolchain record, 2026-09-15 (UTC)

Written by the T0 P5 log script from the same run as
records/logs/t0-p5/gate.log. No toolchain or target was installed in P5.

- Full gate (all seven steps) passed at: 346385c8c8934c3485b27160a69fd032f35ef8b5
- rustc: rustc 1.98.1 (48a229cea 2026-09-01)
  - commit-hash: 48a229ceaefd4985c50990b14116b6d856af0985
  - host: x86_64-unknown-linux-gnu
  - release: 1.98.1
  - LLVM version: 22.1.8
- cargo: cargo 1.98.1 (797e8a9bc 2026-08-05)
- Active toolchain: 1.98.1-x86_64-unknown-linux-gnu (overridden by 'rust-toolchain.toml')
- Installed targets for that toolchain: i686-unknown-linux-musl x86_64-unknown-linux-gnu
- Kernel (`uname -r`): 6.18.33.2-microsoft-standard-WSL2
- i686 test binary (`file`): target/i686-unknown-linux-musl/debug/deps/xlsx_diff-6260f7a0ee0830e0: ELF 32-bit LSB executable, Intel i386, version 1 (GNU/Linux), statically linked, BuildID[sha1]=ad7b0616635bbe5e3dc94cc97a74343633b44c0f, with debug_info, not stripped
- Guard: every rustup and cargo command ran with RUSTUP_AUTO_INSTALL=0 and CARGO_TERM_COLOR=never.
