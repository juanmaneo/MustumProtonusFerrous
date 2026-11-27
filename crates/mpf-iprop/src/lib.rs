//! Port of the portable logic in Wine's `dlls/iprop/iprop.spec` and of the stack-size rule that
//! `winebuild` applies to stdcall exports (`get_args_size` in `tools/winebuild/utils.c`).
//!
//! `dlls/iprop` has no C sources, and `Makefile.in` builds it with `--data-only`, so every entry
//! in the spec is a forward to `ole32`. The DLL has no code of its own and no `@ stub` entries, so
//! the `__wine_spec_unimplemented_stub` model used by `mpf-fontsub` does not apply.
//!
//! Ported:
//! - the eight `@ stdcall` forwards: [`EXPORTS`], [`Export`], [`find_export`], [`forward_target`]
//! - the i386 stack bytes of each stdcall signature: [`stack_bytes`], [`ArgKind`]
//!
//! Skipped:
//! - The `ole32` functions that the forwards reach. They live in `dlls/ole32`, not in this DLL.
//! - The `int64`, `int128`, `float` and `double` branches of `get_args_size`. `iprop.spec` uses
//!   none of these types.
//! - Stack sizes for x86_64 and ARM. `Makefile.in` enables only i386.
//! - The `_name@N` decoration itself. That is `winebuild` output, not something the DLL computes.

mod args;
mod exports;

pub use args::{stack_bytes, ArgKind};
pub use exports::{find_export, forward_target, Export, EXPORTS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
