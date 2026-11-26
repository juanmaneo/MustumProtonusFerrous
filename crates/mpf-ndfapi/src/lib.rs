//! Port of the portable logic in Wine's `dlls/ndfapi/ndfapi_main.c` and `dlls/ndfapi/ndfapi.spec`.
//!
//! Ported:
//! - The explicit exports whose body is a `FIXME` trace followed by `return E_NOTIMPL`:
//!   [`ndf_close_incident`], [`ndf_create_connectivity_incident`], [`ndf_create_web_incident`],
//!   [`ndf_create_winsock_incident`], [`ndf_diagnose_incident`], [`ndf_execute_diagnosis`], and
//!   [`E_NOTIMPL`] from `include/winerror.h`
//! - The `@ stub` exports of `ndfapi.spec` and the `__wine_spec_unimplemented_stub` record they
//!   raise: [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export such as
//!   [`ndf_cancel_incident`] and [`dll_get_class_object`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments. The C bodies never read the arguments, so the ports
//!   take none. Their out-parameters (`NDFHANDLE *handle`, `ULONG *count`, `RootCauseInfo **causes`)
//!   are not written on this path, so the caller keeps whatever it stored.
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: `stdcall -private` in
//!   `ndfapi.spec`, with no body in `dlls/ndfapi`.

mod api;
mod stub;

pub use api::{
    ndf_close_incident, ndf_create_connectivity_incident, ndf_create_web_incident,
    ndf_create_winsock_incident, ndf_diagnose_incident, ndf_execute_diagnosis, E_NOTIMPL,
};
pub use stub::{
    dll_get_class_object, ndf_cancel_incident, ndf_create_dns_incident,
    ndf_create_grouping_incident, ndf_create_inbound_incident, ndf_create_incident,
    ndf_create_net_connection_incident, ndf_create_pnrp_incident, ndf_create_sharing_incident,
    ndf_create_web_incident_ex, ndf_get_trace_file, ndf_repair_incident, ndf_repair_incident_ex,
    ndf_run_dll_diagnose_incident, ndf_run_dll_diagnose_net_connection_incident,
    ndf_run_dll_diagnose_with_answer_file, ndf_run_dll_duplicate_ip_defending_system,
    ndf_run_dll_duplicate_ip_offending_system, ndf_run_dll_help_topic,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
