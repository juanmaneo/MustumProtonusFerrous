/// `MODULE` in `dlls/msxml4/Makefile.in`.
pub const MODULE_NAME: &str = "msxml4.dll";

/// `VER_FILEDESCRIPTION_STR` in `dlls/msxml4/Makefile.in`.
pub const FILE_DESCRIPTION: &str = "Wine MSXML 4.0";

/// `VER_PRODUCTVERSION` in `dlls/msxml4/Makefile.in`.
pub const PRODUCT_VERSION: [u16; 4] = [4, 20, 9818, 0];

/// The resource type that `WINE_MANIFEST` passes in `dlls/msxml4/rsrc.rc`. It is `RT_MANIFEST`
/// in `include/winuser.h`.
pub const RT_MANIFEST: u16 = 24;

/// The manifest file that `WINE_MANIFEST` names in `dlls/msxml4/rsrc.rc`.
pub const MANIFEST_FILE: &str = "msxml4.manifest";

/// The `assemblyIdentity` element of `dlls/msxml4/msxml4.manifest`, plus the `file` element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssemblyIdentity {
    pub identity_type: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub processor_architecture: &'static str,
    pub public_key_token: &'static str,
    pub file: &'static str,
}

/// The assembly identity of `dlls/msxml4/msxml4.manifest`.
pub fn assembly_identity() -> AssemblyIdentity {
    AssemblyIdentity {
        identity_type: "win32",
        name: "Microsoft.MSXML2",
        version: "4.1.0.0",
        processor_architecture: "",
        public_key_token: "6bd6b9abf345378f",
        file: "msxml4.dll",
    }
}
