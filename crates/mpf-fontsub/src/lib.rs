//! Port of the portable logic in Wine's `dlls/fontsub/main.c` and `dlls/fontsub/fontsub.spec`.
//!
//! Ported:
//! - `CreateFontPackage` format gate and copy-through: [`create_font_package`], [`FontPackageOut`],
//!   with the `TTFCFP_*`, `ERR_*` and `NO_ERROR` values from `include/fontsub.h`
//! - `fontsub.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `MergeFontPackage` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`merge_font_package`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `FIXME` trace of the arguments, and the `flags`, `face_index`, `lang`, `platform`,
//!   `encoding`, `keep_list`, `keep_len`, `reallocproc`, `freeproc` and `reserved` parameters,
//!   which the C body never reads or calls.
//! - The `__cdecl` calling convention and the OS heap: `allocproc` is a closure argument.

mod exports;
mod package;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use package::{
    create_font_package, FontPackageOut, ERR_GENERIC, ERR_MEM, NO_ERROR, TTFCFP_DELTA,
    TTFCFP_SUBSET, TTFCFP_SUBSET1,
};
pub use stub::{
    merge_font_package, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
