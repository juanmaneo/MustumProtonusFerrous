//! Port of the portable logic in Wine's `dlls/dplay/dplay.spec` and `dlls/dplay/Makefile.in`.
//!
//! `dlls/dplay` has no C sources. It is a data-only DLL (`EXTRADLLFLAGS = -Wb,--data-only`)
//! whose two spec entries forward to `dplayx.dll`, so the port models the export table and the
//! build constants. No entry is `@ stub`, so there is no `StubException` to model.
//!
//! Ported:
//! - The export table: [`EXPORTS`], [`Export`], [`export`], with the i386 stack size that
//!   `winebuild` puts in each decorated name: [`Export::stack_bytes`]
//! - The data-only rule, which accepts only forwarded entries: [`data_only_accepts`]
//! - The module name and the `VER_*` version resource inputs: [`MODULE_NAME`], [`VersionInfo`],
//!   [`version_info`]
//!
//! Skipped:
//! - `DirectPlayCreate` and `DirectPlayEnumerate` themselves. They are implemented in
//!   `dlls/dplayx`, which has its own crate.
//! - The loader's resolution of a forwarder to `dplayx.dll`, which runs in ntdll.
//! - The ordinal assignment in `winebuild` (`assign_ordinals`). The ordinals it gives this spec
//!   are recorded as values in [`EXPORTS`].
//! - The PE export directory layout, the assembler output, and the version resource that
//!   `include/common.ver` builds from the `VER_*` values.

mod exports;
mod module;

pub use exports::{data_only_accepts, export, Export, EXPORTS};
pub use module::{version_info, VersionInfo, MODULE_NAME};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
