//! Oracle-derived test vectors for the ported sppc logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `SLOpen`, `SLClose`,
//! `SLPersistApplicationPolicies` and `SLGetLicensingStatusInformation` (`dlls/sppc/sppc.c`) and
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/sppc/`, and running the harness.
//! The harness printed 77 rows: rows 01-14 for the four explicit functions and rows 15-77 for the
//! 63 `@ stub` exports. ASan and UBSan flagged no row, so none are excluded.

use super::*;

/// Stands in for the non-NULL `SLID` pointers the C harness passes. Only NULL-ness is ported.
const APP: Slid = [0x11; 16];
const PRODUCT: Slid = [0x22; 16];

fn stub_fields(s: StubException) -> (u32, u32, u32, &'static str, &'static str) {
    (s.code, s.flags, s.number_parameters, s.module, s.function)
}

#[test]
fn oracle_explicit_rows() {
    // row 01 SLOpen handle=NULL -> ret=0x80070057
    assert_eq!(sl_open(None), E_INVALIDARG);

    // row 02 SLOpen handle=&h h_in=0x1234 -> ret=0x00000000 handle=0xdeadbeef
    let mut handle = 0x1234;
    assert_eq!(sl_open(Some(&mut handle)), S_OK);
    assert_eq!(handle, 0xdead_beef);

    // row 03 SLOpen handle=&h h_in=NULL -> ret=0x00000000 handle=0xdeadbeef
    let mut handle = 0;
    assert_eq!(sl_open(Some(&mut handle)), S_OK);
    assert_eq!(handle, 0xdead_beef);

    // row 04 SLClose handle=NULL -> ret=0x00000000
    assert_eq!(sl_close(0), S_OK);
    // row 05 SLClose handle=0xdeadbeef -> ret=0x00000000
    assert_eq!(sl_close(0xdead_beef), S_OK);
    // row 06 SLClose handle=0x1234 -> ret=0x00000000
    assert_eq!(sl_close(0x1234), S_OK);

    // row 07 SLPersist app=NULL product=NULL flags=0 -> ret=0x80070057
    assert_eq!(sl_persist_application_policies(None, None, 0), E_INVALIDARG);
    // row 08 SLPersist app=&a product=NULL flags=0 -> ret=0x00000000
    assert_eq!(sl_persist_application_policies(Some(&APP), None, 0), S_OK);
    // row 09 SLPersist app=NULL product=&p flags=0xffffffff -> ret=0x80070057
    assert_eq!(
        sl_persist_application_policies(None, Some(&PRODUCT), 0xffff_ffff),
        E_INVALIDARG
    );
    // row 10 SLPersist app=&a product=&p flags=0x1 -> ret=0x00000000
    assert_eq!(
        sl_persist_application_policies(Some(&APP), Some(&PRODUCT), 0x1),
        S_OK
    );
    // row 11 SLPersist app=&a product=NULL flags=0xffffffff -> ret=0x00000000
    assert_eq!(
        sl_persist_application_policies(Some(&APP), None, 0xffff_ffff),
        S_OK
    );

    // row 12 SLGetLicensingStatus all=NULL -> ret=0xc004f002
    assert_eq!(
        sl_get_licensing_status_information(),
        SL_E_RIGHT_NOT_CONSUMED
    );
    // row 13 SLGetLicensingStatus handle=0x1234 app=&a product=&p name=x -> ret=0xc004f002
    assert_eq!(
        sl_get_licensing_status_information(),
        SL_E_RIGHT_NOT_CONSUMED
    );
    // row 14 SLGetLicensingStatus handle=NULL app=&a product=NULL name=NULL -> ret=0xc004f002
    assert_eq!(
        sl_get_licensing_status_information(),
        SL_E_RIGHT_NOT_CONSUMED
    );
}

#[test]
fn oracle_stub_rows() {
    // row 15
    assert_eq!(
        stub_fields(sl_call_server()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLCallServer")
    );
    // row 16
    assert_eq!(
        stub_fields(slp_authenticate_genuine_ticket_response()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpAuthenticateGenuineTicketResponse"
        )
    );
    // row 17
    assert_eq!(
        stub_fields(slp_begin_genuine_ticket_transaction()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpBeginGenuineTicketTransaction"
        )
    );
    // row 18
    assert_eq!(
        stub_fields(slp_clear_activation_in_progress()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpClearActivationInProgress"
        )
    );
    // row 19
    assert_eq!(
        stub_fields(slp_deposit_downlevel_genuine_ticket()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpDepositDownlevelGenuineTicket"
        )
    );
    // row 20
    assert_eq!(
        stub_fields(slp_deposit_token_activation_response()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpDepositTokenActivationResponse"
        )
    );
    // row 21
    assert_eq!(
        stub_fields(slp_generate_token_activation_challenge()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpGenerateTokenActivationChallenge"
        )
    );
    // row 22
    assert_eq!(
        stub_fields(slp_get_genuine_blob()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpGetGenuineBlob")
    );
    // row 23
    assert_eq!(
        stub_fields(slp_get_genuine_local()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpGetGenuineLocal")
    );
    // row 24
    assert_eq!(
        stub_fields(slp_get_license_acquisition_info()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpGetLicenseAcquisitionInfo"
        )
    );
    // row 25
    assert_eq!(
        stub_fields(slp_get_ms_pid_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpGetMSPidInformation")
    );
    // row 26
    assert_eq!(
        stub_fields(slp_get_machine_uguid()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpGetMachineUGUID")
    );
    // row 27
    assert_eq!(
        stub_fields(slp_get_token_activation_grant_info()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpGetTokenActivationGrantInfo"
        )
    );
    // row 28
    assert_eq!(
        stub_fields(slp_ia_activate_product()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpIAActivateProduct")
    );
    // row 29
    assert_eq!(
        stub_fields(slp_is_current_installed_product_key_default_key()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLpIsCurrentInstalledProductKeyDefaultKey"
        )
    );
    // row 30
    assert_eq!(
        stub_fields(slp_process_vm_pipe_message()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpProcessVMPipeMessage")
    );
    // row 31
    assert_eq!(
        stub_fields(slp_set_activation_in_progress()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpSetActivationInProgress")
    );
    // row 32
    assert_eq!(
        stub_fields(slp_trigger_service_worker()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpTriggerServiceWorker")
    );
    // row 33
    assert_eq!(
        stub_fields(slp_vl_activate_product()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLpVLActivateProduct")
    );
    // row 34
    assert_eq!(
        stub_fields(sl_consume_right()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLConsumeRight")
    );
    // row 35
    assert_eq!(
        stub_fields(sl_deposit_migration_blob()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLDepositMigrationBlob")
    );
    // row 36
    assert_eq!(
        stub_fields(sl_deposit_offline_confirmation_id()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLDepositOfflineConfirmationId"
        )
    );
    // row 37
    assert_eq!(
        stub_fields(sl_deposit_offline_confirmation_id_ex()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLDepositOfflineConfirmationIdEx"
        )
    );
    // row 38
    assert_eq!(
        stub_fields(sl_deposit_store_token()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLDepositStoreToken")
    );
    // row 39
    assert_eq!(
        stub_fields(sl_fire_event()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLFireEvent")
    );
    // row 40
    assert_eq!(
        stub_fields(sl_gather_migration_blob()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGatherMigrationBlob")
    );
    // row 41
    assert_eq!(
        stub_fields(sl_gather_migration_blob_ex()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGatherMigrationBlobEx")
    );
    // row 42
    assert_eq!(
        stub_fields(sl_generate_offline_installation_id()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLGenerateOfflineInstallationId"
        )
    );
    // row 43
    assert_eq!(
        stub_fields(sl_generate_offline_installation_id_ex()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLGenerateOfflineInstallationIdEx"
        )
    );
    // row 44
    assert_eq!(
        stub_fields(sl_get_active_license_info()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetActiveLicenseInfo")
    );
    // row 45
    assert_eq!(
        stub_fields(sl_get_application_information()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLGetApplicationInformation"
        )
    );
    // row 46
    assert_eq!(
        stub_fields(sl_get_application_policy()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetApplicationPolicy")
    );
    // row 47
    assert_eq!(
        stub_fields(sl_get_authentication_result()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetAuthenticationResult")
    );
    // row 48
    assert_eq!(
        stub_fields(sl_get_encrypted_pid_ex()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetEncryptedPIDEx")
    );
    // row 49
    assert_eq!(
        stub_fields(sl_get_genuine_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetGenuineInformation")
    );
    // row 50
    assert_eq!(
        stub_fields(sl_get_installed_product_key_ids()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLGetInstalledProductKeyIds"
        )
    );
    // row 51
    assert_eq!(
        stub_fields(sl_get_license()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetLicense")
    );
    // row 52
    assert_eq!(
        stub_fields(sl_get_license_file_id()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetLicenseFileId")
    );
    // row 53
    assert_eq!(
        stub_fields(sl_get_license_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetLicenseInformation")
    );
    // row 54
    assert_eq!(
        stub_fields(sl_get_p_key_id()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetPKeyId")
    );
    // row 55
    assert_eq!(
        stub_fields(sl_get_p_key_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetPKeyInformation")
    );
    // row 56
    assert_eq!(
        stub_fields(sl_get_policy_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetPolicyInformation")
    );
    // row 57
    assert_eq!(
        stub_fields(sl_get_policy_information_dword()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLGetPolicyInformationDWORD"
        )
    );
    // row 58
    assert_eq!(
        stub_fields(sl_get_product_sku_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetProductSkuInformation")
    );
    // row 59
    assert_eq!(
        stub_fields(sl_get_slid_list()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetSLIDList")
    );
    // row 60
    assert_eq!(
        stub_fields(sl_get_service_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLGetServiceInformation")
    );
    // row 61
    assert_eq!(
        stub_fields(sl_install_license()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLInstallLicense")
    );
    // row 62
    assert_eq!(
        stub_fields(sl_install_proof_of_purchase()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLInstallProofOfPurchase")
    );
    // row 63
    assert_eq!(
        stub_fields(sl_install_proof_of_purchase_ex()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLInstallProofOfPurchaseEx")
    );
    // row 64
    assert_eq!(
        stub_fields(sl_is_genuine_local_ex()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLIsGenuineLocalEx")
    );
    // row 65
    assert_eq!(
        stub_fields(sl_load_application_policies()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLLoadApplicationPolicies")
    );
    // row 66
    assert_eq!(
        stub_fields(sl_persist_rts_payload_override()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLPersistRTSPayloadOverride"
        )
    );
    // row 67
    assert_eq!(
        stub_fields(sl_re_arm()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLReArm")
    );
    // row 68
    assert_eq!(
        stub_fields(sl_register_event()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLRegisterEvent")
    );
    // row 69
    assert_eq!(
        stub_fields(sl_register_plugin()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLRegisterPlugin")
    );
    // row 70
    assert_eq!(
        stub_fields(sl_set_authentication_data()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLSetAuthenticationData")
    );
    // row 71
    assert_eq!(
        stub_fields(sl_set_current_product_key()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLSetCurrentProductKey")
    );
    // row 72
    assert_eq!(
        stub_fields(sl_set_genuine_information()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLSetGenuineInformation")
    );
    // row 73
    assert_eq!(
        stub_fields(sl_uninstall_license()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLUninstallLicense")
    );
    // row 74
    assert_eq!(
        stub_fields(sl_uninstall_proof_of_purchase()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLUninstallProofOfPurchase")
    );
    // row 75
    assert_eq!(
        stub_fields(sl_unload_application_policies()),
        (
            0x80000100,
            0x1,
            2,
            "sppc.dll",
            "SLUnloadApplicationPolicies"
        )
    );
    // row 76
    assert_eq!(
        stub_fields(sl_unregister_event()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLUnregisterEvent")
    );
    // row 77
    assert_eq!(
        stub_fields(sl_unregister_plugin()),
        (0x80000100, 0x1, 2, "sppc.dll", "SLUnregisterPlugin")
    );
}
