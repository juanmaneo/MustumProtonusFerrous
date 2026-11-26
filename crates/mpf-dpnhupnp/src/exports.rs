//! Export table of `dpnhupnp.spec`. Byte counts are for 32-bit x86, where `ptr` is 4 bytes.

pub const EXPORTS: [(&str, u32); 5] = [
    ("DirectPlayNATHelpCreate", 8),
    ("DllCanUnloadNow", 0),
    ("DllGetClassObject", 12),
    ("DllRegisterServer", 0),
    ("DllUnregisterServer", 0),
];

/// Returns the stdcall argument bytes of a named export, or `None` if it is not exported.
pub fn stdcall_arg_bytes(name: &str) -> Option<u32> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, bytes)| bytes)
}
