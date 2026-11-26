//! Port of the portable logic in Wine's `dlls/dpnaddr/dpnaddr.spec` and `dlls/dpnaddr/Makefile.in`.
//!
//! Ported:
//! - The one `dpnaddr.spec` export, a forward to `dpnet`: [`FORWARDS`], [`Forward`] and
//!   [`forward_for`]
//! - The `stdcall` argument bytes that winebuild's `get_args_size` gives a forward: [`stdcall_arg_bytes`],
//!   [`ArgType`] and [`Cpu`]
//! - The data-only rule that `-Wb,--data-only` turns on in winebuild: [`data_only_accepts`]
//! - The `Makefile.in` constants: [`MODULE`], [`PRODUCT_VERSION`], [`FILE_DESCRIPTION`] and
//!   [`DATA_ONLY`]
//!
//! Skipped:
//! - Executable code: `dlls/dpnaddr` has no `.c`, `.idl`, `.rc` or `.tlb` file, and the DLL is
//!   data-only, so there is no function body or return value to port.
//! - The `@ stub` model: the spec has no `@ stub` line. The export is a forward that the loader
//!   resolves in `dpnet.dll`, so no `__wine_spec_unimplemented_stub` record applies.
//! - The ordinal assignment: the spec line has no ordinal, so winebuild's `assign_ordinals` gives
//!   1. The port stores that result (oracle row 04) and does not port the assignment.
//! - ARM alignment of `int64` and `double` in `get_args_size`, and the ARM targets: this crate
//!   covers i386 and x86_64 only.
//! - The `.spec` grammar and winebuild's `FLAG_FORWARD` detection: the port states the forward
//!   directly.

mod args;
mod forwards;
mod module;

pub use args::{stdcall_arg_bytes, ArgType, Cpu};
pub use forwards::{forward_for, Forward, FORWARDS};
pub use module::{data_only_accepts, DATA_ONLY, FILE_DESCRIPTION, MODULE, PRODUCT_VERSION};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
