//! Port of the portable logic in Wine's `dlls/wldp/wldp.c` and `dlls/wldp/wldp.spec`.
//!
//! Ported:
//! - `WldpIsDynamicCodePolicyEnabled`, `WldpGetLockdownPolicy` and `WldpQueryWindowsLockdownMode`:
//!   the return values and out-parameter writes: [`is_dynamic_code_policy_enabled`],
//!   [`get_lockdown_policy`] and [`query_windows_lockdown_mode`], with the `WLDP_*` values from
//!   `include/wldp.h` and [`hresult_from_nt`] from `include/winerror.h`
//! - `wldp.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 26 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`wldp_is_app_approved_by_policy`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments. The `flags` argument of `WldpGetLockdownPolicy` is
//!   never read by the C body, so the port accepts and ignores it.
//! - A NULL `is_enabled` or `state` pointer: the C bodies write through it unconditionally, so
//!   a NULL there is a crash. A Rust `&mut` cannot be NULL, so the port has no such path.
//! - The fields of `WLDP_HOST_INFORMATION`: the body only checks the pointer for NULL, so
//!   [`WldpHostInformation`] carries no data.
//! - The `__cdecl` and `WINAPI` calling conventions.

mod exports;
mod lockdown;
mod stubs;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use lockdown::{
    get_lockdown_policy, hresult_from_nt, is_dynamic_code_policy_enabled,
    query_windows_lockdown_mode, Bool, Hresult, WldpHostInformation, FACILITY_NT_BIT, FALSE, S_OK,
    WLDP_LOCKDOWN_DEFINED_FLAG, WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED, WLDP_WINDOWS_LOCKDOWN_MODE_MAX,
    WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL, WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED,
};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
