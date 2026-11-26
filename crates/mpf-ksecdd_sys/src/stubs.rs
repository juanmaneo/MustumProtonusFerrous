use crate::stub::{wine_spec_unimplemented_stub, StubException, MODULE_NAME};

/// `SystemPrng`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn system_prng() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SystemPrng")
}

/// `AcceptSecurityContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn accept_security_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "AcceptSecurityContext")
}

/// `AcquireCredentialsHandleW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn acquire_credentials_handle_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "AcquireCredentialsHandleW")
}

/// `AddCredentialsW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn add_credentials_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "AddCredentialsW")
}

/// `ApplyControlToken`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn apply_control_token() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "ApplyControlToken")
}

/// `BCryptEnumProviders`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn b_crypt_enum_providers() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "BCryptEnumProviders")
}

/// `BCryptRegisterConfigChangeNotify`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn b_crypt_register_config_change_notify() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "BCryptRegisterConfigChangeNotify")
}

/// `BCryptResolveProviders`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn b_crypt_resolve_providers() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "BCryptResolveProviders")
}

/// `BCryptUnregisterConfigChangeNotify`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn b_crypt_unregister_config_change_notify() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "BCryptUnregisterConfigChangeNotify")
}

/// `CompleteAuthToken`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn complete_auth_token() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "CompleteAuthToken")
}

/// `CredMarshalTargetInfo`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn cred_marshal_target_info() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "CredMarshalTargetInfo")
}

/// `DeleteSecurityContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn delete_security_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "DeleteSecurityContext")
}

/// `EnumerateSecurityPackagesW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn enumerate_security_packages_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "EnumerateSecurityPackagesW")
}

/// `ExportSecurityContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn export_security_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "ExportSecurityContext")
}

/// `FreeContextBuffer`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn free_context_buffer() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "FreeContextBuffer")
}

/// `FreeCredentialsHandle`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn free_credentials_handle() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "FreeCredentialsHandle")
}

/// `GetSecurityUserInfo`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn get_security_user_info() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "GetSecurityUserInfo")
}

/// `ImpersonateSecurityContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn impersonate_security_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "ImpersonateSecurityContext")
}

/// `ImportSecurityContextW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn import_security_context_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "ImportSecurityContextW")
}

/// `InitSecurityInterfaceW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn init_security_interface_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "InitSecurityInterfaceW")
}

/// `InitializeSecurityContextW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn initialize_security_context_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "InitializeSecurityContextW")
}

/// `KSecRegisterSecurityProvider`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn k_sec_register_security_provider() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "KSecRegisterSecurityProvider")
}

/// `KSecValidateBuffer`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn k_sec_validate_buffer() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "KSecValidateBuffer")
}

/// `LsaEnumerateLogonSessions`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn lsa_enumerate_logon_sessions() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "LsaEnumerateLogonSessions")
}

/// `LsaGetLogonSessionData`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn lsa_get_logon_session_data() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "LsaGetLogonSessionData")
}

/// `MakeSignature`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn make_signature() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "MakeSignature")
}

/// `MapSecurityError`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn map_security_error() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "MapSecurityError")
}

/// `QueryContextAttributesW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn query_context_attributes_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "QueryContextAttributesW")
}

/// `QueryCredentialsAttributesW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn query_credentials_attributes_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "QueryCredentialsAttributesW")
}

/// `QuerySecurityContextToken`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn query_security_context_token() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "QuerySecurityContextToken")
}

/// `QuerySecurityPackageInfoW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn query_security_package_info_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "QuerySecurityPackageInfoW")
}

/// `RevertSecurityContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn revert_security_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "RevertSecurityContext")
}

/// `SealMessage`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn seal_message() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SealMessage")
}

/// `SecLookupAccountName`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sec_lookup_account_name() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SecLookupAccountName")
}

/// `SecLookupAccountSid`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sec_lookup_account_sid() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SecLookupAccountSid")
}

/// `SecLookupWellKnownSid`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sec_lookup_well_known_sid() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SecLookupWellKnownSid")
}

