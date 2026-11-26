//! The `dlls/mfh264enc/mfh264enc.spec` export table.

/// The names in `mfh264enc.spec`, in spec order. All four are `stdcall -private`. None is `@ stub`.
pub const EXPORTS: [&str; 4] = [
    "DllCanUnloadNow",
    "DllGetClassObject",
    "DllRegisterServer",
    "DllUnregisterServer",
];

/// Whether `mfh264enc.spec` exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}
