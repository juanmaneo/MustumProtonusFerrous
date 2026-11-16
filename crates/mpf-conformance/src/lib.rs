//! Conformance harness (Phase 2): runs a table of named cases against the
//! MPF APIs and reports TAP-style results. Cases are seeded from behaviors
//! that wine's own `dlls/*/tests` assert; each new ported component adds its
//! cases here, so `cargo run -p mpf-conformance` is the Phase 2 gate.

/// One conformance case.
pub struct Case {
    pub name: &'static str,
    pub run: fn() -> Result<(), String>,
}

pub mod cases;

/// Runs all cases, printing `ok`/`not ok` TAP lines. Returns (passed, failed).
pub fn run(cases: &[Case]) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;
    for (i, case) in cases.iter().enumerate() {
        let n = i + 1;
        match (case.run)() {
            Ok(()) => {
                println!("ok {n} - {}", case.name);
                passed += 1;
            }
            Err(e) => {
                println!("not ok {n} - {} ({e})", case.name);
                failed += 1;
            }
        }
    }
    println!("# {passed} passed, {failed} failed, {} total", cases.len());
    (passed, failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok_case() -> Result<(), String> {
        Ok(())
    }

    fn bad_case() -> Result<(), String> {
        Err("boom".to_string())
    }

    #[test]
    fn run_counts_pass_and_fail() {
        let cases = [
            Case { name: "a", run: ok_case },
            Case { name: "b", run: bad_case },
            Case { name: "c", run: ok_case },
        ];
        assert_eq!(run(&cases), (2, 1));
        assert_eq!(run(&[]), (0, 0));
    }
}
