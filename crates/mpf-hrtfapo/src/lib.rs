//! Port of the portable logic in Wine's `dlls/hrtfapo/main.c` and `dlls/hrtfapo/hrtfapo.spec`.
//!
//! Ported:
//! - `CreateHrtfApo` (`@ stdcall`), whose body only returns `E_NOTIMPL`: [`create_hrtf_apo`],
//!   with [`E_NOTIMPL`]
//! - `hrtfapo.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The four `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`create_hrtf_apo_with_dataset_type`], [`create_hrtf_engine_factory`],
//!   [`is_hrtf_apo_available`], [`get_hrtf_engine_min_frame_count`],
//!   [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `FIXME` trace of the arguments, which the C body only prints.
//! - The `WINAPI` (`stdcall`) calling convention, and the `HrtfApoInit` and `IXAPO` COM types from
//!   `include/hrtfapoapi.idl` and `include/xapo.idl`. The port has no COM objects, so the pointer
//!   arguments are generic.
//! - The ordinal form of the stub's function argument. Every export in `hrtfapo.spec` is named.

mod apo;
mod exports;
mod stub;

pub use apo::{create_hrtf_apo, E_NOTIMPL};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    create_hrtf_apo_with_dataset_type, create_hrtf_engine_factory, get_hrtf_engine_min_frame_count,
    is_hrtf_apo_available, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
