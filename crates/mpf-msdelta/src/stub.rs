/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msdelta.dll` in `dlls/msdelta/Makefile.in`.
const MSDELTA_DLL_NAME: &str = "msdelta.dll";

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

/// `ApplyDeltaA`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn apply_delta_a() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "ApplyDeltaA")
}

/// `ApplyDeltaB`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn apply_delta_b() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "ApplyDeltaB")
}

/// `ApplyDeltaProvidedB`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn apply_delta_provided_b() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "ApplyDeltaProvidedB")
}

/// `ApplyDeltaW`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn apply_delta_w() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "ApplyDeltaW")
}

/// `CreateDeltaA`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn create_delta_a() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "CreateDeltaA")
}

/// `CreateDeltaB`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn create_delta_b() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "CreateDeltaB")
}

/// `CreateDeltaW`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn create_delta_w() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "CreateDeltaW")
}

/// `DeltaFree`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn delta_free() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "DeltaFree")
}

/// `DeltaNormalizeProvidedB`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn delta_normalize_provided_b() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "DeltaNormalizeProvidedB")
}

/// `GetDeltaInfoA`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn get_delta_info_a() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "GetDeltaInfoA")
}

/// `GetDeltaInfoB`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn get_delta_info_b() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "GetDeltaInfoB")
}

/// `GetDeltaInfoW`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn get_delta_info_w() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "GetDeltaInfoW")
}

/// `GetDeltaSignatureA`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn get_delta_signature_a() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "GetDeltaSignatureA")
}

/// `GetDeltaSignatureB`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn get_delta_signature_b() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "GetDeltaSignatureB")
}

/// `GetDeltaSignatureW`, exported as `@ stub` in `dlls/msdelta/msdelta.spec`.
pub fn get_delta_signature_w() -> StubException {
    wine_spec_unimplemented_stub(MSDELTA_DLL_NAME, "GetDeltaSignatureW")
}
