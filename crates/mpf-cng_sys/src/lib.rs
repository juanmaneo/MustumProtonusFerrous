//! Port of the portable logic in Wine's `dlls/cng.sys/main.c` and `dlls/cng.sys/cng.sys.spec`.
//!
//! Ported:
//! - `DriverEntry` (`main.c`): always `STATUS_SUCCESS`: [`driver_entry`], [`STATUS_SUCCESS`]
//! - The 63 entries of `cng.sys.spec`, the 35 `@ stdcall` forwards to `bcrypt.<name>` and the
//!   28 `@ stub` exports: [`EXPORTS`], [`Export`], [`ExportKind`], [`find_export`]
//! - `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`), the exception each stub raises:
//!   [`unimplemented_stub`], [`StubException`], [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`], [`WINE_SPEC_FILE_NAME`]
//!
//! Skipped:
//! - The bodies of the 35 forwarded functions: they live in `dlls/bcrypt`, not in this DLL.
//! - Export ordinals: winebuild assigns them, and the spec does not state them.
//! - `ExceptionAddress` of the stub record: it is the address of the stub routine, not a fixed value.
//! - `DriverEntry`'s `DRIVER_OBJECT *` and `UNICODE_STRING *` arguments: both are unused.

mod driver;
mod exports;
mod stubs;

pub use driver::{driver_entry, NtStatus, STATUS_SUCCESS};
pub use exports::{find_export, Export, ExportKind, EXPORTS};
pub use stubs::{
    unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
    WINE_SPEC_FILE_NAME,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
