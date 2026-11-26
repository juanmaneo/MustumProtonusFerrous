//! Port of the portable logic in Wine's `dlls/msnet32/msnet_main.c` and `dlls/msnet32/msnet32.spec`.
//!
//! Ported:
//! - `MSNET32_57`, the only explicit C function: [`msnet32_57`], which returns `-1` (`FAILURE`)
//!   whatever its arguments are
//! - The 95 `@ stub` exports (ordinals 1 to 96 except 57): one `ordinal_N` helper per export,
//!   each returning the [`StubException`] that `__wine_spec_unimplemented_stub` raises, with the
//!   module `msnet32.dll` and the ordinal as the function
//! - `__wine_spec_unimplemented_stub`: [`wine_spec_unimplemented_stub`], [`StubException`],
//!   [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The `FIXME` trace of the arguments in `MSNET32_57`, which the C body only prints.
//! - The `stdcall` convention. The arguments are `i32` and `*mut c_void`, and none is read.
//! - The rest of the `EXCEPTION_RECORD` that the C stub fills in (`ExceptionRecord`,
//!   `ExceptionAddress`, and the unused `ExceptionInformation` slots).
//! - The `for (;;)` loop that raises the record again and again: the port returns the record.

mod exports;
mod stub;

pub use exports::*;
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
