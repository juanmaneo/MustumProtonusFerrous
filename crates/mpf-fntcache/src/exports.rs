//! Export table of Wine's `dlls/fntcache/fntcache.spec` checked against `main.c`.

/// The exports listed in `fntcache.spec`, in spec order.
pub const SPEC_EXPORTS: [&str; 3] = ["DllRegisterServer", "DllUnregisterServer", "ServiceMain"];

/// Whether `main.c` defines the export. `DllRegisterServer` and `DllUnregisterServer` are spec-only:
/// the reference has no body for them, so no return value is modelled.
pub fn defined_in_main_c(export: &str) -> bool {
    export == "ServiceMain"
}
