//! Port of the portable logic in Wine's `dlls/connect/main.c` and
//! `dlls/connect/connect.spec`.
//!
//! Ported:
//! - `IsInternetConnected` (`main.c`), which reports success without probing
//!   the network: [`is_internet_connected`]
//! - `DllCanUnloadNow`, exported `-private` in `connect.spec` and supplied by
//!   `libs/winecrt0/dll_canunload.c`: [`dll_can_unload_now`]
//! - `S_OK` and `S_FALSE` from `winerror.h`: [`S_OK`], [`S_FALSE`]
//! - The `stub` exports in `connect.spec` (generated `__wine_spec_unimplemented_stub`
//!   call raises `EXCEPTION_WINE_STUB`): [`STUB_EXPORTS`], [`stub_fault`]
//!
//! Skipped:
//! - The `stub` exports' real behaviour: Wine has none. Only the fault they raise is modelled.
//! - The `FIXME` trace in `IsInternetConnected`.

mod stubs;

pub use stubs::{
    stub_fault, StubFault, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, MODULE_NAME, STUB_EXPORTS,
};

/// `S_OK` from `winerror.h`.
pub const S_OK: i32 = 0;

/// `S_FALSE` from `winerror.h`.
pub const S_FALSE: i32 = 1;

/// `IsInternetConnected`: always `S_OK`.
pub fn is_internet_connected() -> i32 {
    S_OK
}

/// `DllCanUnloadNow`: always `S_FALSE`.
pub fn dll_can_unload_now() -> i32 {
    S_FALSE
}

#[cfg(test)]
mod edge_checks;

#[cfg(test)]
mod oracle_checks;
