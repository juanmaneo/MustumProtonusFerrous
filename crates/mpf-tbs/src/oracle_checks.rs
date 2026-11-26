//! Oracle-derived test vectors for the ported tbs logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `Tbsi_Context_Create`,
//! `Tbsi_GetDeviceInfo` and `GetDeviceIDString` (`dlls/tbs/tbs.c`) and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/tbs/`, and running the harness.
//! The harness calls each of the 21 `tbs.spec` exports once. The 21 printed rows are all kept:
//! ASan and UBSan report nothing for any of them, so no vector is excluded.

use super::*;

#[test]
fn oracle_explicit_rows() {
    // row 04 GetDeviceIDString(out,64,used,tpm) -> ret=0x80004001
    assert_eq!(get_device_id_string() as u32, 0x8000_4001);

    // row 06 Tbsi_Context_Create(params,out) -> ret=0x8028400f out_set=0
    assert_eq!(tbsi_context_create(), 0x8028_400f);

    // row 09 Tbsi_GetDeviceInfo(64,info) -> ret=0x8028400f
    assert_eq!(tbsi_get_device_info(), 0x8028_400f);
}

#[test]
fn oracle_stub_rows() {
    // row 01 Tbsi_Create_Attestation_From_Log -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Create_Attestation_From_Log
    assert_eq!(
        tbsi_create_attestation_from_log(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Create_Attestation_From_Log",
        }
    );

    // row 02 Tbsi_Get_TCG_Logs -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Get_TCG_Logs
    assert_eq!(
        tbsi_get_tcg_logs(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Get_TCG_Logs",
        }
    );

    // row 03 GetDeviceID -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=GetDeviceID
    assert_eq!(
        get_device_id(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "GetDeviceID",
        }
    );

    // row 05 GetDeviceIDWithTimeout -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=GetDeviceIDWithTimeout
    assert_eq!(
        get_device_id_with_timeout(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "GetDeviceIDWithTimeout",
        }
    );

    // row 07 Tbsi_Create_Windows_Key -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Create_Windows_Key
    assert_eq!(
        tbsi_create_windows_key(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Create_Windows_Key",
        }
    );

    // row 08 Tbsi_FilterLog -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_FilterLog
    assert_eq!(
        tbsi_filter_log(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_FilterLog",
        }
    );

    // row 10 Tbsi_Get_OwnerAuth -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Get_OwnerAuth
    assert_eq!(
        tbsi_get_owner_auth(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Get_OwnerAuth",
        }
    );

    // row 11 Tbsi_Get_TCG_Log -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Get_TCG_Log
    assert_eq!(
        tbsi_get_tcg_log(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Get_TCG_Log",
        }
    );

    // row 12 Tbsi_Get_TCG_Log_Ex -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Get_TCG_Log_Ex
    assert_eq!(
        tbsi_get_tcg_log_ex(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Get_TCG_Log_Ex",
        }
    );

    // row 13 Tbsi_Physical_Presence_Command -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Physical_Presence_Command
    assert_eq!(
        tbsi_physical_presence_command(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Physical_Presence_Command",
        }
    );

    // row 14 Tbsi_Revoke_Attestation -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_Revoke_Attestation
    assert_eq!(
        tbsi_revoke_attestation(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_Revoke_Attestation",
        }
    );

    // row 15 Tbsi_ShaHash -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsi_ShaHash
    assert_eq!(
        tbsi_sha_hash(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsi_ShaHash",
        }
    );

    // row 16 Tbsip_Cancel_Commands -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsip_Cancel_Commands
    assert_eq!(
        tbsip_cancel_commands(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsip_Cancel_Commands",
        }
    );

    // row 17 Tbsip_Context_Close -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsip_Context_Close
    assert_eq!(
        tbsip_context_close(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsip_Context_Close",
        }
    );

    // row 18 Tbsip_Submit_Command -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsip_Submit_Command
    assert_eq!(
        tbsip_submit_command(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsip_Submit_Command",
        }
    );

    // row 19 Tbsip_Submit_Command_NonBlocking -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsip_Submit_Command_NonBlocking
    assert_eq!(
        tbsip_submit_command_non_blocking(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsip_Submit_Command_NonBlocking",
        }
    );

    // row 20 Tbsip_TestInterruptInformation -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsip_TestInterruptInformation
    assert_eq!(
        tbsip_test_interrupt_information(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsip_TestInterruptInformation",
        }
    );

    // row 21 Tbsip_TestMorBit -> raised=1 count=1 code=0x80000100 flags=0x1 nparams=2 module=tbs.dll function=Tbsip_TestMorBit
    assert_eq!(
        tbsip_test_mor_bit(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "tbs.dll",
            function: "Tbsip_TestMorBit",
        }
    );
}
