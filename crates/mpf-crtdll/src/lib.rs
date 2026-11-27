//! Port of the portable logic in Wine's `dlls/crtdll/crtdll.spec` and of the `dlls/msvcrt`
//! routines it exports. `dlls/crtdll` has no `.c` files of its own: its `Makefile.in` sets
//! `PARENTSRC = ../msvcrt`, so the function bodies are the msvcrt ones.
//!
//! Ported:
//! - `crtdll.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - the nine `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per stub export:
//!   [`getsystime`], [`setsystime`], [`ismbbalnum`], [`ismbbalpha`], [`ismbbgraph`],
//!   [`ismbbkalnum`], [`ismbbkpunct`], [`ismbbprint`] and [`ismbbpunct`]
//! - `math.c`: [`abs`], [`labs`], [`div`], [`ldiv`], [`DivResult`], [`rotl`], [`lrotl`], [`rotr`],
//!   [`lrotr`], [`chgsign`], [`finite`] and [`isnan`]
//! - `ctype.c`: [`toupper`] and [`tolower`], the `_toupper` and `_tolower` exports
//! - `string.c`: [`swab`], the `_swab` export
//!
//! Skipped:
//! - The 31 `@ extern` data exports, which the spec binds to msvcrt data, and the 469 other
//!   `@ cdecl` and `@ varargs` exports. Most of these are locale-dependent, go through the C
//!   runtime's heap, stdio or process state, or call libm.
//! - The `__cdecl` and `__stdcall` calling conventions, and the `-ret64`, `-norelay` and
//!   `-private` flags of the spec. `div` and `ldiv` return the PE build's `div_t` and `ldiv_t`
//!   structs, which is the `__WINE_PE_BUILD` branch of `dlls/msvcrt/math.c`.
//! - The locale path of `tolower` and `toupper`, which depends on `initial_locale`. The port
//!   covers only the `_tolower` and `_toupper` exports.

mod ctype;
mod exports;
mod math;
mod string;
mod stub;

pub use ctype::{tolower, toupper};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use math::{abs, chgsign, div, finite, isnan, labs, ldiv, lrotl, lrotr, rotl, rotr, DivResult};
pub use string::swab;
pub use stub::{
    getsystime, ismbbalnum, ismbbalpha, ismbbgraph, ismbbkalnum, ismbbkpunct, ismbbprint,
    ismbbpunct, setsystime, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
