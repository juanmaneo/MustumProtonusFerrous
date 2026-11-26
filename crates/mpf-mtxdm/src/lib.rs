//! Port of the portable logic in Wine's `dlls/mtxdm/mtxdm_main.c` and `dlls/mtxdm/mtxdm.spec`.
//!
//! Ported:
//! - `GetDispenserManager` (`@ cdecl`): [`get_dispenser_manager`], with the `CoCreateInstance`
//!   arguments it passes: [`CLSID_DISPENSER_MANAGER`], [`IID_IDISPENSER_MANAGER`] and
//!   [`CLSCTX_ALL`], and the [`Guid`] and [`HRESULT`] types
//! - `mtxdm.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - The `TRACE` line, which only logs the `dispenser` pointer.
//! - The COM runtime behind `CoCreateInstance`: class lookup, object creation and the HRESULT
//!   it returns. The port takes that call as a closure and returns the HRESULT it gives back.
//! - The `CDECL` calling convention.

mod dispenser;
mod exports;

pub use dispenser::{
    get_dispenser_manager, Guid, CLSCTX_ALL, CLSID_DISPENSER_MANAGER, HRESULT,
    IID_IDISPENSER_MANAGER,
};
pub use exports::{export_kind, ExportKind, EXPORTS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
