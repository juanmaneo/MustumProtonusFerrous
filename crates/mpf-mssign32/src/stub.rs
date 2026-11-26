/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mssign32.dll` in `dlls/mssign32/Makefile.in`.
const MSSIGN32_DLL_NAME: &str = "mssign32.dll";

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

/// `FreeCryptProvFromCert`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn free_crypt_prov_from_cert() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "FreeCryptProvFromCert")
}

/// `GetCryptProvFromCert`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn get_crypt_prov_from_cert() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "GetCryptProvFromCert")
}

/// `PvkPrivateKeyAcquireContext`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_acquire_context() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyAcquireContext")
}

/// `PvkPrivateKeyAcquireContextA`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_acquire_context_a() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyAcquireContextA")
}

/// `PvkPrivateKeyAcquireContextFromMemoryA`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_acquire_context_from_memory_a() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyAcquireContextFromMemoryA")
}

/// `PvkPrivateKeyLoad`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_load() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyLoad")
}

/// `PvkPrivateKeyLoadA`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_load_a() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyLoadA")
}

/// `PvkPrivateKeyLoadFromMemory`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_load_from_memory() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyLoadFromMemory")
}

/// `PvkPrivateKeyLoadFromMemoryA`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_load_from_memory_a() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyLoadFromMemoryA")
}

/// `PvkPrivateKeyReleaseContext`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_release_context() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyReleaseContext")
}

/// `PvkPrivateKeyReleaseContextA`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_release_context_a() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeyReleaseContextA")
}

/// `PvkPrivateKeySave`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_save() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeySave")
}

/// `PvkPrivateKeySaveA`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_save_a() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeySaveA")
}

/// `PvkPrivateKeySaveToMemory`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_save_to_memory() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeySaveToMemory")
}

/// `PvkPrivateKeySaveToMemoryA`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn pvk_private_key_save_to_memory_a() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "PvkPrivateKeySaveToMemoryA")
}

/// `SignError`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn sign_error() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "SignError")
}

/// `SignerAddTimeStampResponse`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn signer_add_time_stamp_response() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "SignerAddTimeStampResponse")
}

/// `SignerAddTimeStampResponseEx`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn signer_add_time_stamp_response_ex() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "SignerAddTimeStampResponseEx")
}

/// `SignerCreateTimeStampRequest`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn signer_create_time_stamp_request() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "SignerCreateTimeStampRequest")
}

/// `SignerTimeStamp`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn signer_time_stamp() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "SignerTimeStamp")
}

/// `SignerTimeStampEx`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn signer_time_stamp_ex() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "SignerTimeStampEx")
}

/// `SpcGetCertFromKey`, an `@ stub` export in `dlls/mssign32/mssign32.spec`.
pub fn spc_get_cert_from_key() -> StubException {
    wine_spec_unimplemented_stub(MSSIGN32_DLL_NAME, "SpcGetCertFromKey")
}
