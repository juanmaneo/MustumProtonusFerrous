//! The `@ stub` exports of `dlls/sppc/sppc.spec`, one helper per export.
//!
//! `winebuild` points each `@ stub` export at `__wine_spec_unimplemented_stub`, so each
//! helper returns the record that call raises.

use super::stub::{wine_spec_unimplemented_stub, StubException, SPPC_DLL_NAME};

/// `SLCallServer`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_call_server() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLCallServer")
}

/// `SLpAuthenticateGenuineTicketResponse`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_authenticate_genuine_ticket_response() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpAuthenticateGenuineTicketResponse")
}

/// `SLpBeginGenuineTicketTransaction`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_begin_genuine_ticket_transaction() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpBeginGenuineTicketTransaction")
}

/// `SLpClearActivationInProgress`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_clear_activation_in_progress() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpClearActivationInProgress")
}

/// `SLpDepositDownlevelGenuineTicket`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_deposit_downlevel_genuine_ticket() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpDepositDownlevelGenuineTicket")
}

/// `SLpDepositTokenActivationResponse`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_deposit_token_activation_response() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpDepositTokenActivationResponse")
}

/// `SLpGenerateTokenActivationChallenge`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_generate_token_activation_challenge() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpGenerateTokenActivationChallenge")
}

/// `SLpGetGenuineBlob`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_get_genuine_blob() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpGetGenuineBlob")
}

/// `SLpGetGenuineLocal`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_get_genuine_local() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpGetGenuineLocal")
}

/// `SLpGetLicenseAcquisitionInfo`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_get_license_acquisition_info() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpGetLicenseAcquisitionInfo")
}

/// `SLpGetMSPidInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_get_ms_pid_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpGetMSPidInformation")
}

/// `SLpGetMachineUGUID`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_get_machine_uguid() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpGetMachineUGUID")
}

/// `SLpGetTokenActivationGrantInfo`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_get_token_activation_grant_info() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpGetTokenActivationGrantInfo")
}

/// `SLpIAActivateProduct`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_ia_activate_product() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpIAActivateProduct")
}

/// `SLpIsCurrentInstalledProductKeyDefaultKey`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_is_current_installed_product_key_default_key() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpIsCurrentInstalledProductKeyDefaultKey")
}

/// `SLpProcessVMPipeMessage`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_process_vm_pipe_message() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpProcessVMPipeMessage")
}

/// `SLpSetActivationInProgress`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_set_activation_in_progress() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpSetActivationInProgress")
}

/// `SLpTriggerServiceWorker`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_trigger_service_worker() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpTriggerServiceWorker")
}

/// `SLpVLActivateProduct`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn slp_vl_activate_product() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLpVLActivateProduct")
}

/// `SLConsumeRight`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_consume_right() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLConsumeRight")
}

/// `SLDepositMigrationBlob`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_deposit_migration_blob() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLDepositMigrationBlob")
}

/// `SLDepositOfflineConfirmationId`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_deposit_offline_confirmation_id() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLDepositOfflineConfirmationId")
}

/// `SLDepositOfflineConfirmationIdEx`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_deposit_offline_confirmation_id_ex() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLDepositOfflineConfirmationIdEx")
}

/// `SLDepositStoreToken`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_deposit_store_token() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLDepositStoreToken")
}

/// `SLFireEvent`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_fire_event() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLFireEvent")
}

/// `SLGatherMigrationBlob`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_gather_migration_blob() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGatherMigrationBlob")
}

/// `SLGatherMigrationBlobEx`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_gather_migration_blob_ex() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGatherMigrationBlobEx")
}

/// `SLGenerateOfflineInstallationId`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_generate_offline_installation_id() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGenerateOfflineInstallationId")
}

/// `SLGenerateOfflineInstallationIdEx`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_generate_offline_installation_id_ex() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGenerateOfflineInstallationIdEx")
}

/// `SLGetActiveLicenseInfo`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_active_license_info() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetActiveLicenseInfo")
}

/// `SLGetApplicationInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_application_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetApplicationInformation")
}

/// `SLGetApplicationPolicy`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_application_policy() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetApplicationPolicy")
}

/// `SLGetAuthenticationResult`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_authentication_result() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetAuthenticationResult")
}

/// `SLGetEncryptedPIDEx`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_encrypted_pid_ex() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetEncryptedPIDEx")
}

/// `SLGetGenuineInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_genuine_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetGenuineInformation")
}

/// `SLGetInstalledProductKeyIds`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_installed_product_key_ids() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetInstalledProductKeyIds")
}

/// `SLGetLicense`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_license() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetLicense")
}

/// `SLGetLicenseFileId`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_license_file_id() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetLicenseFileId")
}

/// `SLGetLicenseInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_license_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetLicenseInformation")
}

/// `SLGetPKeyId`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_p_key_id() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetPKeyId")
}

/// `SLGetPKeyInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_p_key_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetPKeyInformation")
}

/// `SLGetPolicyInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_policy_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetPolicyInformation")
}

/// `SLGetPolicyInformationDWORD`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_policy_information_dword() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetPolicyInformationDWORD")
}

/// `SLGetProductSkuInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_product_sku_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetProductSkuInformation")
}

/// `SLGetSLIDList`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_slid_list() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetSLIDList")
}

/// `SLGetServiceInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_get_service_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLGetServiceInformation")
}

/// `SLInstallLicense`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_install_license() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLInstallLicense")
}

/// `SLInstallProofOfPurchase`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_install_proof_of_purchase() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLInstallProofOfPurchase")
}

/// `SLInstallProofOfPurchaseEx`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_install_proof_of_purchase_ex() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLInstallProofOfPurchaseEx")
}

/// `SLIsGenuineLocalEx`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_is_genuine_local_ex() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLIsGenuineLocalEx")
}

/// `SLLoadApplicationPolicies`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_load_application_policies() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLLoadApplicationPolicies")
}

/// `SLPersistRTSPayloadOverride`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_persist_rts_payload_override() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLPersistRTSPayloadOverride")
}

/// `SLReArm`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_re_arm() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLReArm")
}

/// `SLRegisterEvent`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_register_event() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLRegisterEvent")
}

/// `SLRegisterPlugin`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_register_plugin() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLRegisterPlugin")
}

/// `SLSetAuthenticationData`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_set_authentication_data() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLSetAuthenticationData")
}

/// `SLSetCurrentProductKey`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_set_current_product_key() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLSetCurrentProductKey")
}

/// `SLSetGenuineInformation`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_set_genuine_information() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLSetGenuineInformation")
}

/// `SLUninstallLicense`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_uninstall_license() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLUninstallLicense")
}

/// `SLUninstallProofOfPurchase`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_uninstall_proof_of_purchase() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLUninstallProofOfPurchase")
}

/// `SLUnloadApplicationPolicies`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_unload_application_policies() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLUnloadApplicationPolicies")
}

/// `SLUnregisterEvent`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_unregister_event() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLUnregisterEvent")
}

/// `SLUnregisterPlugin`, exported as `@ stub` in `dlls/sppc/sppc.spec`.
pub fn sl_unregister_plugin() -> StubException {
    wine_spec_unimplemented_stub(SPPC_DLL_NAME, "SLUnregisterPlugin")
}
