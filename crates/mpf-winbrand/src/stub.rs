/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = winbrand.dll` in `dlls/winbrand/Makefile.in`.
const WINBRAND_DLL_NAME: &str = "winbrand.dll";

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

/// `BrandingFormatStringForEdition`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_format_string_for_edition() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingFormatStringForEdition")
}

/// `BrandingLoadBitmap`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_load_bitmap() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingLoadBitmap")
}

/// `BrandingLoadCursor`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_load_cursor() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingLoadCursor")
}

/// `BrandingLoadIcon`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_load_icon() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingLoadIcon")
}

/// `BrandingLoadImage`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_load_image() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingLoadImage")
}

/// `BrandingLoadNeutralStringForEdition`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_load_neutral_string_for_edition() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingLoadNeutralStringForEdition")
}

/// `BrandingLoadString`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_load_string() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingLoadString")
}

/// `BrandingLoadStringForEdition`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn branding_load_string_for_edition() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "BrandingLoadStringForEdition")
}

/// `GetEULAFile`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn get_eula_file() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "GetEULAFile")
}

/// `GetEULAFileEx`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn get_eula_file_ex() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "GetEULAFileEx")
}

/// `GetEULAInCurrentUILanguage`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn get_eula_in_current_ui_language() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "GetEULAInCurrentUILanguage")
}

/// `GetHinstanceByNamespace`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn get_hinstance_by_namespace() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "GetHinstanceByNamespace")
}

/// `GetInstalledEULAPath`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn get_installed_eula_path() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "GetInstalledEULAPath")
}

/// `InstallEULA`, exported as `@ stub` in `dlls/winbrand/winbrand.spec`.
pub fn install_eula() -> StubException {
    wine_spec_unimplemented_stub(WINBRAND_DLL_NAME, "InstallEULA")
}
