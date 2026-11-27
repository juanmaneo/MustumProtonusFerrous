/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mssip32.dll` in `dlls/mssip32/Makefile.in`.
const MSSIP32_DLL_NAME: &str = "mssip32.dll";

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

/// `CryptSIPGetInfo`, exported as `@ stub` in `dlls/mssip32/mssip32.spec`.
pub fn crypt_sip_get_info() -> StubException {
    wine_spec_unimplemented_stub(MSSIP32_DLL_NAME, "CryptSIPGetInfo")
}

/// `CryptSIPGetRegWorkingFlags`, exported as `@ stub` in `dlls/mssip32/mssip32.spec`.
pub fn crypt_sip_get_reg_working_flags() -> StubException {
    wine_spec_unimplemented_stub(MSSIP32_DLL_NAME, "CryptSIPGetRegWorkingFlags")
}

/// `CryptSIPCreateIndirectData`, exported as `@ stub` in `dlls/mssip32/mssip32.spec`.
pub fn crypt_sip_create_indirect_data() -> StubException {
    wine_spec_unimplemented_stub(MSSIP32_DLL_NAME, "CryptSIPCreateIndirectData")
}

/// `CryptSIPGetSignedDataMsg`, exported as `@ stub` in `dlls/mssip32/mssip32.spec`.
pub fn crypt_sip_get_signed_data_msg() -> StubException {
    wine_spec_unimplemented_stub(MSSIP32_DLL_NAME, "CryptSIPGetSignedDataMsg")
}

/// `CryptSIPPutSignedDataMsg`, exported as `@ stub` in `dlls/mssip32/mssip32.spec`.
pub fn crypt_sip_put_signed_data_msg() -> StubException {
    wine_spec_unimplemented_stub(MSSIP32_DLL_NAME, "CryptSIPPutSignedDataMsg")
}

/// `CryptSIPRemoveSignedDataMsg`, exported as `@ stub` in `dlls/mssip32/mssip32.spec`.
pub fn crypt_sip_remove_signed_data_msg() -> StubException {
    wine_spec_unimplemented_stub(MSSIP32_DLL_NAME, "CryptSIPRemoveSignedDataMsg")
}

/// `CryptSIPVerifyIndirectData`, exported as `@ stub` in `dlls/mssip32/mssip32.spec`.
pub fn crypt_sip_verify_indirect_data() -> StubException {
    wine_spec_unimplemented_stub(MSSIP32_DLL_NAME, "CryptSIPVerifyIndirectData")
}
