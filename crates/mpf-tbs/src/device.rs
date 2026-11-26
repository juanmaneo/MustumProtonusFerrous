/// `TBS_RESULT` from `include/tbs.h`.
pub type TbsResult = u32;

/// `HRESULT` from `include/winnt.h`, a signed 32-bit value.
pub type Hresult = i32;

/// `TBS_E_TPM_NOT_FOUND` from `include/winerror.h`.
pub const TBS_E_TPM_NOT_FOUND: TbsResult = 0x8028_400F;

/// `E_NOTIMPL` from `include/winerror.h`, stored as the signed `HRESULT` the C function returns.
pub const E_NOTIMPL: Hresult = 0x8000_4001_u32 as Hresult;

/// `Tbsi_Context_Create` from `dlls/tbs/tbs.c`. The C body traces its arguments and returns
/// `TBS_E_TPM_NOT_FOUND` without writing the context handle.
pub fn tbsi_context_create() -> TbsResult {
    TBS_E_TPM_NOT_FOUND
}

/// `Tbsi_GetDeviceInfo` from `dlls/tbs/tbs.c`. The C body traces its arguments and returns
/// `TBS_E_TPM_NOT_FOUND` without writing the info buffer.
pub fn tbsi_get_device_info() -> TbsResult {
    TBS_E_TPM_NOT_FOUND
}

/// `GetDeviceIDString` from `dlls/tbs/tbs.c`. The C body traces its arguments and returns
/// `E_NOTIMPL` without writing the output buffer.
pub fn get_device_id_string() -> Hresult {
    E_NOTIMPL
}
