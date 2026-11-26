/// The `sensapi.spec` export table, in spec order. Each export is a `stdcall` function with a C
/// body in `dlls/sensapi/sensapi.c`.
pub const EXPORTS: [&str; 3] = [
    "IsDestinationReachableA",
    "IsDestinationReachableW",
    "IsNetworkAlive",
];

/// Whether `sensapi.spec` exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}
