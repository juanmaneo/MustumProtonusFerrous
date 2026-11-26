/// `MODULE = ia2comproxy.dll` in `dlls/ia2comproxy/Makefile.in`.
pub const MODULE: &str = "ia2comproxy.dll";

/// `IMPORTS` in `dlls/ia2comproxy/Makefile.in`.
pub const IMPORTS: [&str; 5] = ["uuid", "oleacc", "oleaut32", "ole32", "rpcrt4"];

/// `dlldata_EXTRADEFS` in `dlls/ia2comproxy/Makefile.in`: the defines for the generated `dlldata.c`.
pub const DLLDATA_EXTRADEFS: [&str; 2] = ["-DWINE_REGISTER_DLL", "-DPROXY_DELEGATION"];

/// Whether `IMPORTS` lists `library`. Matching is case-sensitive.
pub fn is_import(library: &str) -> bool {
    IMPORTS.contains(&library)
}

/// Whether `DLLDATA_EXTRADEFS` contains `define`, such as `-DPROXY_DELEGATION`.
pub fn has_dlldata_define(define: &str) -> bool {
    DLLDATA_EXTRADEFS.contains(&define)
}
