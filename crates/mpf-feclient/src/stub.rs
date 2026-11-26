/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = feclient.dll` in `dlls/feclient/Makefile.in`.
const FECLIENT_DLL_NAME: &str = "feclient.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `EfsUtilGetCurrentKey`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_util_get_current_key() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsUtilGetCurrentKey")
}

/// `EdpAllowFileAccessForProcess`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_allow_file_access_for_process() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpAllowFileAccessForProcess")
}

/// `EdpConsumerCredentialCreate`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_consumer_credential_create() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpConsumerCredentialCreate")
}

/// `EdpConsumerCredentialQuery`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_consumer_credential_query() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpConsumerCredentialQuery")
}

/// `EdpContainerizeFile`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_containerize_file() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpContainerizeFile")
}

/// `EdpCredentialCreate`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_credential_create() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpCredentialCreate")
}

/// `EdpCredentialDelete`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_credential_delete() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpCredentialDelete")
}

/// `EdpCredentialExists`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_credential_exists() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpCredentialExists")
}

/// `EdpCredentialQuery`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_credential_query() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpCredentialQuery")
}

/// `EdpDecontainerizeFile`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_decontainerize_file() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpDecontainerizeFile")
}

/// `EdpDplPolicyEnabledForUser`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_dpl_policy_enabled_for_user() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpDplPolicyEnabledForUser")
}

/// `EdpDplStartCredServiceIfDplEnabledForUser`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_dpl_start_cred_service_if_dpl_enabled_for_user() -> StubException {
    wine_spec_unimplemented_stub(
        FECLIENT_DLL_NAME,
        "EdpDplStartCredServiceIfDplEnabledForUser",
    )
}

/// `EdpDplUpgradePinInfo`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_dpl_upgrade_pin_info() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpDplUpgradePinInfo")
}

/// `EdpDplUpgradeVerifyUser`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_dpl_upgrade_verify_user() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpDplUpgradeVerifyUser")
}

/// `EdpDplUserCredentialsSet`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_dpl_user_credentials_set() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpDplUserCredentialsSet")
}

/// `EdpDplUserUnlockComplete`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_dpl_user_unlock_complete() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpDplUserUnlockComplete")
}

/// `EdpDplUserUnlockStart`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_dpl_user_unlock_start() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpDplUserUnlockStart")
}

/// `EdpFree`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_free() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpFree")
}

/// `EdpGetContainerIdentity`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_get_container_identity() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpGetContainerIdentity")
}

/// `EdpGetCredServiceState`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_get_cred_service_state() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpGetCredServiceState")
}

/// `EdpIsConsumerDataProtectionEnforced`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_is_consumer_data_protection_enforced() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpIsConsumerDataProtectionEnforced")
}

/// `EdpIsConsumerDataProtectionSupported`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_is_consumer_data_protection_supported() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpIsConsumerDataProtectionSupported")
}

/// `EdpPurgeAppLearningEvents`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_purge_app_learning_events() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpPurgeAppLearningEvents")
}

/// `EdpQueryCredServiceInfo`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_query_cred_service_info() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpQueryCredServiceInfo")
}

/// `EdpQueryDplEnforcedPolicyOwnerIds`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_query_dpl_enforced_policy_owner_ids() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpQueryDplEnforcedPolicyOwnerIds")
}

/// `EdpQueryRevokedPolicyOwnerIds`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_query_revoked_policy_owner_ids() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpQueryRevokedPolicyOwnerIds")
}

/// `EdpRmsClearKeys`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_rms_clear_keys() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpRmsClearKeys")
}

/// `EdpSetCredServiceInfo`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_set_cred_service_info() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpSetCredServiceInfo")
}

/// `EdpUnprotectFile`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_unprotect_file() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpUnprotectFile")
}

/// `EdpWriteLogSiteLearningEvents`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn edp_write_log_site_learning_events() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EdpWriteLogSiteLearningEvents")
}

/// `EfsClientCloseFileRaw`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_close_file_raw() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientCloseFileRaw")
}

/// `EfsClientCopyFileRaw`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_copy_file_raw() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientCopyFileRaw")
}

/// `EfsClientDecryptFile`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_decrypt_file() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientDecryptFile")
}

/// `EfsClientDuplicateEncryptionInfo`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_duplicate_encryption_info() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientDuplicateEncryptionInfo")
}

/// `EfsClientEncryptFileEx`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_encrypt_file_ex() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientEncryptFileEx")
}

/// `EfsClientFileEncryptionStatus`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_file_encryption_status() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientFileEncryptionStatus")
}

/// `EfsClientFreeKeyInfo`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_free_key_info() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientFreeKeyInfo")
}

/// `EfsClientFreeProtectorList`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_free_protector_list() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientFreeProtectorList")
}

/// `EfsClientGetEncryptedFileVersion`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_get_encrypted_file_version() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientGetEncryptedFileVersion")
}

/// `EfsClientGetKeyInfo`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_get_key_info() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientGetKeyInfo")
}

/// `EfsClientOpenFileRaw`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_open_file_raw() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientOpenFileRaw")
}

/// `EfsClientQueryProtectors`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_query_protectors() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientQueryProtectors")
}

/// `EfsClientReadFileRaw`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_read_file_raw() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientReadFileRaw")
}

/// `EfsClientWriteFileRaw`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_write_file_raw() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientWriteFileRaw")
}

/// `EfsClientWriteFileWithHeaderRaw`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_client_write_file_with_header_raw() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsClientWriteFileWithHeaderRaw")
}

/// `EfsReprotectFile`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn efs_reprotect_file() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "EfsReprotectFile")
}

/// `FeClientInitialize`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn fe_client_initialize() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "FeClientInitialize")
}

/// `GetLockSessionUnwrappedKey`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn get_lock_session_unwrapped_key() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "GetLockSessionUnwrappedKey")
}

/// `GetLockSessionWrappedKey`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn get_lock_session_wrapped_key() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "GetLockSessionWrappedKey")
}

/// `OefsCheckSupport`, exported as `@ stub` in `dlls/feclient/feclient.spec`.
pub fn oefs_check_support() -> StubException {
    wine_spec_unimplemented_stub(FECLIENT_DLL_NAME, "OefsCheckSupport")
}
