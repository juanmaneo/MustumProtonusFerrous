//! The `dlls/msttsengine/msttsengine.spec` export table.

/// The names in `msttsengine.spec`, in spec order. All four are `stdcall -private`. None is `@ stub`.
pub const EXPORTS: [&str; 4] = [
    "DllCanUnloadNow",
    "DllGetClassObject",
    "DllRegisterServer",
    "DllUnregisterServer",
];

/// Whether `msttsengine.spec` exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}
