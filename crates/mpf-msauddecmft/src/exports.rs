//! The `dlls/msauddecmft/msauddecmft.spec` export table.

/// The names in `msauddecmft.spec`, in spec order. All four are `stdcall -private`. None is `@ stub`.
pub const EXPORTS: [&str; 4] = [
    "DllCanUnloadNow",
    "DllGetClassObject",
    "DllRegisterServer",
    "DllUnregisterServer",
];

/// Whether `msauddecmft.spec` exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}
