//! Port of the portable logic in Wine's `dlls/icmp/icmp.spec` and of the winebuild code that
//! turns its `@ stdcall` forwards into export-table lines (`tools/winebuild/parser.c`, `utils.c`
//! and `spec32.c`).
//!
//! Ported:
//! - the `@ stdcall` forward lines: [`parse_stdcall_forward`], [`Export`], [`exports`] and
//!   [`ICMP_SPEC`], with the ordinal numbering winebuild uses when no entry names one
//! - the argument type names and the i386 stack byte counts: [`ArgType`], [`parse_win32_arg`]
//!   and [`stack_bytes_i386`]
//! - the `winebuild --def` export line for each entry: [`Arch`] and [`def_line`]
//!
//! Skipped:
//! - `@ stub` entries. icmp.spec has none: every entry forwards to iphlpapi, so the
//!   `__wine_spec_unimplemented_stub` model does not apply and no `StubException` is raised.
//! - Entries that are not forwards (code entry points), other calling conventions such as
//!   `@ cdecl`, explicit ordinals, `;` comments and the `MAX_ARGUMENTS` limit.
//!   [`parse_stdcall_forward`] returns `None` for these.
//! - The ARM alignment of `int64` and `double` arguments in `get_args_size`, because only i386
//!   counts are used.
//! - The `-kill-at` option and the `; File generated` and `LIBRARY` header lines of the `.def`
//!   file, which only carry the spec path and module name.

mod args;
mod def;
mod spec;

pub use args::{parse_win32_arg, stack_bytes_i386, ArgType};
pub use def::{def_line, Arch};
pub use spec::{exports, parse_stdcall_forward, Export, ICMP_SPEC};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
