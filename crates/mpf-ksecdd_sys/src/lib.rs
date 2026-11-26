//! Port of the portable logic in Wine's `dlls/ksecdd.sys/main.c` and `dlls/ksecdd.sys/ksecdd.sys.spec`.
//!
//! Ported:
//! - `DriverEntry`, which returns `STATUS_SUCCESS` without using its arguments: [`driver_entry`],
//!   [`NtStatus`], [`STATUS_SUCCESS`]
//! - The 73 `@ stub` exports in `ksecdd.sys.spec`: one function per export, named by the snake_case
//!   of the export name, each returning the record that `__wine_spec_unimplemented_stub` raises:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], [`MODULE_NAME`], [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The 31 `@ stdcall` exports that forward to `bcrypt.dll`. `ksecdd.sys` has no code for them;
//!   the loader resolves each one in `bcrypt.dll`.
//! - The `driver` and `path` arguments of `DriverEntry`: the body reads `path->Buffer` only inside
//!   `TRACE`, which does not affect the return value.
//! - `TRACE` output.
//! - Ordinal exports: every `@ stub` export has a name, so the ordinal form of the `function`
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
    accept_security_context, acquire_credentials_handle_w, add_credentials_w, apply_control_token,
    b_crypt_enum_providers, b_crypt_register_config_change_notify, b_crypt_resolve_providers,
    b_crypt_unregister_config_change_notify, complete_auth_token, cred_marshal_target_info,
    delete_security_context, enumerate_security_packages_w, export_security_context,
    free_context_buffer, free_credentials_handle, get_security_user_info,
    impersonate_security_context, import_security_context_w, init_security_interface_w,
    initialize_security_context_w, k_sec_register_security_provider, k_sec_validate_buffer,
    lsa_enumerate_logon_sessions, lsa_get_logon_session_data, make_signature, map_security_error,
    query_context_attributes_w, query_credentials_attributes_w, query_security_context_token,
    query_security_package_info_w, revert_security_context, seal_message, sec_lookup_account_name,
    sec_lookup_account_sid, sec_lookup_well_known_sid, sec_make_spn, sec_make_spn_ex,
    sec_make_spn_ex2, sec_set_paging_mode, set_credentials_attributes_w, ssl_decrypt_packet,
    ssl_encrypt_packet, ssl_export_key, ssl_free_object, ssl_get_server_identity, ssl_import_key,
    ssl_lookup_cipher_suite_info, ssl_open_provider, sspi_accept_security_context_async,
    sspi_acquire_credentials_handle_async_w, sspi_compare_auth_identities, sspi_copy_auth_identity,
    sspi_create_async_context, sspi_delete_security_context_async,
    sspi_encode_auth_identity_as_strings, sspi_encode_strings_as_auth_identity,
    sspi_free_async_context, sspi_free_auth_identity, sspi_free_credentials_handle_async,
    sspi_get_async_call_status, sspi_initialize_security_context_async_w, sspi_local_free,
    sspi_marshal_auth_identity, sspi_reinit_async_context, sspi_set_async_notify_callback,
    sspi_unmarshal_auth_identity, sspi_validate_auth_identity, sspi_zero_auth_identity,
    system_prng, token_binding_get_key_types_server, token_binding_verify_message, unseal_message,
    verify_signature,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
