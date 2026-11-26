//! Port of the portable logic in Wine's `dlls/pidgen/main.c` and `dlls/pidgen/pidgen.spec`.
//!
//! Ported:
//! - `PIDGenSimpA` and `PIDGenSimpW`, which ignore their arguments and return 0: [`pid_gen_simp_a`],
//!   [`pid_gen_simp_w`]
//! - The `@ stub` exports `PIDGenA`, `PIDGenW`, `SetupPIDGenA`, `SetupPIDGenW` and
//!   `VerifyPIDSequenceW`, modelled as the `__wine_spec_unimplemented_stub` records they raise:
//!   [`pid_gen_a`], [`pid_gen_w`], [`setup_pid_gen_a`], [`setup_pid_gen_w`],
//!   [`verify_pid_sequence_w`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `FIXME` trace of the arguments (`debugstr_a` and `debugstr_w`), which only logs.
//! - The `WINAPI` calling convention, which the Rust signatures do not model.
//! - The `bink.bin` resource that `rsrc.rc` embeds as resources 1 and 2. It is data, not logic.

mod simp;
mod stub;

pub use simp::{pid_gen_simp_a, pid_gen_simp_w};
pub use stub::{
    pid_gen_a, pid_gen_w, setup_pid_gen_a, setup_pid_gen_w, verify_pid_sequence_w,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
