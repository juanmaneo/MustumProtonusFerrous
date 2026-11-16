//! Runs the real `mpf-conformance` binary: the Phase 2 gate is the process
//! exit code, and this is the only way to exercise `main` itself.

use std::process::Command;

#[test]
fn the_binary_runs_every_case_and_exits_zero() {
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-conformance"))
        .output()
        .expect("the conformance binary starts");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(0), "{stdout}");
    assert!(stdout.contains("ok 1 - "), "{stdout}");
    assert!(!stdout.contains("not ok"), "{stdout}");
    assert!(stdout.contains(", 0 failed, "), "{stdout}");
}
