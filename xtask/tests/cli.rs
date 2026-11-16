//! Runs the real `xtask` binary, which is the only way to exercise `main`
//! with arguments the test fully controls: calling `main()` from a unit test
//! would read the test harness's own command line, where a filter such as
//! `cargo test coverage` is indistinguishable from the `coverage` command.

use std::process::{Command, Output};

fn xtask(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(args)
        .output()
        .expect("the xtask binary starts")
}

/// Runs the binary with `MPF_REFS` pointed at a fresh empty directory, so no
/// reference tree is found.
fn xtask_without_refs(args: &[&str]) -> Output {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let refs = std::env::temp_dir().join(format!("mpf-cli-test-{id}"));
    std::fs::create_dir_all(&refs).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(args)
        .env("MPF_REFS", &refs)
        .output()
        .expect("the xtask binary starts");
    let _ = std::fs::remove_dir_all(&refs);
    output
}

#[test]
fn help_goes_to_stdout_and_succeeds() {
    for args in [&["--help"][..], &["-h"], &[]] {
        let output = xtask(args);
        assert_eq!(output.status.code(), Some(0), "{args:?}");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.starts_with("usage: cargo xtask <command>"),
            "{args:?}"
        );
        assert!(output.stderr.is_empty(), "{args:?}");
    }
}

#[test]
fn unknown_commands_print_usage_to_stderr_and_exit_with_2() {
    // `coverage extra` must be rejected by the parser, not start a coverage
    // run, so this stays fast and side-effect free.
    for args in [
        &["bogus"][..],
        &["compare"],
        &["coverage", "extra"],
        &["dlls", "extra"],
        &["compare-all", "--watch"],
        &["compare-all", "--watch", "soon"],
        &["compare-all", "--loop", "5"],
    ] {
        let output = xtask(args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.starts_with("usage: cargo xtask <command>"),
            "{args:?}"
        );
        assert!(output.stdout.is_empty(), "{args:?}");
    }
}

#[test]
fn help_lists_the_phase5_commands() {
    let output = xtask(&["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    for needle in ["compare-all", "--watch", "dlls", "report.txt", "coverage"] {
        assert!(stdout.contains(needle), "{needle} missing from:\n{stdout}");
    }
}

#[test]
fn dlls_and_compare_all_fail_cleanly_without_the_reference_tree() {
    // With MPF_REFS at an empty directory there is no wine tree: both Phase 5
    // commands must say so on stderr and exit 1, not panic or hang.
    for args in [&["dlls"][..], &["compare-all"][..]] {
        let output = xtask_without_refs(args);
        assert_eq!(output.status.code(), Some(1), "{args:?}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.starts_with("error: "), "{args:?}: {stderr}");
        assert!(stderr.contains("MPF_REFS"), "{args:?}: {stderr}");
        assert!(output.stdout.is_empty(), "{args:?}");
    }
}
