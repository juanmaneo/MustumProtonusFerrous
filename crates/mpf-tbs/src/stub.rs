/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = tbs.dll` in `dlls/tbs/Makefile.in`.
const TBS_DLL_NAME: &str = "tbs.dll";

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

/// `Tbsi_Create_Attestation_From_Log`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_create_attestation_from_log() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Create_Attestation_From_Log")
}

/// `Tbsi_Get_TCG_Logs`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_get_tcg_logs() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Get_TCG_Logs")
}

/// `GetDeviceID`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn get_device_id() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "GetDeviceID")
}

/// `GetDeviceIDWithTimeout`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn get_device_id_with_timeout() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "GetDeviceIDWithTimeout")
}

/// `Tbsi_Create_Windows_Key`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_create_windows_key() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Create_Windows_Key")
}

/// `Tbsi_FilterLog`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_filter_log() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_FilterLog")
}

/// `Tbsi_Get_OwnerAuth`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_get_owner_auth() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Get_OwnerAuth")
}

/// `Tbsi_Get_TCG_Log`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_get_tcg_log() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Get_TCG_Log")
}

/// `Tbsi_Get_TCG_Log_Ex`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_get_tcg_log_ex() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Get_TCG_Log_Ex")
}

/// `Tbsi_Physical_Presence_Command`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_physical_presence_command() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Physical_Presence_Command")
}

/// `Tbsi_Revoke_Attestation`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_revoke_attestation() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_Revoke_Attestation")
}

/// `Tbsi_ShaHash`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsi_sha_hash() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsi_ShaHash")
}

/// `Tbsip_Cancel_Commands`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsip_cancel_commands() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsip_Cancel_Commands")
}

/// `Tbsip_Context_Close`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsip_context_close() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsip_Context_Close")
}

/// `Tbsip_Submit_Command`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsip_submit_command() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsip_Submit_Command")
}

/// `Tbsip_Submit_Command_NonBlocking`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsip_submit_command_non_blocking() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsip_Submit_Command_NonBlocking")
}

/// `Tbsip_TestInterruptInformation`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsip_test_interrupt_information() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsip_TestInterruptInformation")
}

/// `Tbsip_TestMorBit`, exported as `@ stub` in `dlls/tbs/tbs.spec`.
pub fn tbsip_test_mor_bit() -> StubException {
    wine_spec_unimplemented_stub(TBS_DLL_NAME, "Tbsip_TestMorBit")
}
