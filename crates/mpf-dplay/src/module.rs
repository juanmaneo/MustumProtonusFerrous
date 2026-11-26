/// The DLL name that `winebuild` writes into the export directory, from `MODULE = dplay.dll` in
/// `dlls/dplay/Makefile.in`.
pub const MODULE_NAME: &str = "dplay.dll";

/// The `VER_*` values that `dlls/dplay/Makefile.in` passes to the version resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionInfo {
    /// `VER_FILEDESCRIPTION_STR`.
    pub file_description: &'static str,
    /// `VER_PRODUCTVERSION`, as its four parts.
    pub product_version: [u16; 4],
    /// Whether `VER_OLESELFREGISTER` is defined. `include/common.ver` only tests that it is
    /// defined, and then adds an `OleSelfRegister` value.
    pub ole_self_register: bool,
}

/// The version resource inputs from `dlls/dplay/Makefile.in`.
pub fn version_info() -> VersionInfo {
    VersionInfo {
        file_description: "Wine DirectPlay",
        product_version: [5, 3, 0, 900],
        ole_self_register: true,
    }
}
