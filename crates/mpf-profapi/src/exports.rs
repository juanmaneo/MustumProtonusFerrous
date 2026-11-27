/// The `profapi.spec` export table, in spec order. Every export is an unnamed `@ stub`, so its
/// name is the decimal ordinal that `winebuild` passes to `__wine_spec_unimplemented_stub`.
pub const EXPORTS: [&str; 17] = [
    "101", "102", "103", "104", "105", "106", "107", "108", "109", "110", "111", "112", "113",
    "114", "115", "116", "117",
];

/// Whether `dlls/profapi/profapi.spec` exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}