/// `SecMakeSPN`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sec_make_spn() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SecMakeSPN")
}

/// `SecMakeSPNEx`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sec_make_spn_ex() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SecMakeSPNEx")
}

/// `SecMakeSPNEx2`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sec_make_spn_ex2() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SecMakeSPNEx2")
}

/// `SecSetPagingMode`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sec_set_paging_mode() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SecSetPagingMode")
}

/// `SetCredentialsAttributesW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn set_credentials_attributes_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SetCredentialsAttributesW")
}

/// `SslDecryptPacket`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_decrypt_packet() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslDecryptPacket")
}

/// `SslEncryptPacket`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_encrypt_packet() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslEncryptPacket")
}

/// `SslExportKey`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_export_key() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslExportKey")
}

/// `SslFreeObject`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_free_object() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslFreeObject")
}

/// `SslGetServerIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_get_server_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslGetServerIdentity")
}

/// `SslImportKey`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_import_key() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslImportKey")
}

/// `SslLookupCipherSuiteInfo`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_lookup_cipher_suite_info() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslLookupCipherSuiteInfo")
}

/// `SslOpenProvider`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn ssl_open_provider() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SslOpenProvider")
}

/// `SspiAcceptSecurityContextAsync`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_accept_security_context_async() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiAcceptSecurityContextAsync")
}

/// `SspiAcquireCredentialsHandleAsyncW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_acquire_credentials_handle_async_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiAcquireCredentialsHandleAsyncW")
}

/// `SspiCompareAuthIdentities`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_compare_auth_identities() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiCompareAuthIdentities")
}

/// `SspiCopyAuthIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_copy_auth_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiCopyAuthIdentity")
}

/// `SspiCreateAsyncContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_create_async_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiCreateAsyncContext")
}

/// `SspiDeleteSecurityContextAsync`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_delete_security_context_async() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiDeleteSecurityContextAsync")
}

/// `SspiEncodeAuthIdentityAsStrings`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_encode_auth_identity_as_strings() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiEncodeAuthIdentityAsStrings")
}

/// `SspiEncodeStringsAsAuthIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_encode_strings_as_auth_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiEncodeStringsAsAuthIdentity")
}

/// `SspiFreeAsyncContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_free_async_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiFreeAsyncContext")
}

/// `SspiFreeAuthIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_free_auth_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiFreeAuthIdentity")
}

/// `SspiFreeCredentialsHandleAsync`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_free_credentials_handle_async() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiFreeCredentialsHandleAsync")
}

/// `SspiGetAsyncCallStatus`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_get_async_call_status() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiGetAsyncCallStatus")
}

/// `SspiInitializeSecurityContextAsyncW`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_initialize_security_context_async_w() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiInitializeSecurityContextAsyncW")
}

/// `SspiLocalFree`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_local_free() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiLocalFree")
}

/// `SspiMarshalAuthIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_marshal_auth_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiMarshalAuthIdentity")
}

/// `SspiReinitAsyncContext`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_reinit_async_context() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiReinitAsyncContext")
}

/// `SspiSetAsyncNotifyCallback`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_set_async_notify_callback() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiSetAsyncNotifyCallback")
}

/// `SspiUnmarshalAuthIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_unmarshal_auth_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiUnmarshalAuthIdentity")
}

/// `SspiValidateAuthIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_validate_auth_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiValidateAuthIdentity")
}

/// `SspiZeroAuthIdentity`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn sspi_zero_auth_identity() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "SspiZeroAuthIdentity")
}

/// `TokenBindingGetKeyTypesServer`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn token_binding_get_key_types_server() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "TokenBindingGetKeyTypesServer")
}

/// `TokenBindingVerifyMessage`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn token_binding_verify_message() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "TokenBindingVerifyMessage")
}

/// `UnsealMessage`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn unseal_message() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "UnsealMessage")
}

/// `VerifySignature`, exported as `@ stub` in `dlls/ksecdd.sys/ksecdd.sys.spec`.
pub fn verify_signature() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "VerifySignature")
}
