//! Port of the portable logic in Wine's `dlls/wmilib.sys/main.c` and `dlls/wmilib.sys/wmilib.sys.spec`.
//!
//! Ported:
//! - `DriverEntry`, which returns `STATUS_SUCCESS` without using its arguments: [`driver_entry`],
//!   [`NtStatus`], [`STATUS_SUCCESS`]
//! - The 5 `@ stub` exports in `wmilib.sys.spec`: one function per export, named by the snake_case
//!   of the export name, each returning the record that `__wine_spec_unimplemented_stub` raises:
//!   [`dll_initialize`], [`dll_unload`], [`wmi_complete_request`], [`wmi_fire_event`],
//!   [`wmi_system_control`], [`wine_spec_unimplemented_stub`], [`StubException`], [`MODULE_NAME`],
//!   [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The `driver` and `path` arguments of `DriverEntry`: the C body never reads them.
//! - The `WINAPI` and `__cdecl` calling conventions.
//! - Ordinal exports: every export in the spec has a name, so the ordinal form of the `function`
//!   field is never used.

mod driver;
mod stub;
mod stubs;

pub use driver::{driver_entry, NtStatus, STATUS_SUCCESS};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
    MODULE_NAME,
};
pub use stubs::{
    dll_initialize, dll_unload, wmi_complete_request, wmi_fire_event, wmi_system_control,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
