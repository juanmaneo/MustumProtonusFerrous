/// The names `dlls/xinput9_1_0/xinput9_1_0.spec` exports, in spec order. All four are `@ stdcall`
/// functions with bodies, and none is `@ stub`.
pub const EXPORTS: [&str; 4] = [
    "XInputGetCapabilities",
    "XInputGetDSoundAudioDeviceGuids",
    "XInputSetState",
    "XInputGetState",
];

/// Whether the spec exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}
