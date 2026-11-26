//! Port of the portable logic in Wine's `dlls/feclient/feclient.spec` and `dlls/feclient/Makefile.in`.
//!
//! Ported:
//! - `feclient.spec` export table: [`EXPORTS`], [`is_export`]
//! - The 50 `@ stub` exports, one helper each, and the `__wine_spec_unimplemented_stub` record
//!   they raise: [`wine_spec_unimplemented_stub`], [`StubException`], and the
//!   `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE` values
//!
//! Skipped:
//! - The reference has no `.c`, `.idl`, `.rc` or `.tlb` file, so there is no function body to
//!   port. Every export is `@ stub`, so no export returns a value. The helpers model the record
//!   that the stub raises; the port returns it instead of looping forever.
//! - The `__cdecl` calling convention, the ordinals (every export has a name), and the
//!   `RtlRaiseException` loop.

mod exports;
mod stub;

pub use exports::{is_export, EXPORTS};
pub use stub::{
    edp_allow_file_access_for_process, edp_consumer_credential_create,
    edp_consumer_credential_query, edp_containerize_file, edp_credential_create,
    edp_credential_delete, edp_credential_exists, edp_credential_query, edp_decontainerize_file,
    edp_dpl_policy_enabled_for_user, edp_dpl_start_cred_service_if_dpl_enabled_for_user,
    edp_dpl_upgrade_pin_info, edp_dpl_upgrade_verify_user, edp_dpl_user_credentials_set,
    edp_dpl_user_unlock_complete, edp_dpl_user_unlock_start, edp_free, edp_get_container_identity,
    edp_get_cred_service_state, edp_is_consumer_data_protection_enforced,
    edp_is_consumer_data_protection_supported, edp_purge_app_learning_events,
    edp_query_cred_service_info, edp_query_dpl_enforced_policy_owner_ids,
    edp_query_revoked_policy_owner_ids, edp_rms_clear_keys, edp_set_cred_service_info,
    edp_unprotect_file, edp_write_log_site_learning_events, efs_client_close_file_raw,
    efs_client_copy_file_raw, efs_client_decrypt_file, efs_client_duplicate_encryption_info,
    efs_client_encrypt_file_ex, efs_client_file_encryption_status, efs_client_free_key_info,
    efs_client_free_protector_list, efs_client_get_encrypted_file_version, efs_client_get_key_info,
    efs_client_open_file_raw, efs_client_query_protectors, efs_client_read_file_raw,
    efs_client_write_file_raw, efs_client_write_file_with_header_raw, efs_reprotect_file,
    efs_util_get_current_key, fe_client_initialize, get_lock_session_unwrapped_key,
    get_lock_session_wrapped_key, oefs_check_support, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
