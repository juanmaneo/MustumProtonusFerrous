//! Port of the portable logic in Wine's `dlls/comcat/comcat.spec` and `dlls/comcat/Makefile.in`,
//! and of the `.def` writer in `tools/winebuild/spec32.c` that turns the spec into export records.
//!
//! `dlls/comcat` has no C sources and no `.idl`, `.rc` or `.tlb` file, so the component has no
//! executable code of its own. What it defines is a four-entry export table, three `-private`
//! stdcall entries with no body and one forward to `ole32`, and the Makefile values beside it.
//!
//! Ported:
//! - `comcat.spec` export table: [`EXPORTS`], [`Export`] and [`export`].
//! - `output_def_file` and its per-export line: [`def_file`], [`def_line`] and [`Target`].
//! - `dlls/comcat/Makefile.in` values: [`MODULE`], [`IMPORTS`], [`VER_OLESELFREGISTER`] and [`makefile_var`].
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer` have no body in `dlls/comcat`
//!   and are not `@ stub`. Winebuild routes only `TYPE_STUB` entries through
//!   `__wine_spec_unimplemented_stub`, so these are not modelled as stubs. Their behaviour is not
//!   defined in this directory, so it is not modelled here.
//! - `DllGetClassObject` forwards to `ole32.DllGetClassObject`, so its behaviour belongs to `ole32`.
//! - The `; File generated automatically` header, which names the local spec path.
//! - The `NONAME`, `DATA` and `=link_name` forms, which no `comcat.spec` entry needs.
//! - The `Import library doesn't export anything` warning, which winebuild prints on stderr.
//! - The Win16 path, `import_only` mode and `kill_at`.

mod def;
mod exports;
mod makefile;

pub use def::{def_file, def_line, Target};
pub use exports::{export, Export, EXPORTS};
pub use makefile::{makefile_var, IMPORTS, MODULE, VER_OLESELFREGISTER};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
