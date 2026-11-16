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
    for args in [&["bogus"][..], &["compare"], &["coverage", "extra"]] {
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
