//! Conformance harness (Phase 2): runs a table of named cases against the
//! MPF APIs and reports TAP-style results. Cases are seeded from behaviors
//! that wine's own `dlls/*/tests` assert; each new ported component adds its
//! cases here, so `cargo run -p mpf-conformance` is the Phase 2 gate.

use std::any::Any;
use std::process::ExitCode;

/// One conformance case.
pub struct Case {
    pub name: &'static str,
    /// Fails by panicking, the way an `assert!` does; [`run`] catches it.
    pub run: fn(),
}

pub mod cases;

/// Runs all cases, printing `ok`/`not ok` TAP lines. A case that panics
/// fails with the panic message. Returns (passed, failed).
pub fn run(cases: &[Case]) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    for (i, case) in cases.iter().enumerate() {
        let n = i + 1;
        match std::panic::catch_unwind(case.run) {
            Ok(()) => {
                println!("ok {n} - {}", case.name);
                passed += 1;
            }
            Err(payload) => {
                println!("not ok {n} - {} ({})", case.name, panic_message(&*payload));
                failed += 1;
            }
        }
    }
    println!("# {passed} passed, {failed} failed, {} total", cases.len());
    (passed, failed)
}

/// The text of a panic payload: `panic!("literal")` carries a `&str`,
/// `panic!("{x}")` a `String`; anything else has no message to show.
fn panic_message(payload: &(dyn Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "non-string panic payload".to_owned())
}

/// The process exit code for a run: 0 when nothing failed, 1 otherwise.
pub fn exit_code(failed: usize) -> ExitCode {
    ExitCode::from(u8::from(failed > 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok_case() {}

    fn str_panic_case() {
        panic!("boom");
    }

    fn string_panic_case() {
        let what = "formatted";
        panic!("{what} boom");
    }

    fn other_panic_case() {
        std::panic::panic_any(5u32);
    }

    #[test]
    fn run_counts_pass_and_fail() {
        let case = |name, run| Case { name, run };
        let cases = [
            case("a", ok_case as fn()),
            case("b", str_panic_case),
            case("c", ok_case),
            case("d", string_panic_case),
            case("e", other_panic_case),
        ];
        assert_eq!(run(&cases), (2, 3));
        assert_eq!(run(&[]), (0, 0));
    }

    #[test]
    fn panic_message_reads_str_and_string_payloads() {
        let from_str: Box<dyn Any + Send> = Box::new("literal");
        let from_string: Box<dyn Any + Send> = Box::new(String::from("owned"));
        let neither: Box<dyn Any + Send> = Box::new(5u32);
        assert_eq!(panic_message(&*from_str), "literal");
        assert_eq!(panic_message(&*from_string), "owned");
        assert_eq!(panic_message(&*neither), "non-string panic payload");
    }

    #[test]
    fn exit_code_is_nonzero_only_on_failure() {
        assert_eq!(exit_code(0), ExitCode::SUCCESS);
        assert_eq!(exit_code(1), ExitCode::FAILURE);
        assert_eq!(exit_code(42), ExitCode::FAILURE);
    }
}
