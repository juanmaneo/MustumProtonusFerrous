//! Port of the portable logic in Wine's `dlls/dispex/usrmarshal.c`.
//!
//! Ported:
//! - `IDispatchEx_InvokeEx_Proxy` flag derivation: [`proxy_flags`]
//! - By-reference argument split and the copy-block size: [`split_args`], [`copy_block_size`]
//! - `IDispatchEx_InvokeEx_Stub` flag decoding: [`stub_flags`], [`StubFlags`]
//! - Stub argument seeding and cleanup: [`seed_ref_args`], [`clear_ref_slots`]
//! - Callee VT check (`DISP_E_BADCALLEE`): [`callee_result`]
//! - Deferred exception fill-in condition: [`deferred_fill_in_runs`]
//! - Constants `NULL_RESULT`, `NULL_EXCEPINFO`, and the HRESULTs and VTs used above.
//!
//! Skipped:
//! - The RPC transport (`IDispatchEx_RemoteInvokeEx_Proxy`) and the COM dispatch call
//!   (`IDispatchEx_InvokeEx`): they are external to the file.
//! - `CoTaskMemAlloc` / `malloc` themselves: only their success or failure is modelled,
//!   as an input flag.
//! - `DllCanUnloadNow`, `DllGetClassObject`, `DllRegisterServer`, `DllUnregisterServer`
//!   (listed in `dispex.spec`): their C bodies are not in this Wine checkout
//!   (`dlls/dispex` has only `usrmarshal.c`), so no return values are modelled.

mod proxy;
mod stub;

pub use proxy::{copy_block_size, proxy_flags, split_args, ArgSplit};
pub use stub::{
    callee_result, clear_ref_slots, deferred_fill_in_runs, seed_ref_args, stub_flags, StubFlags,
};

/// Set in `dwFlags` when the caller passed no result `VARIANT`.
pub const NULL_RESULT: u32 = 0x0002_0000;
/// Set in `dwFlags` when the caller passed no `EXCEPINFO`.
pub const NULL_EXCEPINFO: u32 = 0x0004_0000;

pub const S_OK: u32 = 0;
pub const E_OUTOFMEMORY: u32 = 0x8007_000E;
pub const DISP_E_EXCEPTION: u32 = 0x8002_0009;
pub const DISP_E_BADCALLEE: u32 = 0x8002_0010;

pub const VT_EMPTY: u16 = 0;
pub const VT_BYREF: u16 = 0x4000;

/// `SUCCEEDED(hr)`: the sign bit of an HRESULT is clear.
pub fn succeeded(hr: u32) -> bool {
    (hr as i32) >= 0
}

/// `V_ISBYREF(v)` on the VT alone.
fn is_byref(vt: u16) -> bool {
    (vt & VT_BYREF) != 0
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
