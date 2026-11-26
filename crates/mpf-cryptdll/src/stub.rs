/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = cryptdll.dll` in `dlls/cryptdll/Makefile.in`.
const CRYPTDLL_DLL_NAME: &str = "cryptdll.dll";

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

/// `CDBuildIntegrityVect`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_build_integrity_vect() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDBuildIntegrityVect")
}

/// `CDBuildVect`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_build_vect() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDBuildVect")
}

/// `CDFindCommonCSystem`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_find_common_csystem() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDFindCommonCSystem")
}

/// `CDFindCommonCSystemWithKey`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_find_common_csystem_with_key() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDFindCommonCSystemWithKey")
}

/// `CDGenerateRandomBits`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_generate_random_bits() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDGenerateRandomBits")
}

/// `CDLocateCSystem`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_locate_csystem() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDLocateCSystem")
}

/// `CDLocateCheckSum`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_locate_check_sum() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDLocateCheckSum")
}

/// `CDLocateRng`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_locate_rng() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDLocateRng")
}

/// `CDRegisterCSystem`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_register_csystem() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDRegisterCSystem")
}

/// `CDRegisterCheckSum`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_register_check_sum() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDRegisterCheckSum")
}

/// `CDRegisterRng`, exported as `@ stub` in `dlls/cryptdll/cryptdll.spec`.
pub fn cd_register_rng() -> StubException {
    wine_spec_unimplemented_stub(CRYPTDLL_DLL_NAME, "CDRegisterRng")
}
