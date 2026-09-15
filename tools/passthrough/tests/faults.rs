//! T3 P1 tests (`specs/0006-passthrough.md`): the command line, the error
//! paths and the test-only fault injection with its guard, through the
//! binary. No test leaves a partial output or a temporary file behind.

mod common;

use std::fs;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use common::*;
use zip::CompressionMethod;

const ALLOWED: (&str, &str) = ("PASSTHROUGH_FAULT_ALLOWED", "1");

/// A small archive at `in.zip` in a new temporary directory.
fn with_input(label: &str) -> TempDir {
    let dir = TempDir::new(label);
    zip_with(&dir.path("in.zip"), |w| {
        add_file(
            w,
            "xl/workbook.xml",
            &text(5),
            options(CompressionMethod::Deflated),
        );
    });
    dir
}

/// Asserts an error run: exit 2, nothing on stdout, exactly `line` on
/// stderr, which holds no path.
fn assert_error(copied: &Copied, dir: &TempDir, line: &str) {
    assert_eq!(copied.code, Some(2), "exit; stderr: {}", copied.stderr);
    assert_eq!(copied.stdout, "");
    assert_eq!(copied.stderr, format!("{line}\n"));
    assert!(
        !copied
            .stderr
            .contains(dir.root().to_str().expect("a UTF-8 path")),
        "no path in stderr"
    );
}

fn copy_with(dir: &TempDir, env: &[(&str, &str)]) -> Copied {
    passthrough(&[&dir.path("in.zip"), &dir.path("out.zip")], env)
}

