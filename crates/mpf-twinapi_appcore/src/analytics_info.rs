//! `analytics_version_info_get_DeviceFamilyVersion` in Wine's `analytics_info.c`: the packing of the OS version into a decimal string.
//!
//! Skipped: the `UBR` registry read (the caller passes `revision`, which Wine sets to 0 when the read fails) and the PEB reads (the caller passes the fields).

/// Packs the OS version the way Wine does and returns it in decimal.
///
/// `major`, `minor` and `build` are each masked to 16 bits and shifted in. `revision` is not masked and is OR-ed into the low 32 bits. Its bits 16..31 therefore land on the build field, so `(build 1, revision 0)` and `(build 0, revision 0x10000)` give the same string.
pub fn device_family_version(major: u32, minor: u32, build: u32, revision: u32) -> String {
    let mut version = u64::from(major & 0xffff);
    version = (version << 16) | u64::from(minor & 0xffff);
    version = (version << 16) | u64::from(build & 0xffff);
    version = (version << 16) | u64::from(revision);
    version.to_string()
}
