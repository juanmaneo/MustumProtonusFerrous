//! Engine, sub-layer and memory exports from Wine's `dlls/fwpuclnt/main.c`.
//!
//! Each export is a stub: it logs `FIXME` and reports `RPC_S_CANNOT_SUPPORT`.
//! Out-parameters that the C body writes are returned as values, with `NULL`
//! wherever the C body writes `NULL`.

/// `RPC_S_CANNOT_SUPPORT` from `winerror.h`.
pub const RPC_S_CANNOT_SUPPORT: u32 = 1764;

/// A pointer or handle value as the C code sees it. `NULL` is `0`.
pub type Handle = usize;

/// `NULL` from the C headers.
pub const NULL: Handle = 0;

/// `FwpmFreeMemory0`. The stub neither reads nor frees `*mem`, so it has no
/// observable effect.
pub fn fwpm_free_memory0() {}

/// `FwpmEngineOpen0`. Returns `(status, *engineHandle)`.
pub fn fwpm_engine_open0() -> (u32, Handle) {
    (RPC_S_CANNOT_SUPPORT, NULL)
}

/// `FwpmEngineClose0`.
pub fn fwpm_engine_close0() -> u32 {
    RPC_S_CANNOT_SUPPORT
}

/// `FwpmSubLayerAdd0`.
pub fn fwpm_sub_layer_add0() -> u32 {
    RPC_S_CANNOT_SUPPORT
}

/// `FwpmSubLayerGetByKey0`. Returns `(status, *subLayer)`.
pub fn fwpm_sub_layer_get_by_key0() -> (u32, Handle) {
    (RPC_S_CANNOT_SUPPORT, NULL)
}
