//! Port of the portable logic in Wine's `dlls/wsnmp32/wsnmp32.c` and `dlls/wsnmp32/wsnmp32.spec`.
//!
//! Ported:
//! - The five `stdcall` exports with C bodies: [`snmp_cleanup`], [`snmp_open`],
//!   [`snmp_set_retransmit_mode`], [`snmp_set_translate_mode`] and [`snmp_startup`], with the
//!   `SNMPAPI_*` values from `include/winsnmp.h`: [`SNMPAPI_SUCCESS`], [`SNMPAPI_FAILURE`],
//!   [`SNMPAPI_V2_SUPPORT`], [`SNMPAPI_UNTRANSLATED_V1`] and [`SNMPAPI_ON`]
//! - `wsnmp32.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 43 `stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`snmp_get_timeout`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments. The C bodies never read the `hwnd` and `msg`
//!   arguments of `SnmpOpen` or the `retransmit_mode` and `translate_mode` arguments of the
//!   setters, so the port accepts and ignores them.
//! - The SNMP session, message, PDU, variable-binding and OID implementation: Wine has none.
//! - The `__stdcall` calling convention: the functions are plain Rust functions.

mod calls;
mod exports;
mod stubs;

pub use calls::{
    snmp_cleanup, snmp_open, snmp_set_retransmit_mode, snmp_set_translate_mode, snmp_startup,
    Handle, NULL_SESSION, SNMPAPI_FAILURE, SNMPAPI_ON, SNMPAPI_SUCCESS, SNMPAPI_UNTRANSLATED_V1,
    SNMPAPI_V2_SUPPORT,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