#[test]
fn usage_errors() {
    let dir = with_input("usage");
    let input = dir.path("in.zip");
    let usage = "error: usage: passthrough <input.xlsx> <output.xlsx>";
    assert_error(&passthrough(&[], &[]), &dir, usage);
    assert_error(&passthrough(&[&input], &[]), &dir, usage);
    assert_error(&passthrough(&[&input, &input, &input], &[]), &dir, usage);
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn missing_input() {
    let dir = TempDir::new("missing");
    let copied = passthrough(&[&dir.path("in.zip"), &dir.path("out.zip")], &[]);
    assert_error(
        &copied,
        &dir,
        "error: cannot read the input file: entity not found",
    );
    assert!(dir.names().is_empty());
}

#[test]
fn input_that_is_not_a_zip_archive() {
    let dir = TempDir::new("not-zip");
    fs::write(dir.path("in.zip"), text(4)).expect("write");
    assert_error(
        &copy_with(&dir, &[]),
        &dir,
        "error: the input file is not a readable zip archive",
    );
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn output_directory_missing() {
    let dir = with_input("no-output-dir");
    let copied = passthrough(
        &[&dir.path("in.zip"), &dir.path("absent").join("out.zip")],
        &[],
    );
    assert_error(
        &copied,
        &dir,
        "error: cannot create the output file: entity not found",
    );
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn output_path_without_a_file_name() {
    let dir = with_input("no-file-name");
    let copied = passthrough(&[&dir.path("in.zip"), &dir.root().join("..")], &[]);
    assert_error(&copied, &dir, "error: the output path has no file name");
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn failed_rename_removes_the_temporary_file() {
    let dir = with_input("rename");
    // The target is a non-empty directory, so the copy succeeds and the move
    // into place fails.
    fs::create_dir(dir.path("out.zip")).expect("create a directory");
    fs::write(dir.path("out.zip").join("keep"), b"x").expect("write");
    let copied = copy_with(&dir, &[]);
    assert_eq!(copied.code, Some(2));
    assert!(
        copied
            .stderr
            .starts_with("error: cannot move the output into place: "),
        "stderr: {}",
        copied.stderr
    );
    assert_eq!(copied.stderr.lines().count(), 1);
    assert_eq!(dir.names(), ["in.zip", "out.zip"]);
    assert!(dir.path("out.zip").join("keep").is_file());
}

#[test]
fn existing_output_is_replaced() {
    let dir = with_input("replace");
    fs::write(dir.path("out.zip"), text(3)).expect("write");
    copy_ok(&dir.path("in.zip"), &dir.path("out.zip"));
    assert_eq!(dir.names(), ["in.zip", "out.zip"]);
    assert_eq!(
        fs::read(dir.path("in.zip")).expect("read"),
        fs::read(dir.path("out.zip")).expect("read")
    );
}

#[test]
fn fault_variable_is_refused_without_the_allow_variable() {
    let dir = with_input("guard");
    let refused = "error: PASSTHROUGH_FAULT is set without PASSTHROUGH_FAULT_ALLOWED=1 (test-only)";
    for env in [
        vec![("PASSTHROUGH_FAULT", "exit3")],
        vec![("PASSTHROUGH_FAULT", "")],
        vec![
            ("PASSTHROUGH_FAULT", "panic"),
            ("PASSTHROUGH_FAULT_ALLOWED", "0"),
        ],
        vec![
            ("PASSTHROUGH_FAULT", "abort"),
            ("PASSTHROUGH_FAULT_ALLOWED", "yes"),
        ],
    ] {
        assert_error(&copy_with(&dir, &env), &dir, refused);
        assert_eq!(dir.names(), ["in.zip"]);
    }
    // The allow variable alone changes nothing.
    let copied = copy_with(&dir, &[ALLOWED]);
    assert_eq!(copied.code, Some(0), "stderr: {}", copied.stderr);
    assert_eq!(dir.names(), ["in.zip", "out.zip"]);
}

#[test]
fn unknown_fault_value() {
    let dir = with_input("unknown");
    assert_error(
        &copy_with(&dir, &[("PASSTHROUGH_FAULT", "crash"), ALLOWED]),
        &dir,
        "error: PASSTHROUGH_FAULT has an unknown value",
    );
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn exit3_fault() {
    let dir = with_input("exit3");
    let copied = copy_with(&dir, &[("PASSTHROUGH_FAULT", "exit3"), ALLOWED]);
    assert_eq!((copied.code, copied.signal), (Some(3), None));
    assert_eq!((copied.stdout.as_str(), copied.stderr.as_str()), ("", ""));
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn panic_fault() {
    let dir = with_input("panic");
    let copied = copy_with(&dir, &[("PASSTHROUGH_FAULT", "panic"), ALLOWED]);
    assert_eq!((copied.code, copied.signal), (Some(101), None));
    assert_eq!(copied.stdout, "");
    assert!(
        copied.stderr.contains("passthrough fault injection: panic"),
        "stderr: {}",
        copied.stderr
    );
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn abort_fault() {
    let dir = with_input("abort");
    let copied = copy_with(&dir, &[("PASSTHROUGH_FAULT", "abort"), ALLOWED]);
    assert_eq!((copied.code, copied.signal), (None, Some(6)), "SIGABRT");
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn sleep_fault_is_still_running_after_a_second() {
    let dir = with_input("sleep");
    let mut child = Command::new(env!("CARGO_BIN_EXE_passthrough"))
        .args([dir.path("in.zip"), dir.path("out.zip")])
        .env("PASSTHROUGH_FAULT", "sleep")
        .env(ALLOWED.0, ALLOWED.1)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("start the binary");
    thread::sleep(Duration::from_secs(1));
    let still_running = child.try_wait().expect("poll the child").is_none();
    let _ = child.kill();
    let _ = child.wait();
    assert!(still_running, "the sleep fault waits before copying");
    assert_eq!(dir.names(), ["in.zip"]);
}

#[test]
fn sleep_copying_fault_leaves_the_temporary_file_when_killed() {
    let dir = with_input("sleep-copying");
    let mut child = Command::new(env!("CARGO_BIN_EXE_passthrough"))
        .args([dir.path("in.zip"), dir.path("out.zip")])
        .env("PASSTHROUGH_FAULT", "sleep-copying")
        .env(ALLOWED.0, ALLOWED.1)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("start the binary");
    let temporary = format!(".out.zip.passthrough-{}.tmp", child.id());
    thread::sleep(Duration::from_secs(1));
    let still_running = child.try_wait().expect("poll the child").is_none();
    let while_sleeping = dir.names();
    let _ = child.kill();
    let _ = child.wait();
    assert!(still_running, "the sleep-copying fault waits while copying");
    assert_eq!(
        while_sleeping,
        [temporary.as_str(), "in.zip"],
        "the temporary file exists while the copy sleeps"
    );
    assert_eq!(
        dir.names(),
        [temporary.as_str(), "in.zip"],
        "a kill during the copy leaves the temporary file and no output"
    );
}

#[test]
fn alloc_fault_without_a_limit_then_copies() {
    let dir = with_input("alloc");
    let copied = copy_with(&dir, &[("PASSTHROUGH_FAULT", "alloc"), ALLOWED]);
    assert_eq!(copied.code, Some(0), "stderr: {}", copied.stderr);
    assert_eq!(dir.names(), ["in.zip", "out.zip"]);
}

#[test]
fn alloc_fault_under_a_256_mib_address_space_limit_aborts() {
    let dir = with_input("alloc-limit");
    let out = Command::new("sh")
        .arg("-c")
        .arg("ulimit -v 262144 && exec \"$0\" \"$@\"")
        .arg(env!("CARGO_BIN_EXE_passthrough"))
        .args([dir.path("in.zip"), dir.path("out.zip")])
        .env("PASSTHROUGH_FAULT", "alloc")
        .env(ALLOWED.0, ALLOWED.1)
        .stdin(Stdio::null())
        .output()
        .expect("run the binary under sh");
    assert_eq!(
        (out.status.code(), out.status.signal()),
        (None, Some(6)),
        "the allocation failure aborts"
    );
    let stderr = String::from_utf8(out.stderr).expect("UTF-8");
    assert_eq!(
        stderr.lines().next(),
        Some("memory allocation of 536870912 bytes failed"),
        "the standard library's message"
    );
    assert_eq!(dir.names(), ["in.zip"]);
}
