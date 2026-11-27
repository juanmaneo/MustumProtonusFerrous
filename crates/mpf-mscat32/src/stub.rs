/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mscat32.dll` in `dlls/mscat32/Makefile.in`.
const MSCAT32_DLL_NAME: &str = "mscat32.dll";

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

/// `CryptCATVerifyMember`, exported as `1 stub` in `dlls/mscat32/mscat32.spec`.
pub fn crypt_cat_verify_member() -> StubException {
    wine_spec_unimplemented_stub(MSCAT32_DLL_NAME, "CryptCATVerifyMember")
}

/// `CryptCATCDFEnumAttributes`, exported as `@ stub` in `dlls/mscat32/mscat32.spec`.
pub fn crypt_cat_cdf_enum_attributes() -> StubException {
    wine_spec_unimplemented_stub(MSCAT32_DLL_NAME, "CryptCATCDFEnumAttributes")
}

/// `CryptCATCDFEnumMembers`, exported as `@ stub` in `dlls/mscat32/mscat32.spec`.
pub fn crypt_cat_cdf_enum_members() -> StubException {
    wine_spec_unimplemented_stub(MSCAT32_DLL_NAME, "CryptCATCDFEnumMembers")
}

/// `MsCatConstructHashTag`, exported as `@ stub` in `dlls/mscat32/mscat32.spec`.
pub fn ms_cat_construct_hash_tag() -> StubException {
    wine_spec_unimplemented_stub(MSCAT32_DLL_NAME, "MsCatConstructHashTag")
}

/// `MsCatFreeHashTag`, exported as `@ stub` in `dlls/mscat32/mscat32.spec`.
pub fn ms_cat_free_hash_tag() -> StubException {
    wine_spec_unimplemented_stub(MSCAT32_DLL_NAME, "MsCatFreeHashTag")
}

/// The stub record for `name`, or `None` when the spec does not export `name` as a stub.
/// Names are case-sensitive.
pub fn stub_for_export(name: &str) -> Option<StubException> {
    let stub = match name {
        "CryptCATVerifyMember" => crypt_cat_verify_member(),
        "CryptCATCDFEnumAttributes" => crypt_cat_cdf_enum_attributes(),
        "CryptCATCDFEnumMembers" => crypt_cat_cdf_enum_members(),
        "MsCatConstructHashTag" => ms_cat_construct_hash_tag(),
        "MsCatFreeHashTag" => ms_cat_free_hash_tag(),
        _ => return None,
    };
    Some(stub)
}
