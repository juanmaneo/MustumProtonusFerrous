//! Port of the portable logic in Wine's `dlls/mgmtapi/mgmtapi.c` and `dlls/mgmtapi/mgmtapi.spec`.
//!
//! Ported:
//! - `SnmpMgrOpen` return value: [`snmp_mgr_open`], [`SnmpMgrSession`]
//! - `SnmpMgrTrapListen` return value and last error: [`snmp_mgr_trap_listen`],
//!   [`ERROR_CALL_NOT_IMPLEMENTED`]
//! - `mgmtapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The seven `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`] and one helper per export, from
//!   [`snmp_mgr_close`] to [`snmp_mgr_str_to_oid`]
//!
//! Skipped:
//! - The `FIXME` traces, and the `hostname`, `community`, `timeout` and `retries` arguments of
//!   `SnmpMgrOpen` and the `available` argument of `SnmpMgrTrapListen`, which the C bodies only
//!   print or never touch.
//! - The `WINAPI` calling convention, the global last-error slot (the port returns the value
//!   `SetLastError` stores) and the stdcall argument-stack sizes in the spec.
//! - The `ExceptionRecord` (always NULL) and `ExceptionAddress` (the address of the stub) fields
//!   of the raised record, which the stub records do not carry.
//! - The ordinal form of the stub function name: every `mgmtapi.spec` export has a name, so
//!   `winebuild` never passes an ordinal.

mod exports;
mod mgmtapi;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use mgmtapi::{
    snmp_mgr_open, snmp_mgr_trap_listen, SnmpMgrSession, ERROR_CALL_NOT_IMPLEMENTED,
};
pub use stub::{
    snmp_mgr_close, snmp_mgr_ctl, snmp_mgr_get_trap, snmp_mgr_get_trap_ex, snmp_mgr_oid_to_str,
    snmp_mgr_request, snmp_mgr_str_to_oid, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
