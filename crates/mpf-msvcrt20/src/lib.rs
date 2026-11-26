//! Port of the portable logic in Wine's `dlls/msvcrt20/msvcrt20.c` and `dlls/msvcrt20/msvcrt20.spec`.
//!
//! Ported:
//! - `MSVCRT20__getmainargs` and `MSVCRT20__wgetmainargs` (exported as `__getmainargs` and
//!   `__wgetmainargs`): [`getmainargs`] and [`wgetmainargs`]. `new_mode` reaches the CRT by address
//!   of a local copy, so the CRT's writes through it are not seen by the caller.
//! - `MSVCRT20__matherr` (exported as `_matherr`): [`matherr`], which returns 0.
//! - The two `@ stub` exports, `??_EIostream_init@@QAEPAXI@Z` and `??_GIostream_init@@QAEPAXI@Z`:
//!   [`vector_deleting_dtor_iostream_init`] and [`scalar_deleting_dtor_iostream_init`], and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`].
//!
//! Skipped:
//! - The 1120 forward exports to `msvcrt.dll` and `msvcirt.dll`, including the `@ extern` data
//!   forwards. The PE loader resolves them, and this DLL has no code for them.
//! - The `# extern _fileinfo` line in the spec, which is a comment.
//! - The `__cdecl` calling convention, and the CRT's own `__getmainargs`, `__wgetmainargs` and
//!   `_matherr`: the CRT call is a closure argument, as `allocproc` is in `mpf-fontsub`.

mod mainargs;
mod matherr;
mod stub;

pub use mainargs::{getmainargs, wgetmainargs};
pub use matherr::matherr;
pub use stub::{
    scalar_deleting_dtor_iostream_init, vector_deleting_dtor_iostream_init,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
