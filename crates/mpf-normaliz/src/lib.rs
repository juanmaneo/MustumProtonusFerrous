//! Port of the portable logic in Wine's `dlls/normaliz/normaliz.spec` and of the stack-size rule
//! that `winebuild` applies to stdcall exports (`get_args_size` in `tools/winebuild/utils.c`).
//!
//! `dlls/normaliz` has no C sources, and its `Makefile.in` lists only `normaliz.spec`. Every entry
//! in the spec is a forward to `kernel32`, so the DLL has no code of its own. The spec has no
//! `@ stub` entries, so the `__wine_spec_unimplemented_stub` model used by `mpf-fontsub` does not
//! apply. The port is partial: it covers the export table and the stack-size rule.
//!
//! Ported:
//! - the five `@ stdcall` forwards: [`EXPORTS`], [`Export`], [`find_export`], [`forward_target`]
//! - the i386 stack bytes of each stdcall signature: [`stack_bytes`], [`ArgKind`]
//!
//! Skipped:
//! - The `kernel32` functions that the forwards reach. `kernel32.spec` lists them as `-import`
//!   entries, and they are not part of this DLL.
//! - The `int64`, `int128`, `float` and `double` branches of `get_args_size`. `normaliz.spec` uses
//!   none of these types.
//! - Stack sizes for x86_64 and ARM. `get_abi_name` in `tools/winebuild/utils.c` returns the bare
//!   name on those arches, so there is no stack suffix to compute.
//! - The `_name@N` decoration itself, which is winebuild output, and the ordinals, which the spec
//!   leaves to winebuild.

mod args;
mod exports;

pub use args::{stack_bytes, ArgKind};
pub use exports::{find_export, forward_target, Export, EXPORTS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
