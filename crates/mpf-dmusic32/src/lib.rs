//! Port of the portable logic in Wine's `dlls/dmusic32/dmusic32_main.c`.
//!
//! Ported:
//! - `S_OK` return model of both exports: [`create_cdirectmusic_emulate_port`],
//!   [`enum_legacy_devices`], constant [`S_OK`]
//!
//! Skipped:
//! - Both exports are Wine stubs that only emit a `FIXME` and return `S_OK`.
//!   Their pointer arguments are never read, so the port takes them as `usize`
//!   and ignores them. The `FIXME` trace output is not modelled.
//! - `DllMain`/class factories and the `.spec` export table: not present in this
//!   Wine tree's `dmusic32_main.c`.

mod stubs;

pub use stubs::{create_cdirectmusic_emulate_port, enum_legacy_devices, S_OK};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
