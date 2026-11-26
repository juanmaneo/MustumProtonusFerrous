//! `@ stub` exports. Winebuild routes each one to `__wine_spec_unimplemented_stub` in
//! `libs/winecrt0/stub.c`, which raises `EXCEPTION_WINE_STUB` with the module and function names.

/// `EXCEPTION_WINE_STUB`, raised by `__wine_spec_unimplemented_stub`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// Module name the stubs report. Inferred from the spec file name, see the crate docs.
const MODULE: &str = "dsuiext";

/// The exception a stub raises: its code, flags, and the two arguments it passes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub`: the exception raised when `function` in `module` is called.
pub fn unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        module,
        function,
    }
}

/// `DllInstall`.
pub fn dll_install() -> StubException {
    unimplemented_stub(MODULE, "DllInstall")
}

/// `DsBrowseForContainerA`.
pub fn ds_browse_for_container_a() -> StubException {
    unimplemented_stub(MODULE, "DsBrowseForContainerA")
}

/// `DsBrowseForContainerW`.
pub fn ds_browse_for_container_w() -> StubException {
    unimplemented_stub(MODULE, "DsBrowseForContainerW")
}

/// `DsGetFriendlyClassName`.
pub fn ds_get_friendly_class_name() -> StubException {
    unimplemented_stub(MODULE, "DsGetFriendlyClassName")
}

/// `DsGetIcon`.
pub fn ds_get_icon() -> StubException {
    unimplemented_stub(MODULE, "DsGetIcon")
}
