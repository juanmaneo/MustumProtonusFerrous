/// `MODULE` in `dlls/dpnaddr/Makefile.in`.
pub const MODULE: &str = "dpnaddr.dll";

/// `VER_PRODUCTVERSION` in `dlls/dpnaddr/Makefile.in`.
pub const PRODUCT_VERSION: [u16; 4] = [5, 3, 2600, 2180];

/// `VER_FILEDESCRIPTION_STR` in `dlls/dpnaddr/Makefile.in`.
pub const FILE_DESCRIPTION: &str = "Wine DpnAddr dll";

/// `EXTRADLLFLAGS = -Wb,--data-only` in `dlls/dpnaddr/Makefile.in`.
pub const DATA_ONLY: bool = true;

/// winebuild's data-only check in `tools/winebuild/parser.c`: a data-only DLL accepts only
/// forwarded entry points.
pub fn data_only_accepts(is_forward: bool) -> bool {
    !DATA_ONLY || is_forward
}
