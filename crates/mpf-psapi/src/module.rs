use crate::exports::EXPORTS;

/// `MODULE` in `dlls/psapi/Makefile.in`.
pub const MODULE_NAME: &str = "psapi.dll";

/// `IMPORTLIB` in `dlls/psapi/Makefile.in`.
pub const IMPORT_LIB: &str = "psapi";

/// `VER_PRODUCTVERSION` in `dlls/psapi/Makefile.in`.
pub const PRODUCT_VERSION: [u16; 4] = [5, 1, 2600, 3264];

/// `VER_FILEDESCRIPTION_STR` in `dlls/psapi/Makefile.in`.
pub const FILE_DESCRIPTION: &str = "Wine Process Status Helper";

/// The `.def` body that `winebuild --def -m32` writes for `psapi.spec`: the `LIBRARY` line and
/// the `EXPORTS` block. The generated header comment is left out.
pub fn def_body() -> String {
    let mut body = format!("LIBRARY {MODULE_NAME}\n\nEXPORTS\n");
    for export in &EXPORTS {
        body.push_str(&export.def_line());
        body.push('\n');
    }
    body
}
