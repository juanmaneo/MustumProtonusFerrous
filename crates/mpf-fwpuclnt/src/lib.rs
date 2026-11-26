//! Port of the portable logic in Wine's `dlls/fwpuclnt/main.c`.
//!
//! Ported:
//! - The nine exports with a body in `main.c` (all `stdcall` in
//!   `fwpuclnt.spec`), modelled as their stub return values:
//!   `RPC_S_CANNOT_SUPPORT` and the `NULL` or `0` written to each out-parameter.
//!   Engine, sub-layer and memory: [`engine`]. Filter: [`filter`].
//!
//! Modelled, not ported:
//! - Arguments are not modelled, because the stubs ignore every input.
//! - `FwpmFreeMemory0` is a no-op, since the C stub neither reads nor frees `*mem`.
//!
//! Skipped:
//! - The 137 `stub` exports in `fwpuclnt.spec` that have no body in `main.c`.
//!   Their return values are not in this DLL's reference source, so no
//!   behaviour is modelled for them.
//! - The `FIXME` trace output, which is the debug channel.

pub mod engine;
pub mod filter;

pub use engine::{
    fwpm_engine_close0, fwpm_engine_open0, fwpm_free_memory0, fwpm_sub_layer_add0,
    fwpm_sub_layer_get_by_key0, Handle, NULL, RPC_S_CANNOT_SUPPORT,
};
pub use filter::{
    fwpm_filter_add0, fwpm_filter_create_enum_handle0, fwpm_filter_destroy_enum_handle0,
    fwpm_filter_enum0, FilterEnum,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
