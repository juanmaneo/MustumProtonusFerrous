/// The settings that `dlls/photometadatahandler/Makefile.in` sets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MakefileSettings {
    /// `MODULE`: the file name that `winebuild` passes to the stubs.
    pub module: &'static str,
    /// `EXTRADLLFLAGS`.
    pub extra_dll_flags: &'static str,
    /// `VER_FILEDESCRIPTION_STR`.
    pub file_description: &'static str,
    /// `VER_PRODUCTVERSION`, the four comma-separated fields.
    pub product_version: [u16; 4],
}

/// The settings from `dlls/photometadatahandler/Makefile.in`.
pub fn makefile_settings() -> MakefileSettings {
    MakefileSettings {
        module: "photometadatahandler.dll",
        extra_dll_flags: "-Wb,--prefer-native",
        file_description: "Wine photometadatahandler dll",
        product_version: [7, 0, 0, 0],
    }
}
