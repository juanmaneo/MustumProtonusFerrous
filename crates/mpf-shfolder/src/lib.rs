//! Port of the portable logic in Wine's `dlls/shfolder/shfolder.spec` and the `winebuild` rules that turn it into an export table.
//!
//! `dlls/shfolder` has no C sources and no code of its own. Its spec forwards `SHGetFolderPathA` and
//! `SHGetFolderPathW` to `shell32`, and its Makefile sets `--data-only`. The port covers the export table and the `.def` text
//! that `winebuild` derives from it, so the verdict is partial.
//!
//! Ported:
//! - `shfolder.spec` export table: [`EXPORTS`], [`Export`], [`ArgType`], [`export_by_name`]
//! - `get_args_size` from `tools/winebuild/utils.c`: [`get_args_size`], with the pointer sizes of [`Cpu`] from `tools/tools.h`
//! - `winebuild --def` output from `output_def_file` in `tools/winebuild/spec32.c`: [`def_line`], [`def_file`], [`MODULE_NAME`]
//!
//! Skipped:
//! - The forwarded functions. They live in `shell32`, which is not part of this component.
//! - `winebuild`'s `FLAG_FORWARD` test (a `.` in the link name) and its `--data-only` check. Both entries are forwards, so the
//!   port stores the forward string.
//! - Ordinal auto-assignment. The ordinals are the ones `winebuild` printed for this spec.
//! - `@ stub` handling. `shfolder.spec` has no `@ stub` line, so no `__wine_spec_unimplemented_stub` record is ported.
//! - The `ARG_*` types other than `long` and `ptr`, and the `-k` (kill-at) option.
//! - The `VER_*` values in `Makefile.in`. They feed the version resource, not the export table.

mod def;
mod exports;

pub use def::{def_file, def_line, get_args_size, Cpu, MODULE_NAME};
pub use exports::{export_by_name, ArgType, Export, EXPORTS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
