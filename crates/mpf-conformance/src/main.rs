//! `mpf-conformance` binary: runs all seeded conformance cases, exits 1 if
//! any fail. The Phase 2 gate: `cargo run -p mpf-conformance`.

use std::process::ExitCode;

fn main() -> ExitCode {
    let (_passed, failed) = mpf_conformance::run(&mpf_conformance::cases::all());
    mpf_conformance::exit_code(failed)
}
