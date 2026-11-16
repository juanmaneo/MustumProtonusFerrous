//! `mpf-conformance` binary: runs all seeded conformance cases, exits 1 if
//! any fail. The Phase 2 gate: `cargo run -p mpf-conformance`.

use std::process::ExitCode;

fn main() -> ExitCode {
    let cases = mpf_conformance::cases::all();
    let (_passed, failed) = mpf_conformance::run(&cases);
    ExitCode::from(if failed > 0 { 1 } else { 0 })
}
