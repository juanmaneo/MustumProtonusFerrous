//! Port of the portable logic in Wine's `dlls/tbs/tbs.c` and `dlls/tbs/tbs.spec`.
//!
//! Ported:
//! - `Tbsi_Context_Create`, `Tbsi_GetDeviceInfo` and `GetDeviceIDString`, which return a fixed
//!   value: [`tbsi_context_create`], [`tbsi_get_device_info`], [`get_device_id_string`], with
//!   [`TBS_E_TPM_NOT_FOUND`] and [`E_NOTIMPL`] from `include/winerror.h`
//! - `tbs.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 18 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per stub, such as
//!   [`get_device_id`] and [`tbsip_test_mor_bit`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments. The C bodies never read `params`, `out`, `size`,
//!   `info`, `used` or `tpm`, so the ported functions take no arguments.
//! - The `WINAPI` and `__cdecl` calling conventions.
//! - The stub loop itself: the port returns the record instead of raising it on every iteration,
//!   since `RtlRaiseException` never returns to the C caller.

mod device;
mod exports;
mod stub;

pub use device::{
    get_device_id_string, tbsi_context_create, tbsi_get_device_info, Hresult, TbsResult, E_NOTIMPL,
    TBS_E_TPM_NOT_FOUND,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    get_device_id, get_device_id_with_timeout, tbsi_create_attestation_from_log,
    tbsi_create_windows_key, tbsi_filter_log, tbsi_get_owner_auth, tbsi_get_tcg_log,
    tbsi_get_tcg_log_ex, tbsi_get_tcg_logs, tbsi_physical_presence_command,
    tbsi_revoke_attestation, tbsi_sha_hash, tbsip_cancel_commands, tbsip_context_close,
    tbsip_submit_command, tbsip_submit_command_non_blocking, tbsip_test_interrupt_information,
    tbsip_test_mor_bit, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
