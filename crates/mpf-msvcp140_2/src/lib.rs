//! Port of the portable logic in Wine's `dlls/msvcp140_2/math.c`.
//!
//! Ported:
//! - `__std_smf_hypot3` (`hypot(hypot(x, y), z)` in double): [`std_smf_hypot3`]
//! - `__std_smf_hypot3f` (`hypotf(hypotf(x, y), z)` in float): [`std_smf_hypot3f`]
//!
//! Skipped:
//! - The 42 other `__std_smf_*` special-function exports. `msvcp140_2.spec` maps
//!   each of them to `stub`, and `math.c` has no body for them.
//! - The i386 `___std_*` stdcall-decorated aliases in the `.spec` file: calling
//!   convention glue, not logic.
//! - `Makefile.in`, `msvcp140_2.spec` and the empty `tests/` directory: build wiring.

/// `__std_smf_hypot3`: `hypot(hypot(x, y), z)` in double precision.
pub fn std_smf_hypot3(x: f64, y: f64, z: f64) -> f64 {
    x.hypot(y).hypot(z)
}

/// `__std_smf_hypot3f`: `hypotf(hypotf(x, y), z)` in single precision. The
/// intermediate is rounded to `f32`, so it overflows at `f32::MAX` even when the
/// double-precision version would not.
pub fn std_smf_hypot3f(x: f32, y: f32, z: f32) -> f32 {
    x.hypot(y).hypot(z)
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
