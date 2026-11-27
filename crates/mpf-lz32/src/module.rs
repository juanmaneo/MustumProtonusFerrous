use crate::exports::EXPORTS;

/// `MODULE` in `dlls/lz32/Makefile.in`.
pub const MODULE_NAME: &str = "lz32.dll";

/// `IMPORTLIB` in `dlls/lz32/Makefile.in`.
pub const IMPORT_LIB: &str = "lz32";

/// `VER_PRODUCTVERSION` in `dlls/lz32/Makefile.in`.
pub const PRODUCT_VERSION: [u16; 4] = [5, 1, 2600, 2180];

/// The `.def` body that `winebuild --def -m32` writes for `lz32.spec`: the `LIBRARY` line and
/// the `EXPORTS` block. The generated header comment is left out.
pub fn def_body() -> String {
    let mut body = format!("LIBRARY {MODULE_NAME}\n\nEXPORTS\n");
    for export in &EXPORTS {
        body.push_str(&export.def_line());
        body.push('\n');
    }
    body
}
