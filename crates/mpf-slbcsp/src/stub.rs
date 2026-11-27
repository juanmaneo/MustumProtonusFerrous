/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = slbcsp.dll` in `dlls/slbcsp/Makefile.in`.
const SLBCSP_DLL_NAME: &str = "slbcsp.dll";

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

/// `CPAcquireContext`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_acquire_context() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPAcquireContext")
}

/// `CPSetHashParam`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_set_hash_param() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPSetHashParam")
}

/// `CPSetKeyParam`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_set_key_param() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPSetKeyParam")
}

/// `CPSetProvParam`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_set_prov_param() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPSetProvParam")
}

/// `CPCreateHash`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_create_hash() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPCreateHash")
}

/// `CPDecrypt`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_decrypt() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPDecrypt")
}

/// `CPDeriveKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_derive_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPDeriveKey")
}

/// `CPDestroyHash`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_destroy_hash() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPDestroyHash")
}

/// `CPDestroyKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_destroy_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPDestroyKey")
}

/// `CPDuplicateHash`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_duplicate_hash() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPDuplicateHash")
}

/// `CPDuplicateKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_duplicate_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPDuplicateKey")
}

/// `CPEncrypt`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_encrypt() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPEncrypt")
}

/// `CPExportKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_export_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPExportKey")
}

/// `CPGenKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_gen_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPGenKey")
}

/// `CPGenRandom`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_gen_random() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPGenRandom")
}

/// `CPGetHashParam`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_get_hash_param() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPGetHashParam")
}

/// `CPGetKeyParam`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_get_key_param() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPGetKeyParam")
}

/// `CPGetProvParam`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_get_prov_param() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPGetProvParam")
}

/// `CPGetUserKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_get_user_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPGetUserKey")
}

/// `CPHashData`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_hash_data() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPHashData")
}

/// `CPHashSessionKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_hash_session_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPHashSessionKey")
}

/// `CPImportKey`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_import_key() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPImportKey")
}

/// `CPReleaseContext`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_release_context() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPReleaseContext")
}

/// `CPSignHash`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_sign_hash() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPSignHash")
}

/// `CPVerifySignature`, exported as `@ stub` in `dlls/slbcsp/slbcsp.spec`.
pub fn cp_verify_signature() -> StubException {
    wine_spec_unimplemented_stub(SLBCSP_DLL_NAME, "CPVerifySignature")
}
