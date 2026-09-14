//! xlsx-diff: the entry-level fidelity harness. This is the T0 skeleton;
//! T1 adds the archive comparison.

use std::process::ExitCode;

const USAGE: &str = "usage: xlsx-diff <input.xlsx> <output.xlsx>";

/// Exit status for usage errors and other failures.
const EXIT_ERROR: u8 = 2;

/// Returns the message to print on stderr and the exit status.
fn run() -> (&'static str, u8) {
    (USAGE, EXIT_ERROR)
}

fn main() -> ExitCode {
    let (message, status) = run();
    eprintln!("{message}");
    ExitCode::from(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_usage_and_exits_2() {
        let (message, status) = run();
        assert!(message.starts_with("usage: xlsx-diff "));
        assert_eq!(status, 2);
    }
}
