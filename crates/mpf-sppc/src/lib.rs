//! Port of the portable logic in Wine's `dlls/sppc/sppc.c` and `dlls/sppc/sppc.spec`.
//!
//! Ported:
//! - `SLOpen` and `SLClose`: [`sl_open`], [`sl_close`], [`Hslc`], [`SLC_HANDLE`], with the
//!   `HRESULT` values [`S_OK`] and [`E_INVALIDARG`]
//! - `SLPersistApplicationPolicies`: [`sl_persist_application_policies`] and [`Slid`]
//! - `SLGetLicensingStatusInformation`: [`sl_get_licensing_status_information`] and
//!   [`SL_E_RIGHT_NOT_CONSUMED`]
//! - `sppc.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports: the `__wine_spec_unimplemented_stub` record ([`wine_spec_unimplemented_stub`],
//!   [`StubException`]) and one helper per stub export, for example [`sl_consume_right`]
//!
//! Skipped:
//! - The `FIXME` traces. Each C body prints its arguments there and does nothing else.
//! - The `WINAPI` calling convention and the `debugstr_w` and `wine_dbgstr_guid` formatting.
//! - The arguments of `SLGetLicensingStatusInformation`, which the C body never reads.
//! - The `product` and `flags` arguments of `SLPersistApplicationPolicies`, which the C body
//!   never reads. The `SLID` values are only tested for NULL.

mod exports;
mod licensing;
mod persist;
mod session;
mod stub;
mod stubs;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use licensing::{sl_get_licensing_status_information, SL_E_RIGHT_NOT_CONSUMED};
pub use persist::{sl_persist_application_policies, Slid};
pub use session::{sl_close, sl_open, Hresult, Hslc, E_INVALIDARG, SLC_HANDLE, S_OK};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
