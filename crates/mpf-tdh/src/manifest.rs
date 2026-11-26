/// `STATUS_SUCCESS` from `include/ntstatus.h`.
pub const STATUS_SUCCESS: u32 = 0;

/// `TdhLoadManifest` from `dlls/tdh/tdh_main.c`. The C body only traces the manifest path and
/// returns `STATUS_SUCCESS`, so the port ignores the argument.
pub fn tdh_load_manifest(_manifest: Option<&[u16]>) -> u32 {
    STATUS_SUCCESS
}

/// `TdhLoadManifestFromBinary` from `dlls/tdh/tdh_main.c`. Same as [`tdh_load_manifest`]: the
/// argument is only traced and the return value is `STATUS_SUCCESS`.
pub fn tdh_load_manifest_from_binary(_binary: Option<&[u16]>) -> u32 {
    STATUS_SUCCESS
}
